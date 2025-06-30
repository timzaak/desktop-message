cmake_minimum_required(VERSION 3.31)

project(c_example LANGUAGES C)

#if(CMAKE_BUILD_TYPE STREQUAL "Debug")
#    set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../target/debug")
#elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
#    set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../target/release")
#endif()
# TODO: change it
set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/release")
set(RUST_LIB_NAME "deskmsg_c")
include_directories("${CMAKE_SOURCE_DIR}/../../${RUST_LIB_NAME}/include")

if(WIN32)
    set(RUST_LINK_LIB ${RUST_LIB_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}.lib)
else()
    set(RUST_LINK_LIB ${RUST_LIB_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX})
endif()


set(SOURCES c_example.c)
add_executable(c_example ${SOURCES})

target_link_libraries(c_example PRIVATE ${RUST_LINK_LIB})


if(WIN32)
    add_custom_command(TARGET c_example POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${RUST_LIB_DIR}/${CMAKE_SHARED_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}
            $<TARGET_FILE_DIR:c_example>
    )
endif()