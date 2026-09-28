//! Support for displays connected to the FMC of STM32 MCUs.
//!
//! The display controller is accessed through an 8080-style 16-bit parallel bus, mapped on one of
//! the NOR/SRAM sub-banks of the FMC.
//! One FMC address line drives the D/CX signal of the display controller: writing to the command
//! address of the bank sends a command, writing to its data address sends parameters or pixels.
//!
//! [`FmcInterface`] implements the [`mipidsi`] display interface on top of such a bus, so that any
//! display controller supported by [`mipidsi`] can be used.
//! Board-specific modules provide ready-to-use initialization for the displays of supported boards.
#![no_std]
#![deny(missing_docs)]

#[cfg(context = "stm32f723e-disco")]
pub mod stm32f723e_disco;

use core::convert::Infallible;

use embassy_stm32::{
    Peri,
    fmc::Fmc,
    gpio::{AfType, Flex, OutputType, Pin, Pull, Speed},
    pac::{
        FMC,
        fmc::vals::{Accmod, Mtyp, Mwid},
    },
    peripherals,
};
use mipidsi::interface::{Interface, InterfaceKind};

pub use mipidsi;

/// NOR/SRAM sub-bank of the FMC, selected by its chip-select signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SramBank {
    /// Sub-bank selected by `FMC_NE1`.
    Ne1,
    /// Sub-bank selected by `FMC_NE2`.
    Ne2,
    /// Sub-bank selected by `FMC_NE3`.
    Ne3,
    /// Sub-bank selected by `FMC_NE4`.
    Ne4,
}

impl SramBank {
    fn index(self) -> usize {
        match self {
            Self::Ne1 => 0,
            Self::Ne2 => 1,
            Self::Ne3 => 2,
            Self::Ne4 => 3,
        }
    }

    /// Returns the base address of the sub-bank in the memory map.
    #[must_use]
    pub fn base_address(self) -> usize {
        0x6000_0000 + self.index() * 0x0400_0000
    }
}

/// Asynchronous access timings of a NOR/SRAM sub-bank, in HCLK cycles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SramTimings {
    /// Address setup phase duration (0 to 15).
    pub address_setup: u8,
    /// Address hold phase duration (1 to 15).
    pub address_hold: u8,
    /// Data phase duration (1 to 255).
    pub data_setup: u8,
    /// Bus turnaround phase duration (0 to 15).
    pub bus_turnaround: u8,
}

/// Enables the FMC and configures `bank` as an asynchronous 16-bit SRAM with write access, using
/// access mode A.
///
/// The FMC pins must be configured separately, see [`fmc_pin()`].
pub fn configure_sram_bank(fmc: Peri<'_, peripherals::FMC>, bank: SramBank, timings: SramTimings) {
    Fmc::new_raw(fmc).enable();

    // BCR1 has a different register type than BCR2 to BCR4, but the fields used here are the same.
    macro_rules! configure_bcr {
        ($reg:expr) => {
            $reg.write(|w| {
                w.set_mtyp(Mtyp::SRAM);
                w.set_mwid(Mwid::BITS16);
                w.set_wren(true);
                w.set_mbken(true);
            })
        };
    }

    FMC.btr(bank.index()).write(|w| {
        w.set_addset(timings.address_setup);
        w.set_addhld(timings.address_hold);
        w.set_datast(timings.data_setup);
        w.set_busturn(timings.bus_turnaround);
        w.set_accmod(Accmod::A);
    });

    match bank {
        SramBank::Ne1 => configure_bcr!(FMC.bcr1()),
        _ => configure_bcr!(FMC.bcr(bank.index() - 1)),
    }
}

/// Configures `pin` in the alternate function `af` used by the FMC.
///
/// The pin stays configured as long as the returned [`Flex`] is alive.
pub fn fmc_pin(pin: Peri<'_, impl Pin>, af: u8) -> Flex<'_> {
    let mut flex = Flex::new(pin);
    flex.set_as_af_unchecked(
        af,
        AfType::output_pull(OutputType::PushPull, Speed::VeryHigh, Pull::Up),
    );
    flex
}

/// [`mipidsi`] display interface for a display controller connected to a NOR/SRAM sub-bank of
/// the FMC through a 16-bit bus.
pub struct FmcInterface {
    command_address: usize,
    data_address: usize,
}

#[expect(
    unsafe_code,
    reason = "the display controller is accessed through memory-mapped I/O"
)]
impl FmcInterface {
    /// Creates a new interface for a display controller mapped on `bank`, whose D/CX signal is
    /// driven by the FMC address line `FMC_A{dcx_address_line}`.
    ///
    /// # Safety
    ///
    /// `bank` must have been configured as a 16-bit SRAM with [`configure_sram_bank()`], with its
    /// pins configured, and must only be accessed through the returned interface.
    #[must_use]
    pub unsafe fn new(bank: SramBank, dcx_address_line: u8) -> Self {
        let command_address = bank.base_address();
        // The FMC addresses 16-bit words on a 16-bit bus: `FMC_A[n]` is driven by HADDR[n + 1].
        let data_address = command_address | (1 << (dcx_address_line + 1));

        Self {
            command_address,
            data_address,
        }
    }

    fn write(address: usize, value: u16) {
        // SAFETY: `address` belongs to a configured FMC bank, as guaranteed by the caller of
        // `new()`.
        unsafe { core::ptr::write_volatile(address as *mut u16, value) };
        // The FMC region is mapped as Normal memory by default, where consecutive writes to the
        // same address may be merged: wait for each write to complete.
        cortex_m::asm::dsb();
    }

    fn write_command(&mut self, command: u16) {
        Self::write(self.command_address, command);
    }

    fn write_data(&mut self, data: u16) {
        Self::write(self.data_address, data);
    }
}

impl Interface for FmcInterface {
    type Word = u16;
    type Error = Infallible;

    const KIND: InterfaceKind = InterfaceKind::Parallel16Bit;

    fn send_command(&mut self, command: u8, args: &[u8]) -> Result<(), Self::Error> {
        self.write_command(u16::from(command));
        for arg in args {
            self.write_data(u16::from(*arg));
        }
        Ok(())
    }

    fn send_pixels<const N: usize>(
        &mut self,
        pixels: impl IntoIterator<Item = [Self::Word; N]>,
    ) -> Result<(), Self::Error> {
        for pixel in pixels {
            for word in pixel {
                self.write_data(word);
            }
        }
        Ok(())
    }

    fn send_repeated_pixel<const N: usize>(
        &mut self,
        pixel: [Self::Word; N],
        count: u32,
    ) -> Result<(), Self::Error> {
        for _ in 0..count {
            for word in pixel {
                self.write_data(word);
            }
        }
        Ok(())
    }
}
