#!/bin/sh -x

compile:
	@echo "Exporting RUSTFLAGS..."
	@export RUSTFLAGS="-L /usr/lib/x86_64-linux-gnu"
	@echo "Compiling for Linux with musl libs..."
	@cargo build --release --target x86_64-unknown-linux-musl
	@echo "Compiling for Windows with gnu libs..."
	@cargo build --release --target x86_64-pc-windows-gnu
	@echo "Done."
