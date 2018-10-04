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
SHA256=$(shasum -a 256 $FILENAME | cut -d " " -f 1)

FORMULA=$(cat <<EOM
class Kubesecrets < Formula
  desc "Tool to manage secrets in Kubernetes with kubectl"
  homepage "https://github.com/atkinchris/kubesecrets"
  url "https://github.com/atkinchris/kubesecrets/releases/download/v$VERSION/kubesecrets-$VERSION.tar.gz"
  sha256 "$SHA256"

  bottle :unneeded

  depends_on "kubernetes-cli"

  def install
    bin.install "kubesecrets"
  end

  test do
    system "#{bin}/kubesecrets", "--version"
  end
end
EOM
)

echo "$FORMULA" > releases/kubesecrets.rb

git tag --force "v${VERSION}"
git push --tags
