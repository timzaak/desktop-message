# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch64, release-linux,  release-linux-aarch64, release-windows


release-mac-x86_64:
	cargo build --release  --target=x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/libdeskmsg_c.dylib ./release/

release-mac-aarch64:
	cargo build --release  --target=aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/libdeskmsg_c.dylib ./release/

release-linux-aarch64:
	cargo build --release  --target=aarch64-unknown-linux-gnu
	cp target/aarch64-unknown-linux-gnu/release/libdeskmsg_c.so ./release/


release-linux:
	cargo build --release  --target=x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/libdeskmsg_c.so ./release/


release-windows:
	cargo build --release --target=x86_64-pc-windows-msvc
	cp target/x86_64-pc-windows-msvc/release/libdeskmsg_c.dll ./release/
	cp target/x86_64-pc-windows-msvc/release/libdeskmsg_c.dll.lib ./release/




