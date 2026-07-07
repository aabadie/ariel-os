//! Peripheral singleton ownership wrapper.

use core::marker::PhantomData;

/// Owned peripheral singleton.
///
/// This mimics the `Peri` type of the Embassy HALs, which the Ariel OS
/// peripheral management macros expect.
pub struct Peri<'a, T> {
    _peripheral: T,
    _lifetime: PhantomData<&'a mut T>,
}

impl<T> Peri<'static, T> {
    pub(crate) const fn new(peripheral: T) -> Self {
        Self {
            _peripheral: peripheral,
            _lifetime: PhantomData,
        }
    }
}
