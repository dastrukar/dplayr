#!/bin/sh -x

compile:
	@echo "Exporting RUSTFLAGS..."
	@export RUSTFLAGS="-L /usr/lib/x86_64-linux-gnu"
	@echo "Compiling for Linux with musl libs..."
	@cargo build --release --target x86_64-linux-musl
	@echo "Compiling for Windows with msvc..."
	@cargo build --release --target x86_64-windows-msvc
	@echo "Done."
