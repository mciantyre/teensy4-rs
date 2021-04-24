# teensy4-panic

Panic handler for the Teensy 4

When you link `teensy4-panic` into your program, any `panic!()` will cause
your Teensy's LED to blink S.O.S. in Morse code. Supports both Teensy 4.0 and
4.1 boards.

## Usage

Depend on `teensy4-panic`:

```toml
[dependencies]
teensy4-panic = "0.2"
```

Then, include the crate in your final program:

```rust
use teensy4_panic as _;
```

Finally, use `panic!()` to stop the program, and blink the LED.

## Custom panic handlers

By default, `teensy4-panic` enables the `panic-handler` feature. If you want
to use the S.O.S. routine in your own, custom panic handler, disable the default
features, and call `sos()`:

```toml
[dependencies]
teensy4-panic = "0.2"
default-features = false
```

```rust
use teensy4_panic::sos;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // ...
    sos()
}
```

License: MIT OR Apache-2.0
