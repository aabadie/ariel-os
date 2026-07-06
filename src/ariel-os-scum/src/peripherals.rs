//! Types for the peripheral singletons.

macro_rules! define_peripherals {
    ($($(#[$doc:meta])* $name:ident),* $(,)?) => {
        $(
            $(#[$doc])*
            pub struct $name {
                _private: (),
            }
        )*

        /// Contains all the peripheral singletons, each wrapped in an [`Option`].
        #[allow(non_snake_case, reason = "peripheral singletons are conventionally named after the hardware")]
        #[derive(Default)]
        pub struct OptionalPeripherals {
            $(
                $(#[$doc])*
                pub $name: Option<$name>,
            )*
        }

        impl OptionalPeripherals {
            /// Creates an `OptionalPeripherals`, populating all the singletons.
            #[must_use]
            pub fn new() -> Self {
                Self {
                    $($name: Some($name { _private: () }),)*
                }
            }
        }
    }
}

define_peripherals!(
    /// GPIO 0.
    GPIO0,
    /// GPIO 1.
    GPIO1,
    /// GPIO 2.
    GPIO2,
    /// GPIO 3.
    GPIO3,
    /// GPIO 4.
    GPIO4,
    /// GPIO 5.
    GPIO5,
    /// GPIO 6.
    GPIO6,
    /// GPIO 7.
    GPIO7,
    /// GPIO 8. Used by the optical calibration (3-wire-bus calibration clock).
    GPIO8,
    /// GPIO 9.
    GPIO9,
    /// GPIO 10.
    GPIO10,
    /// GPIO 11.
    GPIO11,
    /// GPIO 12.
    GPIO12,
    /// GPIO 13.
    GPIO13,
    /// GPIO 14.
    GPIO14,
    /// GPIO 15.
    GPIO15,
    /// UART peripheral.
    UART0,
    /// RF timer peripheral.
    RFTIMER,
);
