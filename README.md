### desktop message

PC <- deskmsg -> devices, support Rust, C, C++ and Node.js.

deskmsg = mqtt server + http static file server + mdns-sd or ble service discovery.

mqtt server used [rmqtt](https://github.com/rmqtt/rmqtt).

mdns-sd used [mdns-sd](https://github.com/keepsimple1/mdns-sd).

ble used [btleplug](https://github.com/deviceplug/btleplug).

### how to use it with desktop software

It provides rust static library, C dynamic library and node.js module.

[c examples](examples/c_example) is an example about C to use it.

[rust examples](bin) is an example about Rust to use it.

[electron example](examples/electron_example) is an example about Electron to use it.

## Building and Using `deskmsg_c` (for C/C++ Projects)

The `deskmsg_c` library provides a C-compatible API for the core Rust functionality. It can be integrated into C/C++
projects using CMake.

**Prerequisites:**

* Rust (latest stable, with Cargo)
* CMake (version 3.15 or higher)
* C compiler (e.g., GCC, Clang, MSVC)

**Building `deskmsg_c`:**

1. Navigate to the `deskmsg_c` directory.
2. Run `cargo build` (for debug build) or `cargo build --release` (for release build).
   This will compile the Rust library and generate the C header file (`deskmsg_c/include/deskmsg_c.h`) and shared
   library via `cbindgen`.

**Integrating with a C/C++ CMake Project:**
Refer to `examples/c_example` of how to link and use the library.

## TODO:

- [ ] Release deskmsg_napi to npm
- [ ] Release deskmsg lib to crates.io
