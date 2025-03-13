#!/bin/bash

# Check for uncommitted changes
if [[ -n $(git status -s) ]]; then
  echo "There are uncommitted changes. Please commit or stash them before releasing."
  exit 1
fi

# Get the latest release tag on the master branch
PREVIOUS_TAG=$(git describe --tags --abbrev=0 --match "v[0-9]*.[0-9]*.[0-9]*" master 2>/dev/null)

if [[ -z $PREVIOUS_TAG ]]; then
  echo "No previous tags found. This appears to be the first release."
else
  echo "Previous tag: $PREVIOUS_TAG"
fi

# Ask the user to input a new release tag
read -p "Enter the new release tag (x.y.z): " NEW_TAG

# Validate the new tag format (basic semver validation)
if ! [[ $NEW_TAG =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Invalid tag format. Please use the semver format (x.y.z)."
  exit 1
fi

# Build the release executables for macOS and Windows platforms
PROJECT_DIR=$(git rev-parse --show-toplevel)
bash $PROJECT_DIR/scripts/build.sh

git tag "v$NEW_TAG"
if [[ $? -ne 0 ]]; then
  echo "Failed to create the new tag."
  exit 1
fi
echo "Tag $NEW_TAG created successfully."

git push origin tag "v$NEW_TAG"