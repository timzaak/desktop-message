# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch64, release-linux,  release-linux-aarch64, release-windows


release-mac-x86_64:
	cargo build --release  --target=x86_64-apple-darwin
	cp client/target/x86_64-apple-darwin/release/lib.dylib ./release/

release-mac-aarch64:
	cargo build --release  --target=aarch64-apple-darwin
	cp client/target/aarch64-apple-darwin/release/lib.dylib ./release/

release-linux-aarch64:
	cargo build --release  --target=aarch64-unknown-linux-gnu
	cp client/target/aarch64-unknown-linux-gnu/release/lib.o ./release/


release-linux:
	cargo build --release  --target=x86_64-unknown-linux-gnu
	cp client/target/x86_64-unknown-linux-gnu/release/lib.o ./release/


release-windows:
	cargo build --release --target=stable-x86_64-pc-windows-msvc
	cp target/stable-x86_64-pc-windows-msvc/release/lib.dll ./release/
	cp target/stable-x86_64-pc-windows-msvc/release/lib.dll.lib ./release/




