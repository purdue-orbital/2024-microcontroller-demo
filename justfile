alias b := build
alias u := uf2

check:
	cargo check

# install the nesicary tools
setup:
	rustup target install thumbv6m-none-eabi
	cargo install elf2uf2-rs
	cargo install cargo-binstall
	cargo binstall probe-rs-tools

clippy:
	cargo clippy

build:
	cargo build

build_release:
	cargo build --release

# build release and convert to uf2
uf2: build_release
	elf2uf2-rs target/thumbv6m-none-eabi/release/micro_demo micro_demo.uf2

clean:
	cargo clean
	rm micro_demo.uf2