# lora-rx

## About

This application demonstrates raw LoRa reception with Ariel OS, using the
[`lora-phy`](https://crates.io/crates/lora-phy) crate.

Supported boards:

- ST B-L072Z-LRWAN1: Semtech SX1276 radio of the Murata CMWX1ZZABZ module, wired
  to `SPI1`.
- ST NUCLEO-WL55JC: sub-GHz radio (SX126x-compatible) embedded in the STM32WL,
  controlled through the internal `SUBGHZSPI` bus, using the low-power PA.

The board-specific radio setup is in `src/radio.rs`.

It listens continuously and logs every LoRa packet it receives, along with its
RSSI and SNR. It is the companion of the `lora-tx` example: flash `lora-tx` on a
second board tuned to the same frequency and modulation parameters (spreading
factor 10, 125 kHz bandwidth, coding rate 4/8) to see packets arrive here.

Adjust `LORA_FREQUENCY_IN_HZ` in `src/main.rs` so it matches the transmitter and
is legal in your region.

## How to run

In this directory, run

    laze build -b st-b-l072z-lrwan1 run

or

    laze build -b st-nucleo-wl55jc run
