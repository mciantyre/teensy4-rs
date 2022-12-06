# teensy4-pins

Hardware pins for the Teensy 4.0 and 4.1 boards

`teensy4-pins` is designed to the [`imxrt-iomuxc`] crate. The pins API constrains
the processor pads to the ones that are available on the Teensy 4.0 and 4.1. It also
exposes type aliases that simplify pin identification in the type system.

[`imxrt-iomuxc`]: https://docs.rs/imxrt_iomuxc/

Note that this pin API is optional. You are free to configure the pins using the
pad identifiers, instead of the physical pin identifiers. Pads are available directly
from the `imxrt-iomuxc` crate.

See the API documentation for more information.

License: MIT OR Apache-2.0
