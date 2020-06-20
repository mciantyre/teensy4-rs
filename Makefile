RUSTUP ?= rustup
CARGO ?= cargo
TEENSY_LOADER ?= teensy_loader_cli
MODE ?= --release

# Ensure the thumbv7em-none-eabihf component is installed
THUMBV7EM_NONE_EABIHF_INSTALLED := $(shell $(RUSTUP) component list | grep 'rust-std-thumbv7em-none-eabihf.*(installed)' > /dev/null; echo $$?)
ifeq ($(THUMBV7EM_NONE_EABIHF_INSTALLED), 1)
  $(shell $(RUSTUP) component add thumbv7em-none-eabihf)
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

TARGET_EXAMPLES := target/thumbv7em-none-eabihf/release/examples
EXAMPLES := $(shell ls examples | xargs basename | cut -f 1 -d .)

.PHONY: all
all:
	@cargo build $(MODE) --examples
	@for example in $(EXAMPLES);\
		do cargo objcopy $(MODE) --example $$example \
			-- -O ihex -R .eeprom $(TARGET_EXAMPLES)/$$example.hex;\
		done

.PHONY: example_%
example_%:
	@cargo build \
		$(MODE) --example $(subst example_,,$@)

.PHONY: objcopy_%
objcopy_%: example_%
	@cargo objcopy \
		$(MODE) --example $(subst objcopy_,,$@) \
		-- -O ihex -R .eeprom \
		$(TARGET_EXAMPLES)/$(subst objcopy_,,$@).hex

.PHONY: download_%
download_%: objcopy_%
	@$(LOADER) $(TARGET_EXAMPLES)/$(subst download_,,$@).hex

libt4boot:
	@make -C teensy4-rt/bin

libt4usb:
	@make -C teensy4-usb-sys/bin

.PHONY: clean
clean:
	@cargo clean