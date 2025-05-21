extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_cpp_compat(true)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// don't change this, it auto generated")
        .with_include_guard("TINY_PROTOCOL")
        .with_no_includes()
        .with_pragma_once(true)
        .with_item_prefix("tiny_protocol_")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/tiny-protocol.h");
}