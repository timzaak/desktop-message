# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch64, release-linux,  release-linux-aarch64, release-windows


release-mac-x86_64:
	cargo build --release  --target=x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/libdeskmg.dylib ./release/

release-mac-aarch64:
	cargo build --release  --target=aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/libdeskmg.dylib ./release/

release-linux-aarch64:
	cargo build --release  --target=aarch64-unknown-linux-gnu
	cp target/aarch64-unknown-linux-gnu/release/libdeskmg.so ./release/


release-linux:
	cargo build --release  --target=x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/libdeskmg.so ./release/


release-windows:
	cargo build --release --target=x86_64-pc-windows-msvc
	cp target/x86_64-pc-windows-msvc/release/libdeskmg.dll ./release/
	cp target/x86_64-pc-windows-msvc/release/libdeskmg.dll.lib ./release/




