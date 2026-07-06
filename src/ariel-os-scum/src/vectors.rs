//! SCuM interrupt vector table.
//!
//! There is no PAC for SCuM, so the device part of the cortex-m-rt vector
//! table is hand-written here (the interrupt layout comes from the SDK's
//! scum.h). Handlers default to `DefaultHandler` through the PROVIDE
//! entries in device.x, except for the two optical calibration vectors
//! which are wired to the SDK C handler: the calibration clock is received
//! on GPIO8, so both `OPTICAL_SFD` and `EXT_GPIO8_ACTIVEHIGH` must call
//! `OPTICAL_SFD_Handler`.

#![expect(unsafe_code, reason = "hand-written interrupt vector table")]

use scum_sdk_sys::OPTICAL_SFD_Handler;

#[repr(C)]
union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

unsafe extern "C" {
    fn UART();
    fn EXT_GPIO3_ACTIVEHIGH_DEBOUNCED();
    fn EXT_OPTICAL_IRQ_IN();
    fn ADC();
    fn RF();
    fn RFTIMER();
    fn RAWCHIPS_STARTVAL();
    fn RAWCHIPS_32();
    fn EXT_GPIO9_ACTIVELOW();
    fn EXT_GPIO10_ACTIVELOW();
}

#[unsafe(no_mangle)]
extern "C" fn OPTICAL_SFD() {
    // SAFETY: interrupt handler of the vendored SDK code, only called by
    // the corresponding interrupt.
    unsafe { OPTICAL_SFD_Handler() }
}

#[unsafe(no_mangle)]
extern "C" fn EXT_GPIO8_ACTIVEHIGH() {
    // SAFETY: interrupt handler of the vendored SDK code; GPIO8 carries the
    // optical calibration clock, so this aliases to the same handler.
    unsafe { OPTICAL_SFD_Handler() }
}

#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
static __INTERRUPTS: [Vector; 15] = [
    Vector { handler: UART }, // 0
    Vector {
        handler: EXT_GPIO3_ACTIVEHIGH_DEBOUNCED,
    }, // 1
    Vector {
        handler: EXT_OPTICAL_IRQ_IN,
    }, // 2
    Vector { handler: ADC },  // 3
    Vector { reserved: 0 },   // 4
    Vector { reserved: 0 },   // 5
    Vector { handler: RF },   // 6
    Vector { handler: RFTIMER }, // 7
    Vector {
        handler: RAWCHIPS_STARTVAL,
    }, // 8
    Vector {
        handler: RAWCHIPS_32,
    }, // 9
    Vector { reserved: 0 },   // 10
    Vector {
        handler: OPTICAL_SFD,
    }, // 11
    Vector {
        handler: EXT_GPIO8_ACTIVEHIGH,
    }, // 12
    Vector {
        handler: EXT_GPIO9_ACTIVELOW,
    }, // 13
    Vector {
        handler: EXT_GPIO10_ACTIVELOW,
    }, // 14
];
