# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch64, release-linux,  release-linux-aarch64, release-windows, prepare-linux, prepare-mac-x86_64,prepare-mac-aarch64,prepare-windows

prepare-linux:
	sudo apt-get install -y libdbus-1-dev pkg-config
	rustup target add x86_64-unknown-linux-gnu
prepare-mac-x86_64:
	brew install dbus #pkg-config
	rustup target add x86_64-apple-darwin
prepare-mac-aarch64:
	brew install dbus #pkg-config
	rustup target add aarch64-apple-darwin
prepare-windows:
	vcpkg install dbus
	rustup target add x86_64-pc-windows-msvc


release-mac-x86_64:
	mkdir -p ./release/lib
	mkdir -p ./release/include
	cargo build --release --target=x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/libdeskmsg_c.dylib ./release/lib/
	cp deskmsg_c/include/deskmsg_c.h ./release/include/

release-mac-aarch64:
	mkdir -p ./release/lib
	mkdir -p ./release/include
	cargo build --release  --target=aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/libdeskmsg_c.dylib ./release/lib/
	cp deskmsg_c/include/deskmsg_c.h ./release/include/

release-linux-aarch64:
	mkdir -p ./release/lib
	mkdir -p ./release/include
	cargo build --release --target=aarch64-unknown-linux-gnu
	cp target/aarch64-unknown-linux-gnu/release/libdeskmsg_c.so ./release/lib/
	cp deskmsg_c/include/deskmsg_c.h ./release/include/


release-linux:
	mkdir -p ./release/lib
	mkdir -p ./release/include
	cargo build --release --target=x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/libdeskmsg_c.so ./release/lib/
	cp deskmsg_c/include/deskmsg_c.h ./release/include/


release-windows:
	mkdir -p ./release/lib
	mkdir -p ./release/include
	cargo build --release --target=x86_64-pc-windows-msvc
	cp target/x86_64-pc-windows-msvc/release/deskmsg_c.dll ./release/lib/
	cp target/x86_64-pc-windows-msvc/release/deskmsg_c.dll.lib ./release/lib/
	cp target/x86_64-pc-windows-msvc/release/deskmsg_c.pdb ./release/lib/
	cp deskmsg_c/include/deskmsg_c.h ./release/include/


