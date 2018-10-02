#!/bin/bash
########################################################
# Script to build and package a release                #
# usage: ./release.sh                                  #
########################################################


# Fail on the first error, rather than continuing
set -e

VERSION=$(cat Cargo.toml | sed -n -e 's/^.*version = "//p'  | sed -e 's/^"//' -e 's/"$//')
FILENAME="releases/kubesecrets-${VERSION}.tar.gz"

cargo build --release
rm -rf releases
mkdir -p releases
tar -czf $FILENAME --directory=target/release kubesecrets
shasum -a 256 $FILENAME > $FILENAME.shasum

git tag --force "v${VERSION}"
git push --tags
