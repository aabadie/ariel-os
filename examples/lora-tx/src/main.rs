//! LoRa transmit (raw PHY) example.
//!
//! This example uses the HAL-agnostic [`lora_phy`] driver to transmit a small LoRa packet every
//! few seconds. Pair it with the `lora-rx` example running on a second board, tuned to the same
//! frequency and modulation parameters, to observe the packets being received.
//! The board-specific radio setup is in the [`radio`] module.
#![no_main]
#![no_std]
// "LoRa", "SX1276" and similar domain terms trip the doc_markdown lint.
#![allow(clippy::doc_markdown)]

mod pins;
mod radio;

use ariel_os::{log::info, time::Timer};
use lora_phy::mod_params::{Bandwidth, CodingRate, SpreadingFactor};

/// LoRa carrier frequency. Set this to a value legal in your region (this is an
/// EU868 channel).
const LORA_FREQUENCY_IN_HZ: u32 = 868_100_000;

/// Transmit power in dBm; 14 dBm is the EU868 limit.
const TX_POWER_DBM: i32 = 14;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let mut lora = radio::init(peripherals).await;

    let modulation = lora
        .create_modulation_params(
            SpreadingFactor::_10,
            Bandwidth::_125KHz,
            CodingRate::_4_8,
            LORA_FREQUENCY_IN_HZ,
        )
        .unwrap();
    let mut tx_params = lora
        .create_tx_packet_params(4, false, true, false, &modulation)
        .unwrap();

    let mut counter: u8 = 0;
    loop {
        let buffer = [0xAB, counter];

        lora.prepare_for_tx(&modulation, &mut tx_params, TX_POWER_DBM, &buffer)
            .await
            .unwrap();
        lora.tx().await.unwrap();
        lora.sleep(false).await.unwrap();

        info!("LoRa packet sent (counter = {})", counter);
        counter = counter.wrapping_add(1);

        Timer::after_secs(5).await;
    }
}
