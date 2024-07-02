#!/bin/bash

# Path to Cargo.toml
CARGO_TOML="packages/Cargo.toml"
WHISKY_CARGO_TOML="packages/whisky/Cargo.toml"
SIDAN_CSL_RS_CARGO_TOML="packages/sidan-csl-rs/Cargo.toml"
EXAMPLES_CARGO_TOML="packages/whisky-examples/Cargo.toml"

# Extract the current main version
current_version=$(grep '^version = ' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/')

# Split the version into major, minor, and patch
IFS='.' read -r major minor patch <<< "$current_version"

# Increment the patch version
new_patch=$((patch + 1))

# Construct the new version
new_version="$major.$minor.$new_patch"

# Update the version in workspace Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$CARGO_TOML"

# Update the version in whisky Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/sidan-csl-rs = { version = \"=$current_version\"/sidan-csl-rs = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"

# Update the version in sidan-csl-rs Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$SIDAN_CSL_RS_CARGO_TOML"

# Update the version in examples Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$EXAMPLES_CARGO_TOML"
sed -i '' "s/whisky = { version = \"=$current_version\"/whisky = { version = \"=$new_version\"/" "$EXAMPLES_CARGO_TOML"


echo "Version bumped to $new_version for all cargo.toml"