### Tiny-PROTOCOL

PC <- tiny-protocol -> devices

tiny-protocol = mqtt server + http static file server + mdns-sd service discovery

mqtt server used [rmqtt](https://github.com/rmqtt/rmqtt), the [$sys System topic](https://github.com/rmqtt/rmqtt/blob/master/docs/en_US/sys-topic.md) you may need.

mdns-sd used [mdns-sd](https://github.com/keepsimple1/mdns-sd).

### how to use it with desktop software

It provides C header file and dynamic library, the [CmakeLists.txt](./CMakeLists.txt) is an example to use it in Windows environment.


### TODO:
1. -[ ] plugin config read from string rather than dir. https://github.com/rmqtt/rmqtt/issues/196

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

## Building and Using `deskmsg-napi` (for Node.js Projects)

The `deskmsg-napi` module provides Node.js bindings for the `deskmsg_c` functionality, allowing you to use it from JavaScript or TypeScript.

**Prerequisites:**
*   Rust (latest stable, with Cargo)
*   Node.js (version 10 or higher recommended)
*   NPM or Yarn
*   (On some systems) Python and build tools (like `windows-build-tools` on Windows) might be required for `node-gyp` or N-API build processes, though `@napi-rs/cli` aims to minimize these external dependencies.

**Building `deskmsg-napi`:**
1.  Navigate to the `deskmsg_napi` directory.
2.  Install Node.js dependencies: `npm install` (or `yarn install`).
3.  Build the N-API module:
    *   For a release build: `npm run build`
    *   For a debug build: `npm run build:debug`
    This will compile the Rust code into a `.node` native addon file.

**Using `deskmsg-napi` in your Node.js project:**
1.  Add `deskmsg-napi` as a dependency to your project.
    *   If `deskmsg-napi` is published to a registry: `npm install deskmsg-napi`
    *   For local development, you can use `npm link` or point to the local path:
        In `deskmsg_napi` directory: `npm link`
        In your project: `npm link deskmsg-napi`
    *   Alternatively, `npm install path/to/deskmsg_napi`.
2.  Require and use the module in your JavaScript/TypeScript code:
    ```javascript
    const { startServer, getConfig, discovery, syncTest } = require('deskmsg-napi');
    
    async function main() {
      const config = { mqtt_address: '0.0.0.0:1883', http_address: '0.0.0.0:8080', basic_path: '/mqtt' };
      try {
        await startServer(JSON.stringify(config));
        console.log('Server started!');
        const currentCfg = await getConfig();
        console.log('Current Config:', currentCfg);
        // ... use other functions
      } catch (e) {
        console.error('Error:', e.message);
      }
    }
    main();
    ```
3.  Refer to `deskmsg_napi/test.js` for more detailed example usage.

**TypeScript:**
*   The `deskmsg-napi` package is configured to generate TypeScript type definitions (`index.d.ts`) when built. This provides type safety and autocompletion when using the module in TypeScript projects.
