//! Low-level FFI bindings to the SCuM SDK C code.
//!
//! The vendored SDK code (see the vendor directory) provides the chip
//! initialization and the optical clock calibration, which must run at
//! every boot. Only these entry points are exposed; everything else is
//! internal to the C code.
//!
//! The C code is only compiled when building for the `scum` laze context;
//! on other targets this crate is empty.

#![no_std]
#![deny(missing_docs)]
#![cfg_attr(
    context = "scum",
    expect(unsafe_code, reason = "FFI bindings to the SCuM SDK C code")
)]

#[cfg(context = "scum")]
unsafe extern "C" {
    /// Initializes the mote: analog scan chain, LDOs, GPIO banks and clock
    /// sources.
    ///
    /// Must be called once at startup, before `perform_calibration`.
    pub fn initialize_mote();

    /// Runs the optical clock calibration.
    ///
    /// Blocks (on `wfi`) until the calibration completes. The calibration
    /// is driven by the optical programmer LED sequence, triggered by
    /// flashing with `scum-programmer --calibrate`; without it this
    /// function never returns.
    ///
    /// The `OPTICAL_SFD` and `EXT_GPIO8_ACTIVEHIGH` interrupts must be
    /// wired to `OPTICAL_SFD_Handler` and NVIC interrupts must be enabled.
    pub fn perform_calibration();

    /// The optical calibration interrupt handler.
    ///
    /// Must be installed for both the `OPTICAL_SFD` (IRQ 11) and
    /// `EXT_GPIO8_ACTIVEHIGH` (IRQ 12) interrupt vectors.
    pub fn OPTICAL_SFD_Handler();
}
