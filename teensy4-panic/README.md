# teensy4-panic

Panic handler for the Teensy 4.

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

Finally, use `panic!()` to stop the program and blink the LED.

## Features

The table below summarizes this crate's features. Each subsection details the feature.

| Feature         |         Description                                | Default feature? |
| --------------- | -------------------------------------------------- | ---------------- |
| `panic-handler` | Define the Teensy 4's panic handler in this crate  |        ✓         |
| `log`           | Log the panic message using `log::error!`          |                  |

It does not make sense to _disable_ `panic-handler` and _enable_ `log`, since logging can
only happen when `panic-handler` is enabled.

### Custom panic handlers

By default, `teensy4-panic` enables the `panic-handler` feature. If you want
to use the S.O.S. routine in your own panic handler, disable the default
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
    // Your panic handler here...
    sos()
}
```

### Log the panic message

If the `log` feature is enabled, the crate links with [the `log` crate](https://crates.io/crates/log).
Before blinking, the panic handler logs the panic message and source location at the error priority,
using `log::error!`. The logging target is `teensy4_panic`.

The example below shows a `panic!` and an example of its corresponding log message:

```rust
const DELAY_MS: u32 = 5_000;
panic!("This is a panic message written after {}ms", DELAY_MS);
```
```
[ERROR teensy4_panic]: panicked at 'This is a panic message written after 5000ms', examples/panic_log.rs:22:5
```

The panic handler only emits the log message once. You're responsible for making sure that the
log message can reach its destination while a `panic!` is active.

License: MIT OR Apache-2.0
