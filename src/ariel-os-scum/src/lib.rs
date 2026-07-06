//! Items specific to the SCuM (Single Chip micro Mote).
//!
//! This is an out-of-tree demo port; only basic features are supported.

#![no_std]
#![deny(missing_docs)]

pub mod gpio;

pub mod irqs;

pub mod peripherals;

#[cfg(feature = "time")]
mod time_driver;

pub mod uart;

#[cfg(context = "scum")]
mod vectors;

#[doc(hidden)]
pub mod identity {
    use ariel_os_embassy_common::identity;

    pub type DeviceId = identity::NoDeviceId<identity::NotImplemented>;
}

#[doc(hidden)]
pub mod peripheral {}

#[doc(hidden)]
pub use peripherals::OptionalPeripherals;

#[doc(hidden)]
pub trait IntoPeripheral<'a, T> {
    fn into_hal_peripheral(self) -> T;
}

#[doc(hidden)]
impl<T> IntoPeripheral<'_, T> for T {
    fn into_hal_peripheral(self) -> T {
        self
    }
}

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    // The chip must be initialized and calibrated before any driver is set
    // up: all SCuM clocks are on-chip oscillators that the optical
    // calibration tunes at every boot. The calibration is driven by the
    // programmer LED sequence (scum-programmer --calibrate) and blocks
    // until complete.
    #[cfg(context = "scum")]
    {
        #![expect(unsafe_code, reason = "one-time chip initialization FFI")]
        // SAFETY: called once at startup, before any driver is set up; the
        // vector table (src/vectors.rs) wires the optical calibration
        // interrupts to the SDK handler, as these functions require.
        unsafe {
            scum_sdk_sys::initialize_mote();
            scum_sdk_sys::perform_calibration();
        }
    }

    // The RF timer frequency depends on the calibrated clocks, so the time
    // driver is only initialized once the calibration completed.
    #[cfg(feature = "time")]
    time_driver::init();

    OptionalPeripherals::new()
}
