#!/bin/bash

# Build script for Filler AI Player

echo "Building Filler AI Player..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Error: Cargo is not installed. Please install Rust."
    exit 1
fi

# Build in release mode for optimal performance
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Binary location: target/release/filler"
    
    # Copy binary to solution root for easier access
    cp target/release/filler ./filler
    echo "Binary copied to: ./filler"
    
    # Make it executable
    chmod +x ./filler
    
    echo ""
    echo "You can now run the player with:"
    echo "./game_engine -f maps/map01 -p1 solution/filler -p2 robots/bender"
else
    echo "Build failed!"
    exit 1
fi
