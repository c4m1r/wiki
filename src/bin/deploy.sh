#!/bin/bash
# Deploy script for GitHub Pages

set -e

# Navigate to project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
cd "$PROJECT_ROOT"

echo "Deploying to GitHub Pages..."

# Build the project
./src/bin/build.sh

# Copy book contents to root for GitHub Pages
echo "Copying build output to root..."
if [ -d "book" ]; then
    cp -r book/* ./
else
    echo "Error: Book folder not found."
    exit 1
fi

# Remove book folder since we copied its contents
rm -rf book

echo "Deployment completed!"
echo "Files are now ready for GitHub Pages."
echo "Commit and push to deploy."
