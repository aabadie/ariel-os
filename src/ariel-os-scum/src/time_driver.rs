//! embassy-time driver based on the SCuM RF timer.
//!
//! The RF timer is a 32-bit up-counter with 8 compare channels, running at
//! 500 kHz once the chip clocks are calibrated. The embassy tick rate is
//! the standard 1 MHz (embassy-time-driver has no 500 kHz tick feature),
//! so hardware counter values are scaled by exactly 2.
//!
//! Compare channel 0 is the alarm channel. The 32-bit counter is extended
//! to a 64-bit count by detecting counter wraps in `now_raw`; this requires
//! `now_raw` to be called at least once per counter wrap period (about
//! 2.4 hours), which is guaranteed by compare channel 1 firing at every
//! wrap, and by capping how far ahead the alarm channel is armed.

#![expect(unsafe_code, reason = "hardware register access")]

use core::cell::RefCell;
use core::task::Waker;

use critical_section::Mutex;
use embassy_time_queue_utils::Queue;

use crate::irqs::Interrupt;

// RF timer register block (see the SDK's scum.h).
const RFTIMER_BASE: usize = 0x4200_0000;
const CONTROL: *mut u32 = RFTIMER_BASE as *mut u32;
const COUNTER: *const u32 = (RFTIMER_BASE + 0x04) as *const u32;
const MAX_COUNT: *mut u32 = (RFTIMER_BASE + 0x08) as *mut u32;
const INT: *const u32 = (RFTIMER_BASE + 0x70) as *const u32;
const INT_CLEAR: *mut u32 = (RFTIMER_BASE + 0x74) as *mut u32;

const fn compare(channel: usize) -> *mut u32 {
    (RFTIMER_BASE + 0x10 + 4 * channel) as *mut u32
}

const fn compare_control(channel: usize) -> *mut u32 {
    (RFTIMER_BASE + 0x30 + 4 * channel) as *mut u32
}

const CONTROL_ENABLE: u32 = 0x01;
const CONTROL_INTERRUPT_ENABLE: u32 = 0x02;
const CONTROL_COUNT_RESET: u32 = 0x04;
const COMPARE_ENABLE: u32 = 0x01;
const COMPARE_INTERRUPT_ENABLE: u32 = 0x02;

const ALARM_CHANNEL: usize = 0;
const WRAP_CHANNEL: usize = 1;

/// Never arm a compare closer than this to the current counter value, the
/// hardware may miss it (`MINIMUM_COMPAREVALE_ADVANCE` in the SDK).
const MINIMUM_COMPARE_ADVANCE: u32 = 5;

/// Cap on how far ahead the alarm channel is armed (in hardware ticks).
///
/// This ensures the interrupt handler calls `now_raw` well within a counter
/// wrap period. Armed checkpoints simply re-arm from the interrupt handler.
const MAX_ARM_DELTA: u32 = 0x4000_0000;

/// Embassy ticks (1 MHz) per hardware tick (500 kHz).
const TICKS_PER_COUNT: u64 = 2;

struct Inner {
    /// Counter value at the previous `now_raw` call, for wrap detection.
    last_counter: u32,
    /// Number of counter wraps since boot.
    wraps: u32,
    queue: Queue,
}

impl Inner {
    const fn new() -> Self {
        Self {
            last_counter: 0,
            wraps: 0,
            queue: Queue::new(),
        }
    }

    /// Returns the wrap-extended hardware counter value.
    fn now_raw(&mut self) -> u64 {
        // SAFETY: reading the counter register has no side effect.
        let counter = unsafe { COUNTER.read_volatile() };
        if counter < self.last_counter {
            self.wraps += 1;
        }
        self.last_counter = counter;
        (u64::from(self.wraps) << 32) | u64::from(counter)
    }

    /// Returns the current time in embassy ticks.
    fn now(&mut self) -> u64 {
        self.now_raw() * TICKS_PER_COUNT
    }

    /// Arms the alarm channel for embassy tick `at`.
    ///
    /// Returns false if `at` is already due, in which case the caller must
    /// process the timer queue again.
    fn arm(&mut self, at: u64) -> bool {
        if at == u64::MAX {
            // Nothing scheduled: disable the alarm channel.
            // SAFETY: disables the compare channel, no other side effect.
            unsafe { compare_control(ALARM_CHANNEL).write_volatile(0) };
            return true;
        }

        let now_raw = self.now_raw();
        let at_raw = at.div_ceil(TICKS_PER_COUNT);
        if at_raw <= now_raw {
            return false;
        }

        let delta = u32::try_from(at_raw - now_raw)
            .unwrap_or(u32::MAX)
            .clamp(MINIMUM_COMPARE_ADVANCE, MAX_ARM_DELTA);
        let target = self.last_counter.wrapping_add(delta);
        // SAFETY: clears a pending alarm interrupt and programs the alarm
        // compare channel; the registers are only accessed with interrupts
        // masked (critical section or interrupt handler).
        unsafe {
            INT_CLEAR.write_volatile(1 << ALARM_CHANNEL);
            compare(ALARM_CHANNEL).write_volatile(target);
            compare_control(ALARM_CHANNEL)
                .write_volatile(COMPARE_ENABLE | COMPARE_INTERRUPT_ENABLE);
        }

        // If the counter raced past the target while arming, the compare
        // may have been missed: force the interrupt to process the queue.
        if self.now_raw() >= now_raw + u64::from(delta) {
            cortex_m::peripheral::NVIC::pend(Interrupt::RFTIMER);
        }

        true
    }

    /// Processes the timer queue and re-arms the alarm channel.
    fn process_queue(&mut self) {
        let now = self.now();
        let mut next = self.queue.next_expiration(now);
        while !self.arm(next) {
            let now = self.now();
            next = self.queue.next_expiration(now);
        }
    }
}

struct RfTimerDriver {
    inner: Mutex<RefCell<Inner>>,
}

impl RfTimerDriver {
    fn on_interrupt(&self) {
        critical_section::with(|cs| {
            // SAFETY: acknowledges the interrupt flags that fired; the
            // enabled channels are all owned by this driver.
            unsafe {
                let flags = INT.read_volatile();
                INT_CLEAR.write_volatile(flags);
            }
            self.inner.borrow_ref_mut(cs).process_queue();
        });
    }
}

impl embassy_time_driver::Driver for RfTimerDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| self.inner.borrow_ref_mut(cs).now())
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut inner = self.inner.borrow_ref_mut(cs);
            if inner.queue.schedule_wake(at, waker) {
                inner.process_queue();
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: RfTimerDriver = RfTimerDriver {
    inner: Mutex::new(RefCell::new(Inner::new()))
});

/// Initializes the RF timer.
///
/// Must only be called once the chip clocks are calibrated, as the RF timer
/// frequency depends on them.
pub(crate) fn init() {
    // SAFETY: one-time configuration of the RF timer: free-running 32-bit
    // counter, wrap detection channel armed, alarm channel disabled.
    unsafe {
        MAX_COUNT.write_volatile(u32::MAX);
        compare(WRAP_CHANNEL).write_volatile(0);
        compare_control(WRAP_CHANNEL).write_volatile(COMPARE_ENABLE | COMPARE_INTERRUPT_ENABLE);
        compare_control(ALARM_CHANNEL).write_volatile(0);
        INT_CLEAR.write_volatile(0xFF);
        CONTROL.write_volatile(CONTROL_ENABLE | CONTROL_INTERRUPT_ENABLE | CONTROL_COUNT_RESET);
    }

    // SAFETY: the RFTIMER handler (below) is set up in the vector table and
    // only drives this time driver.
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::RFTIMER) };
}

#[cfg(context = "scum")]
#[unsafe(no_mangle)]
extern "C" fn RFTIMER() {
    DRIVER.on_interrupt();
}
