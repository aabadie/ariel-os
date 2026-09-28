#[cfg(context = "st-b-l072z-lrwan1")]
use ariel_os::hal::spi;

// SPI1 is wired to the on-module Semtech SX1276 radio on the B-L072Z-LRWAN1.
#[cfg(context = "st-b-l072z-lrwan1")]
pub type RadioSpi = spi::main::SPI1;

#[cfg(context = "st-b-l072z-lrwan1")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PB3,
    spi_miso: PA6,
    spi_mosi: PA7,
    spi_cs: PA15, // Radio NSS
    reset: PC0,   // Radio RESET
    dio0: PB4,    // Radio DIO0, used as the TX/RX-done IRQ line
    tcxo: PA12,   // Powers the module TCXO (enabled at boot)
});

// The sub-GHz radio of the STM32WL is internal: only its SPI bus, the DMA channels used by that
// bus, and the RF switch control pins are needed.
#[cfg(context = "st-nucleo-wl55jc")]
ariel_os::hal::define_peripherals!(Peripherals {
    subghzspi: SUBGHZSPI,
    dma_tx: DMA1_CH1,
    dma_rx: DMA1_CH2,
    rf_ctrl1: PC4, // FE_CTRL1
    rf_ctrl2: PC5, // FE_CTRL2
    rf_ctrl3: PC3, // FE_CTRL3
});
