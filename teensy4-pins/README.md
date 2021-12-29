# teensy4-pins

Hardware pins for the Teensy 4.0 and 4.1 boards

`teensy4-pins` is designed to the [`imxrt-iomuxc`] crate. The pins API constrains
the processor pads to the ones that are available on the Teensy 4.0 and 4.1. It also
exposes type aliases that simplify pin identification in the type system.

[`imxrt-iomuxc`]: https://docs.rs/imxrt-iomuxc/0.1/imxrt_iomuxc/

Note that this pin API is optional. You are free to configure the pins using the
pad identifiers, instead of the physical pin identifiers. Pads are available directly
from the `imxrt-iomuxc` crate.

## Teensy 4.0

To acquire Teensy 4.0 pins, call `t40::from_pads` and provide all
of the processor pads:

```rust
use teensy4_pins::t40;

let pads = // Handle to all processor pads
    # unsafe { Pads::new() };
let pins = t40::from_pads(pads);
```

## Teensy 4.1

The approach is the same as the Teensy 4.0, replacing `t40` with `t41`:

```rust
use teensy4_pins::t41;

let pads = // Handle to all processor pads
    # unsafe { Pads::new() };
let pins = t41::from_pads(pads);
```

## Safety

The safe APIs expect to work on the only instance of the processor pads. If you don't have that
available, or you need more flexibility, use the unsafe [`t40::Pin::new`](t40/struct.Pins.html#method.new)
or [`t41::Pins::new`](t41/struct.Pins.html#method.new) constructor methods to create an instance
that may be aliasing another handle to the pads or pins.

License: MIT OR Apache-2.0
