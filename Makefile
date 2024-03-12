target := aarch64-unknown-none
mode := debug
kernel := target/$(target)/$(mode)/myos
bin := target/$(target)/$(mode)/kernel.bin

objdump := rust-objdump --arch-name=aarch64
objcopy := rust-objcopy --binary-architecture=aarch64

.PHONY: kernel build clean qemu run env

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview rustfmt
	rustup target add $(target)

kernel:
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

build: $(bin)

clean:
	cargo clean

qemu: build
	qemu-system-aarch64 \
		-machine raspi3b \
		-nographic \
		-serial null \
		-serial mon:stdio \
		-kernel $(bin)
		# -device loader,file=$(bin),addr=0x80000

run: build qemu