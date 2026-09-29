//! Sub-GHz radio embedded in STM32WL MCUs.
//!
//! The radio is compatible with the Semtech `SX126x`, and is controlled through the internal
//! `SUBGHZSPI` bus.
//! Its NSS, BUSY, and RESET signals are not GPIOs but are accessed through the PWR and RCC
//! registers, and its IRQ line is a dedicated interrupt.
//!
//! This module only provides access to the radio; radio drivers are implemented on top of it.

use embassy_stm32::{
    Peri, PeripheralType, bind_interrupts,
    interrupt::{
        self, InterruptExt as _,
        typelevel::{Handler, SUBGHZ_RADIO},
    },
    mode::Async,
    pac, peripherals,
    spi::{self, Spi},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Delay;
use embedded_hal_async::{
    delay::DelayNs,
    spi::{ErrorType, Operation, SpiBus, SpiDevice},
};

/// DMA channel traits required by [`Peripherals`].
pub use embassy_stm32::spi::{RxDma, TxDma};

bind_interrupts!(struct Irqs {
    SUBGHZ_RADIO => RadioInterruptHandler;
});

static RADIO_IRQ: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Handler of the radio interrupt; the interrupt is re-enabled when waiting for the next event.
struct RadioInterruptHandler;

#[expect(unsafe_code, reason = "interrupt handlers are unsafe to implement")]
impl Handler<SUBGHZ_RADIO> for RadioInterruptHandler {
    unsafe fn on_interrupt() {
        interrupt::SUBGHZ_RADIO.disable();
        RADIO_IRQ.signal(());
    }
}

/// Peripherals used by the sub-GHz radio.
pub struct Peripherals<T: PeripheralType + 'static, R: PeripheralType + 'static> {
    /// The internal SPI bus of the radio.
    pub subghzspi: Peri<'static, peripherals::SUBGHZSPI>,
    /// DMA channel used to transmit on the SPI bus.
    pub dma_tx: Peri<'static, T>,
    /// DMA channel used to receive on the SPI bus.
    pub dma_rx: Peri<'static, R>,
}

/// Returns the SPI device and the control signals of the sub-GHz radio.
pub fn new<T, R>(peripherals: Peripherals<T, R>) -> (SubGhzSpiDevice, Control)
where
    T: PeripheralType + TxDma<peripherals::SUBGHZSPI>,
    R: PeripheralType + RxDma<peripherals::SUBGHZSPI>,
{
    let spi = SubGhzSpiDevice {
        bus: Spi::new_subghz(
            peripherals.subghzspi,
            peripherals.dma_tx,
            peripherals.dma_rx,
        ),
    };

    // `Irqs` only exists to bind the interrupt handler.
    let _ = Irqs;
    // Unmask the radio IRQ line (EXTI line 44) for CPU1.
    pac::EXTI
        .cpu(0)
        .imr(1)
        .modify(|w| w.set_line(44 - 32, true));

    (spi, Control { _private: () })
}

/// [`SpiDevice`] for the `SUBGHZSPI` bus, whose NSS signal is driven through the PWR peripheral.
pub struct SubGhzSpiDevice {
    bus: Spi<'static, Async>,
}

impl SubGhzSpiDevice {
    fn set_nss(level: bool) {
        pac::PWR.subghzspicr().modify(|w| w.set_nss(level));
    }
}

impl ErrorType for SubGhzSpiDevice {
    type Error = spi::Error;
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

/// RESET, BUSY, and IRQ signals of the sub-GHz radio.
pub struct Control {
    _private: (),
}

impl Control {
    /// Resets the radio.
    pub async fn reset(&mut self, delay: &mut impl DelayNs) {
        pac::RCC.csr().modify(|w| w.set_rfrst(true));
        delay.delay_ms(1).await;
        pac::RCC.csr().modify(|w| w.set_rfrst(false));
        while pac::RCC.csr().read().rfrstf() {}
    }

    /// Returns whether the radio is busy.
    #[must_use]
    pub fn is_busy(&self) -> bool {
        pac::PWR.sr2().read().rfbusys()
    }

    /// Waits for the next radio interrupt.
    pub async fn wait_for_irq(&mut self) {
        RADIO_IRQ.reset();
        // SAFETY: the interrupt handler is bound through `Irqs`, and only signals `RADIO_IRQ`.
        #[expect(unsafe_code, reason = "enabling the bound radio interrupt")]
        unsafe {
            interrupt::SUBGHZ_RADIO.enable();
        }
        RADIO_IRQ.wait().await;
    }
}
