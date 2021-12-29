# Changelog

## Unreleased

## [0.2.0] - 2021-12-29

- Re-export `imxrt-iomuxc` from the crate. Use these symbols to access pad
  configuration APIs.
- Document all alternate values for all Teensy 4 pins.
- Add Teensy 4.1 pins 42 through 47 for SD card drivers. This changes the size
  of the `ErasedPads` array from 42 to 48. This may **break** your code if you
  are not using the `ErasedPads` type alias.

## [0.1.0] - 2020-10-06

First release.

[0.2.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-pins-0.1.0...teensy4-pins-0.2.0
[0.1.0]: https://github.com/mciantyre/teensy4-rs/releases/tag/teensy4-pins-0.1.0
