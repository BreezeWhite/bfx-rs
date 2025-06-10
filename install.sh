#!/usr/bin/env bash

# Set variables
REPO="BreezeWhite/bfx-rs" # <-- Replace with actual repo, e.g. kohara/lendbot
VERSION="latest" # or specify a version/tag
BINARY_LINUX="bfx-linux"
BINARY_MAC="bfx-mac"

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)   BINARY="$BINARY_LINUX";;
    Darwin*)  BINARY="$BINARY_MAC";;
    *)        echo "Unsupported OS: $OS"; exit 1;;
esac

# Get latest release tag if VERSION is 'latest'
if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')
fi

# Download URL
URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY"

# Find a writable directory in $PATH
IFS=':' read -ra DIRS <<< "$PATH"
for dir in "${DIRS[@]}"; do
    if [ -w "$dir" ]; then
        INSTALL_DIR="$dir"
        break
    fi
done

if [ -z "$INSTALL_DIR" ]; then
    echo "No writable directory found in \$PATH. Try running as sudo or add a writable directory to your \$PATH."
    exit 1
fi

# Download and install
TMPFILE=$(mktemp)
echo "Downloading $BINARY from $URL..."
curl -L "$URL" -o "$TMPFILE"
chmod +x "$TMPFILE"
mv "$TMPFILE" "$INSTALL_DIR/bfx"
echo "Installed 'bfx' to $INSTALL_DIR"
