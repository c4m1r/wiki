#!/bin/bash
# Clean build artifacts

set -e

echo "Cleaning build artifacts..."

# Remove the book folder
if [ -d "book" ]; then
    echo "Removing book folder..."
    rm -rf "book"
fi

# Remove Rust build artifacts
if [ -d "target" ]; then
    echo "Removing Rust build artifacts..."
    rm -rf "target"
fi

if [ -d "src/target" ]; then
    echo "Removing Rust build artifacts..."
    rm -rf "src/target"
fi

echo "Clean completed!"
