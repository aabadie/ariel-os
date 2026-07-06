//! SCuM interrupt numbers.
//!
//! The interrupt layout comes from the SDK's scum.h and matches the vector
//! table in vectors.rs.

/// SCuM interrupt lines.
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// UART interrupt.
    UART = 0,
    /// GPIO3 interrupt.
    EXT_GPIO3_ACTIVEHIGH_DEBOUNCED = 1,
    /// Optical receiver interrupt.
    EXT_OPTICAL_IRQ_IN = 2,
    /// ADC interrupt.
    ADC = 3,
    /// RF interrupt.
    RF = 6,
    /// RF timer interrupt.
    RFTIMER = 7,
    /// RAWCHIPS start value interrupt.
    RAWCHIPS_STARTVAL = 8,
    /// RAWCHIPS 32-bit interrupt.
    RAWCHIPS_32 = 9,
    /// Optical SFD interrupt.
    OPTICAL_SFD = 11,
    /// GPIO8 interrupt (carries the optical calibration clock).
    EXT_GPIO8_ACTIVEHIGH = 12,
    /// GPIO9 interrupt.
    EXT_GPIO9_ACTIVELOW = 13,
    /// GPIO10 interrupt.
    EXT_GPIO10_ACTIVELOW = 14,
}

#[cfg(feature = "time")]
#[expect(unsafe_code, reason = "InterruptNumber is an unsafe trait")]
// SAFETY: the numbers match the SCuM vector table (see scum.h in the SDK
// and vectors.rs), and the highest interrupt number is 14.
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    fn number(self) -> u16 {
        self as u16
    }
}
