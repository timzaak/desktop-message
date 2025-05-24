cmake_minimum_required(VERSION 3.15)
project(DeskMSGProject LANGUAGES C)

set(CMAKE_C_STANDARD 11)

# Add deskmsg_c as a subdirectory. 
# This will execute deskmsg_c/CMakeLists.txt and define the 'deskmsg_c' target.
add_subdirectory(deskmsg_c)

# --- Build the C example --- 
# It's good practice to put examples inside an if(BUILD_TESTING) or similar, 
# but for now, we'll include it directly.

set(C_EXAMPLE_SOURCES deskmsg_c/examples/c_example.c)

# Ensure the header from deskmsg_c can be found.
# The deskmsg_c target (INTERFACE library) should provide this via INTERFACE_INCLUDE_DIRECTORIES.
# So, direct include_directories might not be strictly needed here if deskmsg_c is linked.

add_executable(tiny_rmqtt_example ${C_EXAMPLE_SOURCES})

# Link the example against the deskmsg_c library target.
# The 'deskmsg_c' target is the INTERFACE library defined in deskmsg_c/CMakeLists.txt,
# which in turn links to the deskmsg_c_imported target.
target_link_libraries(tiny_rmqtt_example PRIVATE deskmsg_c)

message(STATUS "Root CMakeLists.txt: Configured to build 'tiny_rmqtt_example' linking against 'deskmsg_c' library.")

# --- DLL Copy for Windows (if needed for running the example directly from build dir) ---
# The IMPORTED library target in deskmsg_c/CMakeLists.txt handles the DLL's location for linking.
# For running the executable, the DLL needs to be in the same directory or in PATH.
# CMake's standard install rules (in deskmsg_c/CMakeLists.txt) will place the DLL correctly upon installation.
# For convenience during development, you might still want to copy the DLL to the example's output directory.

if(WIN32)
    # Define a helper function/macro to get the DLL path from the imported target
    # This is a bit advanced; a simpler way is to reconstruct the path like before,
    # but using the CARGO_TARGET_DIR variable if it's globally available or passed down.
    # For now, let's assume the DLL is found via deskmsg_c properties if possible,
    # or reconstruct the path to the DLL from where Cargo builds it.

    # Option 1: Reconstruct path (simpler if CARGO_TARGET_DIR is known)
    # Ensure CARGO_TARGET_DIR is available (e.g., set as a CACHE variable from deskmsg_c/CMakeLists.txt)
    # Or, for simplicity in this step, assume a relative path from the build directory of the example.
    # This assumes the example executable is in build/ and the DLL is in build/deskmsg_c/
    # A more robust way is to use generator expressions with target_file_location.
    
    # Get the location of the DLL from the imported target if possible (for Release)
    # This is tricky because the actual file location isn't directly available as a simple variable 
    # from an IMPORTED target without knowing the configuration beforehand.
    # However, since deskmsg_c/CMakeLists.txt uses CARGO_TARGET_DIR, we can reuse that concept.
    get_filename_component(CARGO_TARGET_DIR_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/target" ABSOLUTE)

    # We need to copy the DLL for the specific configuration being built.
    # For single-config generators:
    if (CMAKE_BUILD_TYPE STREQUAL "Debug")
        set(DESKMSG_DLL_FILE "${CARGO_TARGET_DIR_ROOT}/debug/deskmsg_c.dll")
    else() # Release or other
        set(DESKMSG_DLL_FILE "${CARGO_TARGET_DIR_ROOT}/release/deskmsg_c.dll")
    endif()

    # For multi-config generators (like Visual Studio), we'd use a generator expression.
    # $<IF:$<CONFIG:Debug>,${CARGO_TARGET_DIR_ROOT}/debug/deskmsg_c.dll,${CARGO_TARGET_DIR_ROOT}/release/deskmsg_c.dll>
    # However, add_custom_command doesn't always play nicely with generator expressions in its COMMAND argument directly.

    # A common approach for custom commands:
    add_custom_command(TARGET tiny_rmqtt_example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "$<$<IF:$<CONFIG:Debug>,${CARGO_TARGET_DIR_ROOT}/debug/deskmsg_c.dll,${CARGO_TARGET_DIR_ROOT}/release/deskmsg_c.dll>" # Genex for path
            $<TARGET_FILE_DIR:tiny_rmqtt_example> # Destination directory
        COMMENT "Copying deskmsg_c.dll to example directory for configuration $<CONFIG>"
    )
endif()

# --- Optional: Add tests using CTest ---
# enable_testing()
# add_test(NAME RunExample COMMAND tiny_rmqtt_example)