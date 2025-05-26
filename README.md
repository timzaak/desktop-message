### Tiny-PROTOCOL

PC <- tiny-protocol -> devices

tiny-protocol = mqtt server + http static file server + mdns-sd or ble service discovery

mqtt server used [rmqtt](https://github.com/rmqtt/rmqtt), the [$sys System topic](https://github.com/rmqtt/rmqtt/blob/master/docs/en_US/sys-topic.md) you may need.

mdns-sd used [mdns-sd](https://github.com/keepsimple1/mdns-sd).

ble used [btleplug](https://github.com/deviceplug/btleplug)

### how to use it with desktop software

It provides C header file and dynamic library, the [CmakeLists.txt](./CMakeLists.txt) is an example to use it in Windows environment.


### TODO:
1. -[ ] plugin config read from string rather than dir. https://github.com/rmqtt/rmqtt/issues/196
2. -[ ] stable ble api

## Building and Using `deskmsg_c` (for C/C++ Projects)

The `deskmsg_c` library provides a C-compatible API for the core Rust functionality. It can be integrated into C/C++ projects using CMake.

**Prerequisites:**
*   Rust (latest stable, with Cargo)
*   CMake (version 3.15 or higher)
*   A C compiler (e.g., GCC, Clang, MSVC)

**Building `deskmsg_c`:**
1.  Navigate to the `deskmsg_c` directory.
2.  Run `cargo build` (for a debug build) or `cargo build --release` (for a release build).
    This will compile the Rust library and generate the C header file (`deskmsg_c/include/deskmsg_c.h`) via `cbindgen`.

**Integrating with a C/C++ CMake Project:**
1.  Ensure `deskmsg_c` has been built with Cargo as described above.
2.  In your main CMake project, you can include `deskmsg_c` as a subdirectory:
    ```cmake
    # In your root CMakeLists.txt
    # Add the path to the deskmsg_c directory
    add_subdirectory(path/to/your/project/deskmsg_c deskmsg_c_build)
    
    # Link your C/C++ target against deskmsg_c
    target_link_libraries(your_cpp_target PRIVATE deskmsg_c)
    ```
3.  The `deskmsg_c/CMakeLists.txt` file defines an `IMPORTED` library target (`deskmsg_c_imported`) and an `INTERFACE` library target (`deskmsg_c`) that your project can link against. This handles include directories and library linkage for different platforms and build configurations.
4.  Refer to `deskmsg_c/examples/c_example.c` and the root `CMakeLists.txt` for an example of how to link and use the library.

**Installation (Optional):**
*   You can install the `deskmsg_c` headers and library files to a system location using CMake. From the build directory of a project that includes `deskmsg_c` (or a standalone build directory for `deskmsg_c` itself if configured): 
    `cmake --build . --target install`
    This requires configuring `CMAKE_INSTALL_PREFIX`.
