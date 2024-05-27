# Changelog

## Unreleased

## [0.3.2] - 2024-05-27

Re-export and document configuration APIs available from imxrt-iomuxc.

## [0.3.1] - 2023-02-09

Add Teensy MicroMod pins in the `tmm` module.

## [0.3.0] - 2023-01-05

**BREAKING** Update to imxrt-iomuxc 0.2. Pin aliases remain the same, but pad
types are different.

**BREAKING** Update to Rust 2021 edition.

Add Teensy 4.1 pins 48 though 54. This increases the size of the `ErasedPads`
array and represents a **breaking** API change. Users who design to the
`ErasedPads` type alias should not be affected by this breakage.

**BREAKING** Remove `into_pins()`, which was deprecated in the previous
release. Users should use `from_pads()`.

## [0.2.0] - 2021-12-29

- Mark functions `#[inline]`.
- Add `from_pads()`, a replacement for `into_pins()`.
- *Deprecate* `into_pins()`. Users should prefer `from_pads()` to convert
  from IOMUXC pads to Teensy 4 pins.
- Re-export `imxrt-iomuxc` from the crate. Use these symbols to access pad
  configuration APIs.
- Document all alternate values for all Teensy 4 pins.
- Add Teensy 4.1 pins 42 through 47 for SD card drivers. This changes the size
  of the `ErasedPads` array from 42 to 48. This may **break** your code if you
  are not using the `ErasedPads` type alias.

## [0.1.0] - 2020-10-06

First release.

[0.3.2]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-pins-0.3.1...teensy4-pins-0.3.2
[0.3.1]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-pins-0.3.0...teensy4-pins-0.3.1
[0.3.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-pins-0.2.0...teensy4-pins-0.3.0
[0.2.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-pins-0.1.0...teensy4-pins-0.2.0
[0.1.0]: https://github.com/mciantyre/teensy4-rs/releases/tag/teensy4-pins-0.1.0
