# iMXRT1062 Peripheral Access Crates

The top-level peripheral access crate (PAC), `imxrt1062-pac`, re-exports all processor peripheral crates. The code in these crates is auto-generated using `svd2rust`, then migrated into this file structure using some semi-automated tooling (available in the top-level `tools` directory). 

If you would like to build your own hardware-abstraction layer for the iMXRT106x, consider including the `imxrt1062-pac` as a dependency. If you'd like to create a very specialized HAL, consider selecting the peripherals of interest as your dependencies.

Unlike most of the crates in this directory, the `imxrt1062-core` crate does not represent a peripheral. Instead, it defines the interrupt table, interrupt names, and interrupt numbers. If you're building your own HAL, you may want this crate to automatically configure the interrupt table.

The rest of the README describes how you might add a new iMXRT1062 peripheral into the PAC. The audience for this walkthrough is an embedded Rust developer who is interested in patching an existing PAC crate with changes made in the SVD file.

## Requirements

Consult the `README.md` in the repository's `svd` directory to generate the default iMXRT1062 PAC.

## Adding a peripheral crate

To simplify the example, let's add the ADC peripheral (specified as `adc1` in the `svd2rust` output) to our PAC. Assuming we're at the root of this repository, use the binary available at `tools/main.rs` to copy the peripheral module into its own crate.

```bash
$ cd tools
$ cargo run  --target x86_64-apple-darwin -- ../svd/imxrt1062 adc1
```

In the example snippet above, ensure that the `--target` flag is set for your host system. Specifying the `--target` may be necessary because the repository workspace is configured for `thumbv7em-none-eabihf`. Consider adding a Cargo configuration file in `tools/.cargo/config` to elide this target specifier. The first argument, `../svd/imxrt1062`, is the well-formed output from `svd2rust`. The second argument, `adc1`, is the module we're interested in importing. The tool accepts more than one module, which may be useful to bulk-add peripheral crates.

A successful run should generate a new crate, `imxrt1062-pac/imxrt1062-adc1`, with all sources available in `src`. The `Cargo.toml` should have all required dependencies:

```toml
[dependencies]
vcell = "0.1.2"

[lib]
bench = false
test = false
```

The tool may also specify that there are no tests and benchmarks in the crate. This is to prevent RLS from complaining about missing `test` or `bench` crates.

If you notice any issues with this approach, or have suggestions for better approaches, please let us know!