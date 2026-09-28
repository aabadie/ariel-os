//! Board-specific setup of the LoRa radio.
//!
//! [`init()`] returns a [`lora_phy::LoRa`] driver, whose type depends on the radio of the board.

use ariel_os::time::Delay;
use lora_phy::LoRa;

pub use board::init;

#[cfg(context = "st-b-l072z-lrwan1")]
mod board {
    //! The board embeds a Semtech SX1276 radio (Murata CMWX1ZZABZ module) wired to `SPI1`.

    use ariel_os::{
        gpio::{Input, IntEnabledInput, Level, Output, Pull},
        hal,
        spi::main::{Kilohertz, SpiDevice, highest_freq_in},
    };
    use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
    use lora_phy::{
        iv::GenericSx127xInterfaceVariant,
        sx127x::{self, Sx127x, Sx1276},
    };
    use static_cell::StaticCell;

    use super::{Delay, LoRa};
    use crate::pins;

    pub type Radio = LoRa<
        Sx127x<
            SpiDevice<'static>,
            GenericSx127xInterfaceVariant<Output<'static>, IntEnabledInput<'static>>,
            Sx1276,
        >,
        Delay,
    >;

    static SPI_BUS: StaticCell<Mutex<CriticalSectionRawMutex, hal::spi::main::Spi>> =
        StaticCell::new();

    pub async fn init(peripherals: pins::Peripherals) -> Radio {
        let mut spi_config = hal::spi::main::Config::default();
        spi_config.frequency =
            const { highest_freq_in(Kilohertz::kHz(200)..=Kilohertz::kHz(1000)) };
        let spi_bus = SPI_BUS.init(Mutex::new(pins::RadioSpi::new(
            peripherals.spi_sck,
            peripherals.spi_miso,
            peripherals.spi_mosi,
            spi_config,
        )));

        let nss = Output::new(peripherals.spi_cs, Level::High);
        let spi = SpiDevice::new(spi_bus, nss);

        // `lora-phy` drives RESET and waits on DIO0; the module has no external RF switch pins to
        // control (hence `None, None`). The antenna is routed through the PA_BOOST pin.
        let reset = Output::new(peripherals.reset, Level::High);
        // Power the module TCXO for the lifetime of the application; the radio is configured
        // with `tcxo_used: true`.
        core::mem::forget(Output::new(peripherals.tcxo, Level::High));
        let irq = Input::builder(peripherals.dio0, Pull::None)
            .build_with_interrupt()
            .unwrap();
        let iv = GenericSx127xInterfaceVariant::new(reset, irq, None, None).unwrap();

        let config = sx127x::Config {
            chip: Sx1276,
            tcxo_used: true,
            rx_boost: false,
            tx_boost: true,
        };
        LoRa::new(Sx127x::new(spi, iv, config), false, Delay)
            .await
            .unwrap()
    }
}

#[cfg(context = "st-nucleo-wl55jc")]
mod board {
    //! The STM32WL embeds a Semtech SX126x-compatible sub-GHz radio.
    //!
    //! The radio is controlled through the internal `SUBGHZSPI` bus.
    //! Its NSS, BUSY, and RESET signals are not GPIOs but are accessed through the PWR and RCC
    //! registers, and its IRQ line is a dedicated interrupt.
    //! The board has an RF switch controlled by three GPIOs (UM2592, section 6.6.3).

    use ariel_os::gpio::{Level, Output};
    use embassy_stm32::{
        Peri, bind_interrupts,
        interrupt::{
            self, InterruptExt as _,
            typelevel::{Binding, Handler, SUBGHZ_RADIO},
        },
        mode::Async,
        pac, peripherals as pac_peripherals,
        spi::{Error as SpiError, Spi},
    };
    use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
    use embedded_hal_async::{
        delay::DelayNs,
        spi::{ErrorType, Operation, SpiBus, SpiDevice},
    };
    use lora_phy::{
        mod_params::RadioError,
        mod_traits::InterfaceVariant,
        sx126x::{self, Stm32wl, Sx126x, TcxoCtrlVoltage},
    };

    use super::{Delay, LoRa};
    use crate::pins;

    pub type Radio = LoRa<Sx126x<SubGhzSpiDevice, Stm32wlInterfaceVariant, Stm32wl>, Delay>;

    bind_interrupts!(struct Irqs {
        SUBGHZ_RADIO => RadioInterruptHandler;
    });

    pub async fn init(peripherals: pins::Peripherals) -> Radio {
        let spi = SubGhzSpiDevice::new(
            peripherals.subghzspi,
            peripherals.dma_tx,
            peripherals.dma_rx,
        );
        let iv = Stm32wlInterfaceVariant::new(
            Irqs,
            Output::new(peripherals.rf_ctrl1, Level::Low),
            Output::new(peripherals.rf_ctrl2, Level::Low),
            Output::new(peripherals.rf_ctrl3, Level::Low),
        );

        let config = sx126x::Config {
            chip: Stm32wl {
                use_high_power_pa: false,
            },
            // The board TCXO is supplied by the PB0-VDDTCXO pin of the radio.
            tcxo_ctrl: Some(TcxoCtrlVoltage::Ctrl1V7),
            use_dcdc: true,
            rx_boost: false,
        };
        LoRa::new(Sx126x::new(spi, iv, config), false, Delay)
            .await
            .unwrap()
    }

    /// [`SpiDevice`] for the `SUBGHZSPI` bus, whose NSS signal is driven through the PWR
    /// peripheral.
    pub struct SubGhzSpiDevice {
        bus: Spi<'static, Async>,
    }

    impl SubGhzSpiDevice {
        fn new(
            subghzspi: Peri<'static, pac_peripherals::SUBGHZSPI>,
            dma_tx: Peri<'static, pac_peripherals::DMA1_CH1>,
            dma_rx: Peri<'static, pac_peripherals::DMA1_CH2>,
        ) -> Self {
            Self {
                bus: Spi::new_subghz(subghzspi, dma_tx, dma_rx),
            }
        }

        fn set_nss(level: bool) {
            pac::PWR.subghzspicr().modify(|w| w.set_nss(level));
        }
    }

    impl ErrorType for SubGhzSpiDevice {
        type Error = SpiError;
    }

    impl SpiDevice for SubGhzSpiDevice {
        async fn transaction(
            &mut self,
            operations: &mut [Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            Self::set_nss(false);
            let result = async {
                for operation in operations {
                    match operation {
                        Operation::Read(buf) => SpiBus::read(&mut self.bus, buf).await?,
                        Operation::Write(buf) => SpiBus::write(&mut self.bus, buf).await?,
                        Operation::Transfer(read, write) => {
                            SpiBus::transfer(&mut self.bus, read, write).await?;
                        }
                        Operation::TransferInPlace(buf) => {
                            SpiBus::transfer_in_place(&mut self.bus, buf).await?;
                        }
                        Operation::DelayNs(ns) => Delay.delay_ns(*ns).await,
                    }
                }
                SpiBus::<u8>::flush(&mut self.bus).await
            }
            .await;
            Self::set_nss(true);
            result
        }
    }

    static RADIO_IRQ: Signal<CriticalSectionRawMutex, ()> = Signal::new();

    /// Handler of the radio interrupt; the interrupt is re-enabled when waiting for the next
    /// event.
    pub struct RadioInterruptHandler;

    #[expect(unsafe_code, reason = "interrupt handlers are unsafe to implement")]
    impl Handler<SUBGHZ_RADIO> for RadioInterruptHandler {
        unsafe fn on_interrupt() {
            interrupt::SUBGHZ_RADIO.disable();
            RADIO_IRQ.signal(());
        }
    }

    /// [`InterfaceVariant`] for the sub-GHz radio of the STM32WL on the NUCLEO-WL55JC.
    pub struct Stm32wlInterfaceVariant {
        // RF switch control pins.
        ctrl1: Output<'static>,
        ctrl2: Output<'static>,
        ctrl3: Output<'static>,
    }

    impl Stm32wlInterfaceVariant {
        fn new(
            _irqs: impl Binding<SUBGHZ_RADIO, RadioInterruptHandler>,
            rf_ctrl1: Output<'static>,
            rf_ctrl2: Output<'static>,
            rf_ctrl3: Output<'static>,
        ) -> Self {
            // Unmask the radio IRQ line (EXTI line 44) for CPU1.
            pac::EXTI
                .cpu(0)
                .imr(1)
                .modify(|w| w.set_line(44 - 32, true));

            Self {
                ctrl1: rf_ctrl1,
                ctrl2: rf_ctrl2,
                ctrl3: rf_ctrl3,
            }
        }

        fn set_rf_switch(&mut self, ctrl1: bool, ctrl2: bool, ctrl3: bool) {
            self.ctrl1.set_level(Level::from(ctrl1));
            self.ctrl2.set_level(Level::from(ctrl2));
            self.ctrl3.set_level(Level::from(ctrl3));
        }
    }

    impl InterfaceVariant for Stm32wlInterfaceVariant {
        async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), RadioError> {
            pac::RCC.csr().modify(|w| w.set_rfrst(true));
            delay.delay_ms(1).await;
            pac::RCC.csr().modify(|w| w.set_rfrst(false));
            while pac::RCC.csr().read().rfrstf() {}
            Ok(())
        }

        async fn wait_on_busy(&mut self) -> Result<(), RadioError> {
            while pac::PWR.sr2().read().rfbusys() {}
            Ok(())
        }

        async fn await_irq(&mut self) -> Result<(), RadioError> {
            RADIO_IRQ.reset();
            // SAFETY: the interrupt handler is bound through `Irqs`, and only signals
            // `RADIO_IRQ`.
            #[expect(unsafe_code, reason = "enabling the bound radio interrupt")]
            unsafe {
                interrupt::SUBGHZ_RADIO.enable();
            }
            RADIO_IRQ.wait().await;
            Ok(())
        }

        async fn enable_rf_switch_rx(&mut self) -> Result<(), RadioError> {
            self.set_rf_switch(true, false, true);
            Ok(())
        }

        async fn enable_rf_switch_tx(&mut self) -> Result<(), RadioError> {
            // Low-power PA path, matching `use_high_power_pa: false`.
            self.set_rf_switch(true, true, true);
            Ok(())
        }

        async fn disable_rf_switch(&mut self) -> Result<(), RadioError> {
            self.set_rf_switch(false, false, false);
            Ok(())
        }
    }
}
