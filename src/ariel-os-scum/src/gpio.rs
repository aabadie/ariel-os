//! Provides GPIO access.
//!
//! The functions in this module are stubs until the GPIO driver is
//! implemented (milestone 4 of the port): the SCuM GPIO banks need to be
//! configured through the analog scan chain, which is only available once
//! the chip has been initialized and calibrated.

macro_rules! define_input_like {
    ($type:ident) => {
        /// Input-like GPIO.
        pub struct $type<'d> {
            _marker: core::marker::PhantomData<&'d ()>,
        }

        impl $type<'_> {
            #[doc(hidden)]
            #[must_use]
            pub fn is_high(&self) -> bool {
                unimplemented!();
            }

            #[doc(hidden)]
            #[must_use]
            pub fn is_low(&self) -> bool {
                unimplemented!();
            }

            #[doc(hidden)]
            #[must_use]
            pub fn get_level(&self) -> crate::gpio::input::Level {
                unimplemented!();
            }

            #[doc(hidden)]
            pub async fn wait_for_high(&mut self) {
                unimplemented!();
            }

            #[doc(hidden)]
            pub async fn wait_for_low(&mut self) {
                unimplemented!();
            }

            #[doc(hidden)]
            pub async fn wait_for_rising_edge(&mut self) {
                unimplemented!();
            }

            #[doc(hidden)]
            pub async fn wait_for_falling_edge(&mut self) {
                unimplemented!();
            }

            #[doc(hidden)]
            pub async fn wait_for_any_edge(&mut self) {
                unimplemented!();
            }
        }

        impl embedded_hal::digital::ErrorType for $type<'_> {
            type Error = core::convert::Infallible;
        }

        impl embedded_hal::digital::InputPin for $type<'_> {
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                unimplemented!();
            }

            fn is_high(&mut self) -> Result<bool, Self::Error> {
                unimplemented!();
            }
        }

        impl embedded_hal_async::digital::Wait for $type<'_> {
            async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
                unimplemented!();
            }

            async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
                unimplemented!();
            }

            async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
                unimplemented!();
            }

            async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
                unimplemented!();
            }

            async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
                unimplemented!();
            }
        }
    };
}

pub mod input {
    //! Input-specific types.

    use crate::IntoPeripheral;

    /// Whether inputs support configuring whether a Schmitt trigger is enabled.
    pub const SCHMITT_TRIGGER_CONFIGURABLE: bool = false;

    /// Marker trait for GPIO peripherals usable as inputs.
    pub trait InputPin {}

    #[doc(hidden)]
    pub fn new<'a, T: InputPin>(
        pin: impl IntoPeripheral<'a, T>,
        _pull: ariel_os_embassy_common::gpio::Pull,
        _schmitt_trigger: bool, // Not supported by hardware
    ) -> Result<Input<'a>, ariel_os_embassy_common::gpio::input::Error> {
        let _ = pin.into_hal_peripheral();
        unimplemented!();
    }

    define_input_like!(Input);

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

    use crate::IntoPeripheral;

    /// Whether outputs support configuring their drive strength.
    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = false;
    /// Whether outputs support configuring their speed/slew rate.
    pub const SPEED_CONFIGURABLE: bool = false;

    /// Marker trait for GPIO peripherals usable as outputs.
    pub trait OutputPin {}

    #[doc(hidden)]
    pub fn new<'a, T: OutputPin>(
        pin: impl IntoPeripheral<'a, T>,
        _initial_level: ariel_os_embassy_common::gpio::Level,
        _drive_strength: super::DriveStrength,
        _speed: super::Speed, // Not supported by hardware
    ) -> Output<'a> {
        let _ = pin.into_hal_peripheral();
        unimplemented!();
    }

    /// Output GPIO.
    pub struct Output<'d> {
        _marker: core::marker::PhantomData<&'d ()>,
    }

    impl embedded_hal::digital::ErrorType for Output<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::OutputPin for Output<'_> {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            unimplemented!();
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            unimplemented!();
        }
    }

    impl embedded_hal::digital::StatefulOutputPin for Output<'_> {
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            unimplemented!();
        }

        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            unimplemented!();
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
    ($($gpio:ident),* $(,)?) => {
        $(
            impl input::InputPin for crate::peripherals::$gpio {}
            impl output::OutputPin for crate::peripherals::$gpio {}
        )*
    };
}

impl_pin_traits!(
    GPIO0, GPIO1, GPIO2, GPIO3, GPIO4, GPIO5, GPIO6, GPIO7, GPIO8, GPIO9, GPIO10, GPIO11, GPIO12,
    GPIO13, GPIO14, GPIO15,
);
