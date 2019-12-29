## MIMXRT1062 SVD

This is the SVD file for the MIMXRT1062. The original SVD is available
in the CMSIS Pack [here](https://developer.arm.com/embedded/cmsis/cmsis-packs/devices/NXP/MIMXRT1062XXXXA).

This SVD has a few custom modifications:

- It was reformatted using the provided `format.sh` script.
- We implemented a `<cluster>` to represent each PWM submodule's register. Before that change, there were four independent submodules for each PWM module. Each submodule had its own name. When represented with a `<cluster>`, the auto-generated Rust code is nicer to work with.

## Generate the PAC supercrate

Run the script `superpac.sh` to generate the super PAC crate. The modules of the super PAC crate may then be imported to our `imxrt1062-pac` crate as independent crates. See the README.md in the `imxrt1062-pac` directory for more information on the import process. Ensure that both `svd2rust` and `form` are installed:

```
cargo install svd2rust
cargo install form
```

The super PAC is excluded from this project's version control system.