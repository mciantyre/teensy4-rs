# Changelog

## Unreleased

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

[0.2.0]: https://github.com/mciantyre/teensy4-rs/compare/teensy4-pins-0.1.0...teensy4-pins-0.2.0
[0.1.0]: https://github.com/mciantyre/teensy4-rs/releases/tag/teensy4-pins-0.1.0
