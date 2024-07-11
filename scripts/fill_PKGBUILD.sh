#!/bin/bash

# Extract version from Cargo.toml
version=$(cat ./Cargo.toml | grep version | head -1 | awk -F \" '{print $2}')

# Check version
if [ -z "$version" ]; then
  echo "Extracting version failed."
  exit 1
fi

# Modify pkgver in PKGBUILD
sed -i "s/^pkgver=.*/pkgver=$version/" ./aur/PKGBUILD

echo "PKGBUILD is modified, new version is $version ."
