#![deny(clippy::pedantic)]

#[cfg(context = "nrf")]
type UartDriver = embassy_nrf::uarte::Uarte<'static>;
#[cfg(context = "rp2040")]
type UartDriver = embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>;
#[cfg(context = "stm32")]
type UartDriver = embassy_stm32::usart::Uart<'static, embassy_stm32::mode::Blocking>;
#[cfg(context = "scum")]
type UartDriver = crate::hal::uart::Uart;

static DEBUG_UART: embassy_sync::once_lock::OnceLock<
    embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        UartDriver,
    >,
> = embassy_sync::once_lock::OnceLock::new();

#[expect(clippy::missing_panics_doc)]
pub fn init(peripherals: &mut crate::hal::OptionalPeripherals) {
    // TODO: this could later be replaced with our UART abstraction and app configuration.
    #[cfg(not(context = "scum"))]
    let uart = iot_lab::get_uart_driver(peripherals);

    // The SCuM UART needs no configuration: the baud rate is fixed by the
    // chip clocking.
    #[cfg(context = "scum")]
    let uart = crate::hal::uart::Uart::new(peripherals.UART0.take().unwrap());

    let _ = DEBUG_UART.init(embassy_sync::mutex::Mutex::new(uart));

    let _ = ariel_os_log::backend::DEBUG_UART_WRITE_FN.init(write_debug_uart);
}

/// Writes the buffer to the debug UART.
///
/// # Errors
///
/// Returns an error if writing to the UART fails.
fn write_debug_uart(buffer: &[u8]) -> Result<(), ariel_os_log::backend::Error> {
    #[cfg(not(context = "scum"))]
    use ariel_os_log::backend::Error;

    #[cfg(any(context = "rp", context = "stm32"))]
    use embedded_io::Write as _;
    #[cfg(context = "nrf")]
    use embedded_io_async::Write as _;

    embassy_futures::block_on(async {
        // This effectively drops logs until the UART driver is populated.
        // If we instead waited on it to be set, this would deadlock when trying to print
        // on the logging output before the driver is populated.
        if let Some(uart) = DEBUG_UART.try_get() {
            let mut uart = uart.lock().await;

            #[cfg(context = "nrf")]
            {
                uart.write(buffer).await.map_err(|_| Error::Writing)?;
                // TODO: is flushing needed here?
                uart.flush().await.map_err(|_| Error::Writing)?;
            }

            #[cfg(context = "rp")]
            {
                uart.write_all(buffer).map_err(|_| Error::Writing)?;
                // TODO: is flushing needed here?
                uart.flush().map_err(|_| Error::Writing)?;
            }

            #[cfg(context = "stm32")]
            {
                uart.write(buffer).map_err(|_| Error::Writing)?;
                // TODO: is flushing needed here?
                uart.flush().map_err(|_| Error::Writing)?;
            }

            #[cfg(context = "scum")]
            uart.write_bytes(buffer);
        }

        Ok(())
    })
}

mod iot_lab {
    //! UART configuration required for [IoT-LAB](https://www.iot-lab.info).

    #[cfg(context = "nrf")]
    pub fn get_uart_driver(peripherals: &mut crate::hal::OptionalPeripherals) -> super::UartDriver {
        let mut config = embassy_nrf::uarte::Config::default();

        embassy_nrf::bind_interrupts!(struct Irqs {
            UARTE0 => embassy_nrf::uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
        });

        #[cfg(any(context = "nrf52dk", context = "nrf52840dk"))]
        let (p, uart_rx, uart_tx) = {
            // https://www.iot-lab.info/docs/boards/nordic-nrf52840dk/
            config.baudrate = embassy_nrf::buffered_uarte::Baudrate::BAUD115200;
            (
                peripherals.UARTE0.take().unwrap(),
                peripherals.P0_08.take().unwrap(),
                peripherals.P0_06.take().unwrap(),
            )
        };

        embassy_nrf::uarte::Uarte::new(p, uart_rx, uart_tx, Irqs, config)
    }

    #[cfg(context = "rp2040")]
    pub fn get_uart_driver(peripherals: &mut crate::hal::OptionalPeripherals) -> super::UartDriver {
        let mut config = embassy_rp::uart::Config::default();

        #[cfg(any(context = "rpi-pico", context = "rpi-pico-w"))]
        let (p, uart_rx, uart_tx) = {
            config.baudrate = 115_200;
            (
                peripherals.UART0.take().unwrap(),
                peripherals.PIN_1.take().unwrap(),
                peripherals.PIN_0.take().unwrap(),
            )
        };

        embassy_rp::uart::Uart::new_blocking(p, uart_tx, uart_rx, config)
    }

    #[cfg(context = "stm32")]
    pub fn get_uart_driver(peripherals: &mut crate::hal::OptionalPeripherals) -> super::UartDriver {
        let mut config = embassy_stm32::usart::Config::default();

        #[cfg(any(context = "st-b-l475e-iot01a", context = "st-nucleo-wb55"))]
        let (p, uart_rx, uart_tx) = {
            // https://www.iot-lab.info/docs/boards/st-b-l475e-iot01a/
            config.baudrate = 115_200;
            (
                peripherals.USART1.take().unwrap(),
                peripherals.PB7.take().unwrap(),
                peripherals.PB6.take().unwrap(),
            )
        };

        #[cfg(context = "stm32u083c-dk")]
        let (p, uart_rx, uart_tx) = {
            config.baudrate = 115_200;
            (
                peripherals.USART2.take().unwrap(),
                peripherals.PA3.take().unwrap(),
                peripherals.PA2.take().unwrap(),
            )
        };

        embassy_stm32::usart::Uart::new_blocking(p, uart_rx, uart_tx, config).unwrap()
    }
}
