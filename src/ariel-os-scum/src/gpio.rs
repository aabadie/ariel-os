//! Provides GPIO access.
//!
//! The 16 SCuM GPIOs are driven and sampled through a single data register
//! (following the SDK's gpio.c and the RIOT port). The pad bank routing and
//! the input/output enables are configured through the analog scan chain by
//! `initialize_mote()` at startup: all pads are output-enabled, and only
//! GPI8 (used by the optical calibration) is input-enabled.
//!
//! Interrupt-driven edge detection is not supported: only GPIO3 and
//! GPIO8-10 have (fixed) interrupt lines, which are not currently wired to
//! this driver. The `wait_for_*` methods of [`input::Input`] are therefore
//! unimplemented.

/// GPIO data register (see the SDK's scum.h): written to drive the
/// output-enabled pads, read to sample the pads.
const GPIO_DATA: *mut u32 = 0x5300_0000 as *mut u32;

fn read_data() -> u32 {
    #[expect(unsafe_code, reason = "hardware register access")]
    // SAFETY: reading the GPIO data register has no side effect.
    unsafe {
        GPIO_DATA.read_volatile()
    }
}

fn modify_data(f: impl FnOnce(u32) -> u32) {
    // The register has no atomic set/clear companions, so the
    // read-modify-write is done inside a critical section.
    critical_section::with(|_| {
        #[expect(unsafe_code, reason = "hardware register access")]
        // SAFETY: read-modify-write of the GPIO data register, made
        // exclusive by the critical section (the vendored SDK C code does
        // not touch this register concurrently).
        unsafe {
            GPIO_DATA.write_volatile(f(GPIO_DATA.read_volatile()));
        }
    });
}

/// Marker trait for GPIO pin peripherals.
///
/// This trait is sealed and implemented by the GPIO peripheral singletons.
pub trait Pin: private::Sealed {
    #[doc(hidden)]
    const NUMBER: u8;
}

mod private {
    pub trait Sealed {}
}

pub mod input {
    //! Input-specific types.

    use crate::peripheral::Peri;

    /// Whether inputs support configuring whether a Schmitt trigger is enabled.
    pub const SCHMITT_TRIGGER_CONFIGURABLE: bool = false;

    /// Marker trait for GPIO peripherals usable as inputs.
    pub trait InputPin: super::Pin {}

    #[doc(hidden)]
    pub fn new<P: InputPin>(
        _pin: Peri<'_, P>,
        _pull: ariel_os_embassy_common::gpio::Pull, // Pulls are not supported by the hardware
        _schmitt_trigger: bool,                     // Not supported by the hardware
    ) -> Result<Input<'_>, ariel_os_embassy_common::gpio::input::Error> {
        Ok(Input {
            mask: 1 << P::NUMBER,
            _lifetime: core::marker::PhantomData,
        })
    }

    /// Input GPIO.
    ///
    /// The pin must be input-enabled through the analog scan chain (`GPI`
    /// controls), which is not done by this driver.
    pub struct Input<'d> {
        mask: u32,
        _lifetime: core::marker::PhantomData<&'d ()>,
    }

    impl Input<'_> {
        #[doc(hidden)]
        #[must_use]
        pub fn is_high(&self) -> bool {
            super::read_data() & self.mask != 0
        }

        #[doc(hidden)]
        #[must_use]
        pub fn is_low(&self) -> bool {
            !self.is_high()
        }

        #[doc(hidden)]
        #[must_use]
        pub fn get_level(&self) -> Level {
            if self.is_high() {
                Level::High
            } else {
                Level::Low
            }
        }

        #[doc(hidden)]
        pub async fn wait_for_high(&mut self) {
            unimplemented!("GPIO edge interrupts are not supported");
        }

        #[doc(hidden)]
        pub async fn wait_for_low(&mut self) {
            unimplemented!("GPIO edge interrupts are not supported");
        }

        #[doc(hidden)]
        pub async fn wait_for_rising_edge(&mut self) {
            unimplemented!("GPIO edge interrupts are not supported");
        }

        #[doc(hidden)]
        pub async fn wait_for_falling_edge(&mut self) {
            unimplemented!("GPIO edge interrupts are not supported");
        }

        #[doc(hidden)]
        pub async fn wait_for_any_edge(&mut self) {
            unimplemented!("GPIO edge interrupts are not supported");
        }
    }

    impl embedded_hal::digital::ErrorType for Input<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::InputPin for Input<'_> {
        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(Input::is_low(self))
        }

        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(Input::is_high(self))
        }
    }

    impl embedded_hal_async::digital::Wait for Input<'_> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            Input::wait_for_high(self).await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            Input::wait_for_low(self).await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            Input::wait_for_rising_edge(self).await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            Input::wait_for_falling_edge(self).await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            Input::wait_for_any_edge(self).await;
            Ok(())
        }
    }

    /// Digital input level.
    pub enum Level {
        /// Low level.
        Low,
        /// High level.
        High,
    }

    ariel_os_embassy_common::define_into_level!();
}

pub mod output {
    //! Output-specific types.

    use crate::peripheral::Peri;

    /// Whether outputs support configuring their drive strength.
    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = false;
    /// Whether outputs support configuring their speed/slew rate.
    pub const SPEED_CONFIGURABLE: bool = false;

    /// Marker trait for GPIO peripherals usable as outputs.
    pub trait OutputPin: super::Pin {}

    #[doc(hidden)]
    pub fn new<P: OutputPin>(
        _pin: Peri<'_, P>,
        initial_level: ariel_os_embassy_common::gpio::Level,
        _drive_strength: super::DriveStrength, // Not supported by the hardware
        _speed: super::Speed,                  // Not supported by the hardware
    ) -> Output<'_> {
        let mut output = Output {
            mask: 1 << P::NUMBER,
            _lifetime: core::marker::PhantomData,
        };
        match initial_level {
            ariel_os_embassy_common::gpio::Level::Low => output.set_low(),
            ariel_os_embassy_common::gpio::Level::High => output.set_high(),
        }
        output
    }

    /// Output GPIO.
    pub struct Output<'d> {
        mask: u32,
        _lifetime: core::marker::PhantomData<&'d ()>,
    }

    impl Output<'_> {
        fn set_high(&mut self) {
            super::modify_data(|data| data | self.mask);
        }

        fn set_low(&mut self) {
            super::modify_data(|data| data & !self.mask);
        }
    }

    impl embedded_hal::digital::ErrorType for Output<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::OutputPin for Output<'_> {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Output::set_low(self);
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Output::set_high(self);
            Ok(())
        }
    }

    impl embedded_hal::digital::StatefulOutputPin for Output<'_> {
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            Ok(super::read_data() & self.mask != 0)
        }

        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(super::read_data() & self.mask == 0)
        }
    }
}

/// Available drive strengths (not configurable on this hardware).
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DriveStrength {
    /// Default drive strength.
    Default,
}

impl ariel_os_embassy_common::gpio::FromDriveStrength for DriveStrength {
    fn from(_drive_strength: ariel_os_embassy_common::gpio::DriveStrength<Self>) -> Self {
        Self::Default
    }
}

/// Available output speeds (not configurable on this hardware).
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Speed {
    /// Default speed.
    Default,
}

impl ariel_os_embassy_common::gpio::FromSpeed for Speed {
    fn from(_speed: ariel_os_embassy_common::gpio::Speed<Self>) -> Self {
        Self::Default
    }
}

macro_rules! impl_pin_traits {
    ($($gpio:ident => $number:literal),* $(,)?) => {
        $(
            impl private::Sealed for crate::peripherals::$gpio {}

            impl Pin for crate::peripherals::$gpio {
                const NUMBER: u8 = $number;
            }

            impl input::InputPin for crate::peripherals::$gpio {}
            impl output::OutputPin for crate::peripherals::$gpio {}
        )*
    };
}

impl_pin_traits!(
    GPIO0 => 0,
    GPIO1 => 1,
    GPIO2 => 2,
    GPIO3 => 3,
    GPIO4 => 4,
    GPIO5 => 5,
    GPIO6 => 6,
    GPIO7 => 7,
    GPIO8 => 8,
    GPIO9 => 9,
    GPIO10 => 10,
    GPIO11 => 11,
    GPIO12 => 12,
    GPIO13 => 13,
    GPIO14 => 14,
    GPIO15 => 15,
);
