#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Rake Build & Install ===${NC}"

# Check for required tools
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}Error: $1 is not installed${NC}"
        exit 1
    fi
}

echo -e "${YELLOW}Checking dependencies...${NC}"
check_command "cargo"
check_command "cmake"
check_command "g++"

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Create build directory
echo -e "${YELLOW}Creating build directory...${NC}"
if [ ! -d "build" ]; then
    mkdir -p build
fi
cd build

# Run CMake
echo -e "${YELLOW}Running CMake...${NC}"
cmake .. -DCMAKE_BUILD_TYPE=Release

# Build
echo -e "${YELLOW}Building project...${NC}"
cmake --build . -j$(nproc)

# Install
echo -e "${YELLOW}Installing...${NC}"
INSTALL_PREFIX="${INSTALL_PREFIX:-$HOME/.local}"
cmake --install . --prefix "$INSTALL_PREFIX"

# Add to PATH
echo -e "${YELLOW}Setting up PATH...${NC}"
if [[ ":$PATH:" != *":$INSTALL_PREFIX/bin:"* ]]; then
    if [[ -f "$HOME/.bashrc" ]]; then
        echo "export PATH=\"\$PATH:$INSTALL_PREFIX/bin\"" >> "$HOME/.bashrc"
        echo -e "${GREEN}Added to ~/.bashrc${NC}"
    fi
    if [[ -f "$HOME/.zshrc" ]]; then
        echo "export PATH=\"\$PATH:$INSTALL_PREFIX/bin\"" >> "$HOME/.zshrc"
        echo -e "${GREEN}Added to ~/.zshrc${NC}"
    fi
fi

# Copy library
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_FILE="rake_parser/target/release/librake_parser.dylib"
else
    LIB_FILE="rake_parser/target/release/librake_parser.so"
fi

if [ -f "$LIB_FILE" ]; then
    cp "$LIB_FILE" "$INSTALL_PREFIX/lib/" 2>/dev/null || true
fi

echo -e "${GREEN}Installation complete!${NC}"
echo -e "${YELLOW}To start using rake, run:${NC}"
echo -e "  export PATH=\"\$PATH:$INSTALL_PREFIX/bin\""
echo -e "  rake --<section>${NC}"
