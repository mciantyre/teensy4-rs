# iMXRT1062 Peripheral Access Crates

The top-level peripheral access crate (PAC), `imxrt1062-pac`, re-exports all processor peripheral crates. The code in these crates os auto-generated using `svd2rust`, then migrated into this file structure using some automated tooling and human touch.

The rest of the README describes the full, semi-automated process of adding a new iMXRT1062 peripheral into the PAC. The audience for this walkthrough is an embedded Rust developer who is interested in using an iMXRT1062 peripheral, and that peripheral is not already specified in the PAC.

## Requirements

Before adding a peripheral, ensure that you have the iMXRT1062 SVD file. SVD files may be found by searching for the iMXRT1062 [here](https://developer.arm.com/tools-and-software/embedded/cmsis). After acquiring the iMXRT1062 SVD, follow the [`svd2rust` instructions](https://docs.rs/svd2rust/0.16.1/svd2rust/). As recommended in the `svd2rust` documentation, use `form` to split out the peripherals into their own modules.

## Adding a peripheral crate

To simplify the example, let's add the ADC peripheral (specified as `adc1` in the `svd2rust` output) to our PAC. Assuming we're at the root of this repository, use the binary available at `tools/main.rs` to copy the peripheral module into its own crate.

```bash
$ cd tools
$ cargo run  --target x86_64-apple-darwin -- /path/to/svd2rust/output ../imxrt1062-pac adc1
```

In the example snippet above, ensure that the `--target` flag is set for your host system. Specifying the `--target` may be necessary because the repository workspace is configured for `thumbv7em-none-eabihf`. The first argument, `/path/to/svd2rust/output`, is the well-formed output from `svd2rust`. The second argument is the path to our PAC crate. The final argument, `adc1`, is the module we're interested in importing. The tool accepts more than one module, which may be useful to bulk-add peripheral crates.

A successful run should generate a new crate, `imxrt1062-pac/imxrt1062-adc1`, with all sources available in `src`. The `Cargo.toml` should have all required dependencies:

```
[dependencies]
bare-metal = "0.2.5"
cortex-m = "0.6.1"
vcell = "0.1.2"

[lib]
bench = false
test = false
```

The tool may also specify that there are no tests and benchmarks in the crate. This is to prevent RLS from complaining about missing `test` or `bench` crates.

Once the crate is available, follow these manual steps:

- add the peripheral crate to the top-level Cargo.toml workspace:

```toml
# Top-level Cargo.toml
[workspace]
members = [
    # Existing members...
    "imxrt1062-pac/imxrt1062-adc1",
    # ...
]
```

- add the peripheral crate as a dependency of the `imxrt1062-pac` crate:

```toml
# imxrt1062/Cargo.toml
[dependencies]
# Existing deps...
imxrt1062-adc1 = { path = "imxrt1062-adc1" }
# ...
```

- re-export the crate from `imxrt1062-pac`, removing the `imxrt1062-` prefix:

```rust
// imxrt1062-pac/src/lib.rs
pub use imxrt1062_adc1 as adc1;
```

- un-comment any related code in `imxrt1062-pac/src/lib.rs` so that the peripheral will be exported by the top-level `Peripherals` struct.

After following the above procedure, the `adc1` module should be accessible from the `imxrt1062-pac` crate.

If you notice any issues with this approach, or have suggestions for better approaches, please let us know!