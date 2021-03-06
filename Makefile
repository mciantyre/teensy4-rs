RUSTUP ?= rustup
CARGO ?= cargo
TEENSY_LOADER ?= teensy_loader_cli
INSTALL_DEPS ?= 1
HOST ?= $(shell rustc --version --verbose | grep host | cut -d ' ' -f 2)

ifneq ($(INSTALL_DEPS),0)
# Ensure the thumbv7em-none-eabihf component is installed
THUMBV7EM_NONE_EABIHF_INSTALLED := $(shell $(RUSTUP) component list | grep 'rust-std-thumbv7em-none-eabihf.*(installed)' > /dev/null; echo $$?)
ifeq ($(THUMBV7EM_NONE_EABIHF_INSTALLED), 1)
  $(shell $(RUSTUP) target add thumbv7em-none-eabihf)
endif

# Ensure llvm-tools-preview is installed
LLVM_TOOLS_INSTALLED := $(shell $(RUSTUP) component list | grep 'llvm-tools-preview.*(installed)' > /dev/null; echo $$?)
ifeq ($(LLVM_TOOLS_INSTALLED),1)
  $(shell $(RUSTUP) component add llvm-tools-preview)
endif

# Ensure cargo binutils are installed
CARGO_BINUTILS_INSTALLED := $(shell $(CARGO) install --list | grep 'cargo-binutils' >/dev/null; echo $$?)
ifeq ($(CARGO_BINUTILS_INSTALLED),1)
  $(shell $(CARGO) install cargo-binutils)
endif

# Use the Teensy command-line loader, if it's available. Otherwise, let the
# user know where the hexfile is by printing out the path.
TEENSY_LOADER_INSTALLED := $(shell which $(TEENSY_LOADER) >/dev/null; echo $$?)
ifeq ($(TEENSY_LOADER_INSTALLED),0)
  LOADER := $(shell which $(TEENSY_LOADER)) -v -w --mcu=TEENSY40
else
  LOADER := echo
endif
endif # INSTALL_DEPS != 0

BSP_EXAMPLES := $(shell find examples/bsp/src/bin -maxdepth 1 -not -type d | xargs basename | cut -f 1 -d .)
RTIC_EXAMPLES := $(shell find examples/rtic/src/bin -maxdepth 1 -not -type d | xargs basename | cut -f 1 -d .)

.PHONY: all bsp rtic
all: bsp rtic

bsp:
	@cargo build --release --workspace
	@for example in $(BSP_EXAMPLES);\
		do rust-objcopy -O ihex target/thumbv7em-none-eabihf/release/$$example target/thumbv7em-none-eabihf/$$example.hex;\
		done
	
rtic:
	@cargo build --release --manifest-path examples/rtic/Cargo.toml
	@for example in $(RTIC_EXAMPLES);\
		do rust-objcopy -O ihex examples/rtic/target/thumbv7em-none-eabihf/release/$$example target/thumbv7em-none-eabihf/$$example.hex;\
		done

libt4start:
	@make -C bin libt4start.a

libt4usb:
	@make -C bin libt4usb.a

.PHONY: clean
clean:
	@make -C bin clean
	@cargo clean
	@cargo clean --manifest-path examples/rtic/Cargo.toml

# Skipping the USB feature testing
#
# We can't link the t4usb library when testing on our host, since
# it's compiled for a different architecture. The documentation tests
# still work.
.PHONY: test
test:
	@cargo +nightly test --lib --tests --target $(HOST)
	@cargo +nightly test --doc --target $(HOST) --all-features

	@cargo +nightly test --manifest-path teensy4-pins/Cargo.toml --lib --tests --target $(HOST) --all-features
	@cargo +nightly test --manifest-path teensy4-pins/Cargo.toml --doc --target $(HOST) --all-features
