//! LoRa radio support for Ariel OS boards.
//!
//! This crate is available as `ariel_os::lora`, when the `lora` laze module is selected.
//!
//! This crate sets up the LoRa radio of the board and returns a [`lora_phy::LoRa`] driver, so that
//! applications do not depend on the radio chip or on how it is connected:
//!
//! ```ignore
//! #[ariel_os::task(autostart, peripherals)]
//! async fn main(peripherals: ariel_os::lora::Peripherals) {
//!     let mut lora = ariel_os::lora::init(peripherals).await.unwrap();
//!     // Use `lora` with the `lora_phy` API.
//! }
//! ```
//!
//! The crate is organized in three layers:
//!
//! - radio chip modules ([`sx127x`], `stm32wl`) set up a radio from its bus and control signals,
//!   independently of the board;
//! - board modules describe how the radio is connected on a given board, and are selected by the
//!   laze context;
//! - the `lora` laze module enables this crate, and restricts applications to boards with a
//!   supported radio.
#![no_std]
#![allow(
    clippy::doc_markdown,
    reason = "\"LoRa\" and radio chip names trip the lint"
)]

pub mod sx127x;

#[cfg(feature = "stm32wl")]
pub mod stm32wl;

pub use lora_phy;

cfg_select! {
    context = "st-b-l072z-lrwan1" => {
        #[path = "boards/st_b_l072z_lrwan1.rs"]
        mod board;
    }
    context = "st-nucleo-wl55jc" => {
        #[path = "boards/st_nucleo_wl55jc.rs"]
        mod board;
    }
    _ => {
        compile_error!("no LoRa radio is supported on this board");
    }
}

pub use board::{Peripherals, Radio, init};
