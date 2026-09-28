# display

## About

This application demonstrates how to use the LCD of the STM32F723E-DISCO board, connected to the
FMC of the MCU, with [`embedded-graphics`](https://docs.rs/embedded-graphics).
It displays the Ariel OS logo and the uptime of the board.

## How to run

In this directory, run

    laze build -b stm32f723e-disco run

## Logo

The logo image in `assets/` is raw RGB565 big-endian pixel data, 133 pixels wide and 150 pixels
high. It was generated from `book/src/figures/ariel-hexacube-orange-rounded.svg` with Inkscape and
Pillow:

```bash
inkscape ../../book/src/figures/ariel-hexacube-orange-rounded.svg --export-type=png \
  --export-height=150 --export-background=black --export-background-opacity=1 \
  --export-filename=logo.png
python3 -c '
from PIL import Image
im = Image.open("logo.png").convert("RGB")
data = bytearray()
for r, g, b in im.getdata():
    data += (((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3)).to_bytes(2, "big")
open("assets/ariel-logo-rgb565be.raw", "wb").write(data)
'
```
