#!/bin/bash

# Flutter Lazy Installation Script
# This script builds and installs the Flutter Lazy tool

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to display error and exit
error_exit() {
    echo -e "${RED}ERROR: $1${NC}" >&2
    exit 1
}

# Function to display a step
display_step() {
    echo -e "\n${BLUE}${BOLD}$1${NC}"
}

# Check dependencies
display_step "Checking dependencies..."

if ! command_exists cargo; then
    error_exit "Rust not installed. Please install Rust from https://rustup.rs/"
fi

if ! command_exists flutter; then
    echo -e "${YELLOW}WARNING: Flutter not found. The generator requires Flutter to be installed to create projects.${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Installation aborted."
        exit 0
    fi
fi

# Navigate to project directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR" || error_exit "Failed to change directory to $SCRIPT_DIR"

display_step "Building Flutter Lazy..."

# Clean previous build if it exists
if [[ -d "target/release" ]]; then
    echo "Cleaning previous build..."
    cargo clean --release
fi

# Build the project
echo "Building Rust project..."
cargo build --release || error_exit "Failed to build the project. Please check the error messages above."

# Verify the binary was created
if [[ ! -f "target/release/flutter_lazy" ]]; then
    error_exit "Build completed but binary not found."
fi

# Copy templates to target directory
display_step "Setting up templates..."
echo "Copying templates..."
mkdir -p target/release/templates || error_exit "Failed to create templates directory"
cp -r templates/* target/release/templates/ || error_exit "Failed to copy template files"

echo -e "${GREEN}Build complete!${NC}"

# Determine installation directory
INSTALL_DIR="/usr/local/bin"
if [[ ! -w "$INSTALL_DIR" ]]; then
    # If /usr/local/bin is not writable, offer $HOME/.local/bin as alternative
    ALT_INSTALL_DIR="$HOME/.local/bin"
    
    # Create directory if it doesn't exist
    if [[ ! -d "$ALT_INSTALL_DIR" ]]; then
        mkdir -p "$ALT_INSTALL_DIR"
    fi
    
    # Ask which directory to use
    echo -e "\n${YELLOW}NOTE: $INSTALL_DIR requires sudo access.${NC}"
    echo "1) Install to $INSTALL_DIR (requires sudo)"
    echo "2) Install to $ALT_INSTALL_DIR"
    echo "3) Skip installation"
    
    read -p "Select an option (1-3): " -r choice
    case $choice in
        1)
            # Global installation (requires sudo)
            display_step "Installing to $INSTALL_DIR..."
            sudo cp target/release/flutter_lazy "$INSTALL_DIR/" || error_exit "Failed to install to $INSTALL_DIR"
            echo -e "${GREEN}Installation complete!${NC}"
            echo "You can now use the generator by running: flutter_lazy"
            ;;
        2)
            # User directory installation
            display_step "Installing to $ALT_INSTALL_DIR..."
            cp target/release/flutter_lazy "$ALT_INSTALL_DIR/" || error_exit "Failed to install to $ALT_INSTALL_DIR"
            
            # Check if the directory is in PATH
            if [[ ":$PATH:" != *":$ALT_INSTALL_DIR:"* ]]; then
                echo -e "${YELLOW}NOTE: $ALT_INSTALL_DIR is not in your PATH.${NC}"
                echo "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
                echo -e "${BOLD}export PATH=\"\$PATH:$ALT_INSTALL_DIR\"${NC}"
                echo ""
                echo "Then restart your terminal or run:"
                echo -e "${BOLD}source ~/.$(basename "$SHELL")rc${NC}"
            fi
            
            echo -e "${GREEN}Installation complete!${NC}"
            echo "You can now use the generator by running: flutter_lazy"
            ;;
        3)
            echo -e "${YELLOW}Installation skipped.${NC}"
            echo "You can run the generator manually from: $SCRIPT_DIR/target/release/flutter_lazy"
            ;;
        *)
            echo -e "${YELLOW}Invalid option. Installation skipped.${NC}"
            echo "You can run the generator manually from: $SCRIPT_DIR/target/release/flutter_lazy"
            ;;
    esac
else
    # Ask if user wants to install
    read -p "Do you want to install the generator to $INSTALL_DIR? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        display_step "Installing to $INSTALL_DIR..."
        cp target/release/flutter_lazy "$INSTALL_DIR/" || error_exit "Failed to install to $INSTALL_DIR"
        echo -e "${GREEN}Installation complete!${NC}"
        echo "You can now use the generator by running: flutter_lazy"
    else
        echo -e "${YELLOW}Installation skipped.${NC}"
        echo "You can run the generator manually from: $SCRIPT_DIR/target/release/flutter_lazy"
    fi
fi

display_step "Next steps:"
echo "1. Run 'flutter_lazy new' to create a new Flutter project"
echo "2. Follow the interactive prompts to configure your project"
echo "3. Check the README for more advanced usage options"
echo -e "\n${GREEN}Thank you for installing Flutter Lazy!${NC}"
