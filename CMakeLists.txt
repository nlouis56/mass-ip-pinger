cmake_minimum_required(VERSION 3.15)

project(ipPinger)

set(CMAKE_CXX_STANDARD 20)

include_directories(include/)

set(SOURCES src/main.cpp src/IP.cpp src/logger.cpp src/scanning.cpp src/threads.cpp src/listTools.cpp)

add_executable(ipPinger ${SOURCES})

add_custom_command(TARGET ipPinger
    POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E remove -f ${CMAKE_BINARY_DIR}/output*
    COMMENT "Deleting old output files"
)

add_custom_command(TARGET ipPinger
    POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E echo "" > ${CMAKE_BINARY_DIR}/debug.log
    COMMENT "Clearing debug.log file"
)

add_custom_command(TARGET ipPinger
    POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E echo "" > ${CMAKE_BINARY_DIR}/error.log
    COMMENT "Clearing error.log file"
)

add_custom_target(ALL)

target_link_libraries(ipPinger pthread nsl)

target_compile_options(ipPinger PRIVATE -Wall -Wextra -Wpedantic)

target_compile_definitions(ipPinger PRIVATE _GNU_SOURCE)

target_compile_features(ipPinger PRIVATE cxx_std_20)
