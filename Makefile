TARGET = aarch64-unknown-none

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) linkscript.ld

XRC = cargo xrustc -j3 --target=$(TARGET) --release

CARGO_OUTPUT = target/$(TARGET)/release/kernel

OBJCOPY = cargo objcopy --
OBJCOPY_PARAMS = --strip-all -O binary

RUNCMD = qemu-system-aarch64 -M raspi3  -serial stdio -kernel kernel.img

.PHONY: all qemu clippy clean objdump nm

all: clean kernel.img

$(CARGO_OUTPUT): $(SOURCES)
	$(XRC)

kernel.img: $(CARGO_OUTPUT)
	cp $< .
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< kernel.img

qemu: all
	$(RUNCMD) -d in_asm

clippy: 
	cargo xclippy --target=$(TARGET)

clean:
	cargo clean

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex kernel

nm:
	cargo nm --target $(TARGET) -- kernel | sort