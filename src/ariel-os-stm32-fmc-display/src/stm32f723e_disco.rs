//! Display of the STM32F723E-DISCO board.
//!
//! The board has a 240x240 pixels LCD driven by a Sitronix ST7789H2 controller, connected to the
//! FMC sub-bank selected by `FMC_NE2`, with its D/CX signal driven by `FMC_A0`.

use core::convert::Infallible;

use embassy_stm32::{
    Peri,
    fmc::{
        A0Pin, D0Pin, D1Pin, D2Pin, D3Pin, D4Pin, D5Pin, D6Pin, D7Pin, D8Pin, D9Pin, D10Pin,
        D11Pin, D12Pin, D13Pin, D14Pin, D15Pin, NE2Pin, NOEPin, NWEPin,
    },
    gpio::{Flex, Level, Output, Speed},
    peripherals,
};
use embedded_hal::delay::DelayNs;
use mipidsi::{Builder, models::ST7789, options::ColorInversion};

use crate::{FmcInterface, SramBank, SramTimings, configure_sram_bank, fmc_pin};

/// Width of the display, in pixels.
pub const WIDTH: u16 = 240;
/// Height of the display, in pixels.
pub const HEIGHT: u16 = 240;

const BANK: SramBank = SramBank::Ne2;
const DCX_ADDRESS_LINE: u8 = 0;

// The ST7789H2 requires a write cycle of at least 66 ns; with HCLK at 216 MHz, this gives a write
// cycle of (9 + 6 + 1) cycles * 4.63 ns = 74 ns.
const TIMINGS: SramTimings = SramTimings {
    address_setup: 9,
    address_hold: 2,
    data_setup: 6,
    bus_turnaround: 1,
};

/// Peripherals used by the display.
#[expect(missing_docs, reason = "fields are named after the board signals")]
pub struct Peripherals<'d> {
    pub fmc: Peri<'d, peripherals::FMC>,
    pub ne2: Peri<'d, peripherals::PG9>,
    pub noe: Peri<'d, peripherals::PD4>,
    pub nwe: Peri<'d, peripherals::PD5>,
    pub a0: Peri<'d, peripherals::PF0>,
    pub d0: Peri<'d, peripherals::PD14>,
    pub d1: Peri<'d, peripherals::PD15>,
    pub d2: Peri<'d, peripherals::PD0>,
    pub d3: Peri<'d, peripherals::PD1>,
    pub d4: Peri<'d, peripherals::PE7>,
    pub d5: Peri<'d, peripherals::PE8>,
    pub d6: Peri<'d, peripherals::PE9>,
    pub d7: Peri<'d, peripherals::PE10>,
    pub d8: Peri<'d, peripherals::PE11>,
    pub d9: Peri<'d, peripherals::PE12>,
    pub d10: Peri<'d, peripherals::PE13>,
    pub d11: Peri<'d, peripherals::PE14>,
    pub d12: Peri<'d, peripherals::PE15>,
    pub d13: Peri<'d, peripherals::PD8>,
    pub d14: Peri<'d, peripherals::PD9>,
    pub d15: Peri<'d, peripherals::PD10>,
    pub reset: Peri<'d, peripherals::PH7>,
    pub backlight: Peri<'d, peripherals::PH11>,
}

/// The [`mipidsi`] display, which implements `embedded_graphics_core::draw_target::DrawTarget`.
pub type Display<'d> = mipidsi::Display<FmcInterface, ST7789, Output<'d>>;

/// Error returned when the display fails to initialize.
pub type InitError = mipidsi::InitError<Infallible, Infallible>;

/// Display of the board, with its backlight.
pub struct Lcd<'d> {
    display: Display<'d>,
    backlight: Output<'d>,
    // Keeps the FMC pins configured.
    _fmc_pins: [Flex<'d>; 20],
}

impl<'d> Lcd<'d> {
    /// Configures the FMC, initializes the display controller, and turns the backlight on.
    ///
    /// # Errors
    ///
    /// Returns an error if the display controller fails to initialize.
    pub fn new(p: Peripherals<'d>, delay: &mut impl DelayNs) -> Result<Self, InitError> {
        // Each pin gets the alternate function of its FMC signal, checked at compile time.
        macro_rules! pins {
            ($($field:ident: $signal:ident),* $(,)?) => {
                [$({
                    let af = $signal::af_num(&*p.$field);
                    fmc_pin(p.$field, af)
                }),*]
            };
        }

        let fmc_pins = pins!(
            ne2: NE2Pin,
            noe: NOEPin,
            nwe: NWEPin,
            a0: A0Pin,
            d0: D0Pin,
            d1: D1Pin,
            d2: D2Pin,
            d3: D3Pin,
            d4: D4Pin,
            d5: D5Pin,
            d6: D6Pin,
            d7: D7Pin,
            d8: D8Pin,
            d9: D9Pin,
            d10: D10Pin,
            d11: D11Pin,
            d12: D12Pin,
            d13: D13Pin,
            d14: D14Pin,
            d15: D15Pin,
        );

        configure_sram_bank(p.fmc, BANK, TIMINGS);

        // SAFETY: the bank has just been configured, with its pins, and is only used by this
        // driver.
        #[expect(unsafe_code, reason = "the bank is configured above")]
        let interface = unsafe { FmcInterface::new(BANK, DCX_ADDRESS_LINE) };

        let reset = Output::new(p.reset, Level::High, Speed::Low);
        let display = Builder::new(ST7789, interface)
            .display_size(WIDTH, HEIGHT)
            .invert_colors(ColorInversion::Inverted)
            .reset_pin(reset)
            .init(delay)?;

        let backlight = Output::new(p.backlight, Level::High, Speed::Low);

        Ok(Self {
            display,
            backlight,
            _fmc_pins: fmc_pins,
        })
    }

    /// Returns the display, to draw on it.
    pub fn display(&mut self) -> &mut Display<'d> {
        &mut self.display
    }

    /// Turns the backlight on or off.
    pub fn set_backlight(&mut self, on: bool) {
        self.backlight.set_level(Level::from(on));
    }
}
