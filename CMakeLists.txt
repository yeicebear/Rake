cmake_minimum_required(VERSION 3.16)
project(rake C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Detect platform
if(APPLE)
    set(RUST_LIB_EXT "dylib")
elseif(WIN32)
    set(RUST_LIB_EXT "dll")
else()
    set(RUST_LIB_EXT "so")
endif()

# Build Rust library
add_custom_target(
    rake_parser_lib ALL
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/rake_parser
    BYPRODUCTS ${CMAKE_CURRENT_SOURCE_DIR}/rake_parser/target/release/librake_parser.${RUST_LIB_EXT}
)

# Find the Rust library
set(RAKE_PARSER_LIB ${CMAKE_CURRENT_SOURCE_DIR}/rake_parser/target/release/librake_parser.${RUST_LIB_EXT})

# Create executable
add_executable(rake
    rake_parser/src/cli.cpp
)

# Link Rust library
target_link_libraries(rake PRIVATE ${RAKE_PARSER_LIB})

# Add dependency
add_dependencies(rake rake_parser_lib)

# Ensure lib directory is in include path
target_include_directories(rake PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/rake_parser/ffi)

# Set RPATH for proper library loading
if(APPLE)
    set_target_properties(rake PROPERTIES
        INSTALL_RPATH "@executable_path/../lib"
        BUILD_RPATH "${CMAKE_CURRENT_SOURCE_DIR}/rake_parser/target/release"
    )
elseif(UNIX)
    set_target_properties(rake PROPERTIES
        INSTALL_RPATH "$ORIGIN/../lib"
        BUILD_RPATH "${CMAKE_CURRENT_SOURCE_DIR}/rake_parser/target/release"
    )
endif()

# Installation
install(TARGETS rake DESTINATION bin)
install(FILES ${RAKE_PARSER_LIB} DESTINATION lib)
install(FILES Rakefile DESTINATION . OPTIONAL)
