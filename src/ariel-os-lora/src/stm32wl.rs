//! Sub-GHz radio embedded in STM32WL MCUs.
//!
//! The radio is compatible with the Semtech SX126x; access to it is provided by the HAL, see
//! `ariel_os_hal::hal::subghz`.
//! The RF switch in front of the antenna is board-specific, see [`RfSwitch`].

use ariel_os_hal::hal::{
    PeripheralType, peripherals,
    subghz::{self, Control, RxDma, SubGhzSpiDevice, TxDma},
};
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use lora_phy::{
    LoRa,
    mod_params::RadioError,
    mod_traits::InterfaceVariant as InterfaceVariantTrait,
    sx126x::{Config, Stm32wl, Sx126x},
};

pub use subghz::Peripherals;

/// [`lora_phy`] driver for the sub-GHz radio.
pub type Radio<S> = LoRa<Sx126x<SubGhzSpiDevice, InterfaceVariant<S>, Stm32wl>, Delay>;

/// Board-specific control of the RF switch in front of the antenna.
pub trait RfSwitch {
    /// Connects the antenna to the receive path.
    fn set_rx(&mut self);
    /// Connects the antenna to the transmit path of the PA in use.
    fn set_tx(&mut self);
    /// Disconnects the antenna.
    fn set_off(&mut self);
}

/// Creates the [`lora_phy`] driver for the sub-GHz radio.
///
/// # Errors
///
/// Returns an error if the radio fails to initialize.
pub async fn init<S: RfSwitch, T, R>(
    peripherals: Peripherals<T, R>,
    rf_switch: S,
    config: Config<Stm32wl>,
) -> Result<Radio<S>, RadioError>
where
    T: PeripheralType + TxDma<peripherals::SUBGHZSPI>,
    R: PeripheralType + RxDma<peripherals::SUBGHZSPI>,
{
    let (spi, control) = subghz::new(peripherals);
    let iv = InterfaceVariant { control, rf_switch };

    LoRa::new(Sx126x::new(spi, iv, config), false, Delay).await
}

/// [`lora_phy`] interface variant for the sub-GHz radio.
pub struct InterfaceVariant<S> {
    control: Control,
    rf_switch: S,
}

impl<S: RfSwitch> InterfaceVariantTrait for InterfaceVariant<S> {
    async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), RadioError> {
        self.control.reset(delay).await;
        Ok(())
    }

    async fn wait_on_busy(&mut self) -> Result<(), RadioError> {
        while self.control.is_busy() {}
        Ok(())
    }

    async fn await_irq(&mut self) -> Result<(), RadioError> {
        self.control.wait_for_irq().await;
        Ok(())
    }

    async fn enable_rf_switch_rx(&mut self) -> Result<(), RadioError> {
        self.rf_switch.set_rx();
        Ok(())
    }

    async fn enable_rf_switch_tx(&mut self) -> Result<(), RadioError> {
        self.rf_switch.set_tx();
        Ok(())
    }

    async fn disable_rf_switch(&mut self) -> Result<(), RadioError> {
        self.rf_switch.set_off();
        Ok(())
    }
}
