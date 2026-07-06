//! Items specific to the SCuM (Single Chip micro Mote).
//!
//! This is an out-of-tree demo port; only basic features are supported.

#![no_std]
#![deny(missing_docs)]

pub mod gpio;

pub mod peripherals;

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
    // Milestone 2 will initialize and calibrate the chip here, before any
    // driver is set up, by calling into the SCuM SDK C code:
    // initialize_mote() then perform_calibration().
    OptionalPeripherals::new()
}
