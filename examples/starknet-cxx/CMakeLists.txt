cmake_minimum_required(VERSION 3.21)

project(starknet_rs_in_cpp CXX)

include(FetchContent)

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
set(CMAKE_CXX_STANDARD 17)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

add_subdirectory(starknet-cxx)

add_executable(main main.cpp)
target_link_libraries(main PRIVATE starknet_cxx_bridge)
