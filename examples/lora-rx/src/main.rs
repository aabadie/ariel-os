//! LoRa receive (raw PHY) example.
//!
//! This example uses the HAL-agnostic `lora-phy` driver to listen for LoRa packets continuously
//! and log each one it receives. It is the companion of the `lora-tx` example: run `lora-tx` on a
//! second board tuned to the same frequency and modulation parameters to see packets arrive here.
//! The board-specific radio setup is provided by `ariel_os::lora`.
#![no_main]
#![no_std]
// "LoRa", "SX1276" and similar domain terms trip the doc_markdown lint.
#![allow(clippy::doc_markdown)]

use ariel_os::log::info;
use ariel_os::lora::lora_phy::{
    RxMode,
    mod_params::{Bandwidth, CodingRate, SpreadingFactor},
};

/// LoRa carrier frequency. Must match the transmitter (this is an EU868
/// channel).
const LORA_FREQUENCY_IN_HZ: u32 = 868_100_000;

/// Largest payload the receiver will accept, in bytes.
const MAX_PAYLOAD_LEN: u8 = 64;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: ariel_os::lora::Peripherals) {
    let mut lora = ariel_os::lora::init(peripherals).await.unwrap();

    // These must match the transmitter's parameters exactly.
    let modulation = lora
        .create_modulation_params(
            SpreadingFactor::_10,
            Bandwidth::_125KHz,
            CodingRate::_4_8,
            LORA_FREQUENCY_IN_HZ,
        )
        .unwrap();
    let rx_params = lora
        .create_rx_packet_params(4, false, MAX_PAYLOAD_LEN, true, false, &modulation)
        .unwrap();

    lora.prepare_for_rx(RxMode::Continuous, &modulation, &rx_params)
        .await
        .unwrap();

    info!("Listening for LoRa packets...");
    let mut buffer = [0u8; MAX_PAYLOAD_LEN as usize];
    loop {
        if let Ok((len, status)) = lora.rx(&rx_params, &mut buffer).await {
            info!(
                "Received {} bytes (rssi = {} dBm, snr = {})",
                len, status.rssi, status.snr
            );
        } else {
            info!("Receive error");
        }
    }
}
