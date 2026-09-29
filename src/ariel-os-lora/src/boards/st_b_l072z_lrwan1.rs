//! ST B-L072Z-LRWAN1: Semtech SX1276 radio of the Murata CMWX1ZZABZ module, wired to `SPI1`.

use ariel_os_embassy::spi::main::{Kilohertz, highest_freq_in};
use ariel_os_hal::{
    gpio::{Input, Level, Output, Pull},
    hal::spi::main::{Config as SpiConfig, SPI1},
};
use embassy_time::Delay;
use lora_phy::{
    LoRa,
    mod_params::RadioError,
    sx127x::{Config, Sx127x, Sx1276},
};

use crate::sx127x;

ariel_os_hal::define_peripherals!(
    /// Peripherals used by the radio.
    Peripherals {
        spi_sck: PB3,
        spi_miso: PA6,
        spi_mosi: PA7,
        spi_cs: PA15, // Radio NSS
        reset: PC0,   // Radio RESET
        dio0: PB4,    // Radio DIO0, used as the TX/RX-done IRQ line
        tcxo: PA12,   // Powers the module TCXO
    }
);

/// [`lora_phy`] driver for the radio of the board.
pub type Radio = sx127x::Radio<Sx1276>;

/// Sets up the radio of the board.
///
/// # Errors
///
/// Returns an error if the radio fails to initialize.
pub async fn init(peripherals: Peripherals) -> Result<Radio, RadioError> {
    let mut spi_config = SpiConfig::default();
    spi_config.frequency = const { highest_freq_in(Kilohertz::kHz(200)..=Kilohertz::kHz(1000)) };
    let spi = SPI1::new(
        peripherals.spi_sck,
        peripherals.spi_miso,
        peripherals.spi_mosi,
        spi_config,
    );

    // Power the module TCXO for the lifetime of the application; the radio is configured with
    // `tcxo_used: true`.
    core::mem::forget(Output::new(peripherals.tcxo, Level::High));

    let signals = sx127x::Signals {
        nss: Output::new(peripherals.spi_cs, Level::High),
        reset: Output::new(peripherals.reset, Level::High),
        dio0: Input::builder(peripherals.dio0, Pull::None)
            .build_with_interrupt()
            .map_err(|_| RadioError::Irq)?,
        // The module has no external RF switch pins to control.
        rf_switch_rx: None,
        rf_switch_tx: None,
    };

    let config = Config {
        chip: Sx1276,
        tcxo_used: true,
        rx_boost: false,
        // The antenna is routed through the PA_BOOST pin.
        tx_boost: true,
    };
    let (spi, iv) = sx127x::interface(spi, signals)?;
    LoRa::new(Sx127x::new(spi, iv, config), false, Delay).await
}
