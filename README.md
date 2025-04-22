# nRF54L simple board configurator

Since Nordic currently doesn't provide `nrfutil` for ARM and
I needed to turn on/off everything on the dev kit for automatic current
measurements, I made my own, trivial nRF54L15-DK board configurator.

## C version
It needs the `libusb-dev` package.

Build it with make, then run it:
- `./sbc on` to turn on everything on the board.
- `./sbc off` to turn off everything on the board.

## Rust version
Build it and run it with:
- `cargo run on` to turn on everything on the board.
- `cargo run off` to turn off everything on the board.

## Warning

This comes with zero warrenties.

The code has only been tested on my dev kit.

I've copied the two USB requests that are being sent by nrfutil when
turning on/off the pins, I have no idea of the protocol and weather
it is safe to run on another DK.

It could be that those requests instead cause a fire when run on a dev kit
from another revision, or something even worse. If that happens, enjoy the bbq, but don't blame me.
