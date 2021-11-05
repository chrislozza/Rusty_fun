cmake_minimum_required(VERSION 3.16)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_BUILD_TYPE debug)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_VERBOSE_MAKEFILE OFF)
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fconcepts -fpermissive")

project(PokerHands VERSION 1.0)

set(poker_src ${PROJECT_SOURCE_DIR}/src)

file(GLOB_RECURSE poker_sources RELATIVE ${PROJECT_SOURCE_DIR} ${poker_src}/*.cpp)
file(GLOB_RECURSE poker_headers RELATIVE ${PROJECT_SOURCE_DIR} ${poker_src}/*.h)

add_executable(poker_tests ${poker_sources} ${poker_headers})
