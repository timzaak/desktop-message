cmake_minimum_required(VERSION 3.31)

project(tiny_rmqtt_example LANGUAGES C)


include_directories(lib/include)

set(SOURCES deskmsg_c/examples/c_example.c)


add_executable(tiny_rmqtt_example ${SOURCES})

target_link_libraries(tiny_rmqtt_example PRIVATE
        ${CMAKE_SOURCE_DIR}/target/release/deskmsg_c.dll.lib

)


if(WIN32)
    add_custom_command(TARGET tiny_rmqtt_example POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            ${CMAKE_SOURCE_DIR}/target/release/deskmsg_c.dll
            $<TARGET_FILE_DIR:tiny_rmqtt_example>
    )
endif()