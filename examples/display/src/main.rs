#![no_main]
#![no_std]

mod pins;

use core::fmt::Write as _;

use ariel_os::{
    log::{error, info},
    reexports::embassy_time::Delay,
    time::{Instant, Timer},
};
use ariel_os_stm32_fmc_display::stm32f723e_disco::{HEIGHT, Lcd, WIDTH};
use embedded_graphics::{
    image::{Image, ImageRawBE},
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};

// Generated from `book/src/figures/ariel-hexacube-orange-rounded.svg`, see the README.
const LOGO_WIDTH: u32 = 133;
const LOGO: &[u8] = include_bytes!("../assets/ariel-logo-rgb565be.raw");

// Orange of the logo, (217, 75, 38) in RGB888.
const ARIEL_ORANGE: Rgb565 = Rgb565::new(217 >> 3, 75 >> 2, 38 >> 3);

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::DisplayPeripherals) {
    let Ok(mut lcd) = Lcd::new(peripherals.into_display_peripherals(), &mut Delay) else {
        error!("Failed to initialize the display");
        return;
    };
    info!("Display initialized");

    let display = lcd.display();
    let Ok(()) = display.clear(Rgb565::BLACK);

    // Center the logo horizontally, near the top of the screen.
    let logo = ImageRawBE::<Rgb565>::new(LOGO, LOGO_WIDTH);
    let logo_position = Point::new((i32::from(WIDTH) - LOGO_WIDTH.cast_signed()) / 2, 16);
    let Ok(()) = Image::new(&logo, logo_position).draw(display);

    let title_style = MonoTextStyle::new(&FONT_10X20, ARIEL_ORANGE);
    let Ok(_) = Text::with_alignment(
        "Ariel OS",
        Point::new(i32::from(WIDTH) / 2, 190),
        title_style,
        Alignment::Center,
    )
    .draw(display);

    // Redraw the background with the text so that the previous value is overwritten.
    let uptime_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::BLACK)
        .build();
    let uptime_position = Point::new(i32::from(WIDTH) / 2, i32::from(HEIGHT) - 16);

    loop {
        let mut uptime = heapless::String::<16>::new();
        let _ = write!(uptime, "uptime: {:>4}s", Instant::now().as_secs());
        let Ok(_) = Text::with_alignment(&uptime, uptime_position, uptime_style, Alignment::Center)
            .draw(display);

        Timer::after_secs(1).await;
    }
}
