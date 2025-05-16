# This is for c_example.c test case
cmake_minimum_required(VERSION 3.31)
project(tiny-mqtt LANGUAGES C)

set(CMAKE_C_STANDARD 11)

include_directories(lib/include)

set(SOURCES
        lib/examples/c_example.c
)


add_executable(tiny_mqtt_example ${SOURCES})


target_link_libraries(tiny_rmqtt_example
        PRIVATE ${CMAKE_SOURCE_DIR}/target/release/lib.dll.lib
)


if(WIN32)
    add_custom_command(TARGET tiny_rmqtt_example POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${CMAKE_SOURCE_DIR}/target/release/lib.dll
            $<TARGET_FILE_DIR:tiny_rmqtt_example>
    )
endif()