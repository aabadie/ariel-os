//! Provides UART access.
//!
//! The SCuM UART is accessed through a single data register; the baud rate
//! (19200) is fixed by the chip clocking and is not configurable. Only
//! blocking transmission is currently implemented; it is used as the
//! logging output.

/// UART data register (see the SDK's scum.h).
const UART_DATA: *mut u32 = 0x5100_0000 as *mut u32;

/// Blocking UART driver.
pub struct Uart {
    _uart0: crate::peripherals::UART0,
}

impl Uart {
    /// Creates a UART driver from the UART peripheral singleton.
    ///
    /// No configuration is needed: the baud rate is fixed by the chip
    /// clocking.
    #[must_use]
    pub fn new(uart0: crate::peripherals::UART0) -> Self {
        Self { _uart0: uart0 }
    }

    /// Writes all the bytes of the buffer (blocking).
    pub fn write_bytes(&mut self, buffer: &[u8]) {
        for byte in buffer {
            #[expect(unsafe_code, reason = "hardware register access")]
            // SAFETY: write to the transmit data register, whose only
            // effect is transmitting the byte; exclusive access to the
            // register is ensured by taking `&mut self`.
            unsafe {
                UART_DATA.write_volatile(u32::from(*byte));
            }
        }
    }
}
