GDB ?= arm-none-eabi-gdb
BIN ?= anne-key

all:
	$(MAKE) dfu

components:
	rustup component add llvm-tools-preview rustfmt clippy
	rustup target add thumbv7m-none-eabi

dfu: components
	cargo objcopy --bin $(BIN) --release -- -O binary $(BIN).bin
	./scripts/dfu-convert.py -b 0x08004000:$(BIN).bin $(BIN).dfu
	ls -l $(BIN).dfu

debug: components
	cargo build --release --features use_semihosting --bin $(BIN)
	$(GDB) -x openocd.gdb target/thumbv7m-none-eabi/release/$(BIN)

gui-debug: components
	cargo build --release --features use_semihosting --bin $(BIN)
	gdbgui --gdb $(GDB) --gdb-args "-x openocd.gdb" target/thumbv7m-none-eabi/release/$(BIN)

bloat: components
	cargo bloat $(BLOAT_ARGS) -n 50 --target thumbv7m-none-eabi --bin $(BIN)

fmt: components
	cargo fmt

clippy: components
	cargo clippy

clean:
	cargo clean
	rm -f $(BIN).bin
	rm -f $(BIN).dfu
	rm -rf _book/

.PHONY: all clean debug bloat fmt clippy
