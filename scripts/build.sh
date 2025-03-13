#!/bin/bash

PLATFORMS=(
  "aarch64-apple-darwin:macos-apple-silicon"
  "x86_64-pc-windows-gnu:windows"
)

PROJECT_DIR=$(git rev-parse --show-toplevel)
PROJECT_NAME=$(bash $PROJECT_DIR/scripts/project-name.sh)

# Create a build directory if it doesn't exist
BUILD_DIR="$(git rev-parse --show-toplevel)/build"
mkdir -p $BUILD_DIR

for PLATFORM_PAIR in "${PLATFORMS[@]}"; do
  PLATFORM=$(echo "$PLATFORM_PAIR" | cut -d: -f1)
  LABEL=$(echo "$PLATFORM_PAIR" | cut -d: -f2)
  echo "Building for platform: $LABEL ($PLATFORM)..."

  cargo build --release --target $PLATFORM

  if [[ $? -ne 0 ]]; then
    echo "Failed to build for platform: $LABEL ($PLATFORM)"
    exit 1
  fi

  # Determine the executable name and extension
  EXECUTABLE_NAME="$PROJECT_NAME"
  if [[ $PLATFORM == *"windows"* ]]; then
    EXECUTABLE_NAME="$EXECUTABLE_NAME.exe"
  fi

  # Zip the executable
  ZIP_FILE="$BUILD_DIR/${LABEL}.zip"
  zip -j $ZIP_FILE $PROJECT_DIR/target/$PLATFORM/release/$EXECUTABLE_NAME

  if [[ $? -ne 0 ]]; then
    echo "Failed to build for platform: $LABEL ($PLATFORM)"
    exit 1
  fi

  echo "Successfully built and zipped for platform: $LABEL ($PLATFORM)"
done