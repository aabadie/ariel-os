//! Semtech SX127x radios (SX1272, SX1276, ...) connected through SPI and GPIOs.
//!
//! This module is independent of the MCU family: it only uses the Ariel OS SPI and GPIO drivers.

use ariel_os_embassy::spi::main::SpiDevice;
use ariel_os_hal::{
    gpio::{IntEnabledInput, Output},
    hal::spi::main::Spi,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Delay;
use lora_phy::{LoRa, iv::GenericSx127xInterfaceVariant, mod_params::RadioError, sx127x::Sx127x};
use static_cell::StaticCell;

/// Interface variant of the radio, driving its RESET signal and waiting on its DIO0 signal.
pub type InterfaceVariant =
    GenericSx127xInterfaceVariant<Output<'static>, IntEnabledInput<'static>>;

/// [`lora_phy`] driver for an SX127x radio, `C` being the radio variant (e.g.,
/// [`lora_phy::sx127x::Sx1276`]).
pub type Radio<C> = LoRa<Sx127x<SpiDevice<'static>, InterfaceVariant, C>, Delay>;

static SPI_BUS: StaticCell<Mutex<CriticalSectionRawMutex, Spi>> = StaticCell::new();

/// Control signals of an SX127x radio.
pub struct Signals {
    /// Chip select of the radio, initially high.
    pub nss: Output<'static>,
    /// RESET signal of the radio, initially high.
    pub reset: Output<'static>,
    /// DIO0 signal of the radio, used as the TX/RX-done IRQ line.
    pub dio0: IntEnabledInput<'static>,
    /// Optional RF switch control for reception.
    pub rf_switch_rx: Option<Output<'static>>,
    /// Optional RF switch control for transmission.
    pub rf_switch_tx: Option<Output<'static>>,
}

/// Creates the SPI device and the interface variant of an SX127x radio on the SPI bus `spi`.
///
/// The radio driver is then created with [`lora_phy::sx127x::Sx127x::new()`] and
/// [`lora_phy::LoRa::new()`], using the variant of the radio.
///
/// # Panics
///
/// Panics if called more than once.
///
/// # Errors
///
/// Returns an error if the interface variant cannot be created.
pub fn interface(
    spi: Spi,
    signals: Signals,
) -> Result<(SpiDevice<'static>, InterfaceVariant), RadioError> {
    let spi_bus = SPI_BUS.init(Mutex::new(spi));
    let spi = SpiDevice::new(spi_bus, signals.nss);

    let iv = GenericSx127xInterfaceVariant::new(
        signals.reset,
        signals.dio0,
        signals.rf_switch_rx,
        signals.rf_switch_tx,
    )?;

    Ok((spi, iv))
}
