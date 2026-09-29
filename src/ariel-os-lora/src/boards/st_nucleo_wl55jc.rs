//! ST NUCLEO-WL55JC: sub-GHz radio of the STM32WL, with an RF switch controlled by three GPIOs
//! (UM2592, section 6.6.3).

use ariel_os_hal::gpio::{Level, Output};
use lora_phy::{
    mod_params::RadioError,
    sx126x::{Config, Stm32wl, TcxoCtrlVoltage},
};

use crate::stm32wl::{self, RfSwitch};

ariel_os_hal::define_peripherals!(
    /// Peripherals used by the radio.
    Peripherals {
        subghzspi: SUBGHZSPI,
        dma_tx: DMA1_CH1,
        dma_rx: DMA1_CH2,
        rf_ctrl1: PC4, // FE_CTRL1
        rf_ctrl2: PC5, // FE_CTRL2
        rf_ctrl3: PC3, // FE_CTRL3
    }
);

/// [`lora_phy`] driver for the radio of the board.
pub type Radio = stm32wl::Radio<NucleoRfSwitch>;

/// RF switch of the board, using the low-power PA path for transmission.
pub struct NucleoRfSwitch {
    ctrl1: Output<'static>,
    ctrl2: Output<'static>,
    ctrl3: Output<'static>,
}

impl NucleoRfSwitch {
    fn set(&mut self, ctrl1: bool, ctrl2: bool, ctrl3: bool) {
        self.ctrl1.set_level(Level::from(ctrl1));
        self.ctrl2.set_level(Level::from(ctrl2));
        self.ctrl3.set_level(Level::from(ctrl3));
    }
}

impl RfSwitch for NucleoRfSwitch {
    fn set_rx(&mut self) {
        self.set(true, false, true);
    }

    fn set_tx(&mut self) {
        // Low-power PA path, matching `use_high_power_pa: false`.
        self.set(true, true, true);
    }

    fn set_off(&mut self) {
        self.set(false, false, false);
    }
}

/// Sets up the radio of the board.
///
/// # Errors
///
/// Returns an error if the radio fails to initialize.
pub async fn init(peripherals: Peripherals) -> Result<Radio, RadioError> {
    let rf_switch = NucleoRfSwitch {
        ctrl1: Output::new(peripherals.rf_ctrl1, Level::Low),
        ctrl2: Output::new(peripherals.rf_ctrl2, Level::Low),
        ctrl3: Output::new(peripherals.rf_ctrl3, Level::Low),
    };

    let config = Config {
        chip: Stm32wl {
            use_high_power_pa: false,
        },
        // The board TCXO is supplied by the PB0-VDDTCXO pin of the radio.
        tcxo_ctrl: Some(TcxoCtrlVoltage::Ctrl1V7),
        use_dcdc: true,
        rx_boost: false,
    };
    stm32wl::init(
        stm32wl::Peripherals {
            subghzspi: peripherals.subghzspi,
            dma_tx: peripherals.dma_tx,
            dma_rx: peripherals.dma_rx,
        },
        rf_switch,
        config,
    )
    .await
}
