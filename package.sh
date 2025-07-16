#!/usr/bin/env bash
# -----------------------------------------------------------------------------
# Packaging script for "synchron"
# Supports: .tar.gz, .pkg.tar.xz, .deb, .rpm, Windows .exe SFX, macOS .dmg
# Requires: Ubuntu 24.04, bash, coreutils, tar, gzip, xz-utils, p7zip-full,
#           dpkg-deb, ruby+gem (for fpm), fpm, genisoimage (for .dmg)
# -----------------------------------------------------------------------------
set -euo pipefail
IFS=$'\n\t'

########## CONFIGURATION ##########
# Project name (must match your Cargo.toml [[package]] name)
NAME="synchron"
# Extract version from Cargo.toml
VERSION=$(grep -E '^version\s*=' Cargo.toml \
  | head -1 \
  | sed -E 's/version\s*=\s*"([^"]+)"/\1/')

# Paths
BUILD_DIR="build"
DIST_DIR="dist"
DOC_FILES=(LICENSE README.md README_zh.md)

# List of targets and their architecture labels
declare -A LINUX_TARGETS=(
  [x86_64-unknown-linux-gnu]=amd64
  [i686-unknown-linux-gnu]=i386
  [aarch64-unknown-linux-gnu]=arm64
)
WINDOWS_TARGETS=(x86_64-pc-windows-gnu i686-pc-windows-gnu)
DARWIN_TARGETS=(x86_64-apple-darwin aarch64-apple-darwin)

########## DEPENDENCY CHECK ##########
required_cmds=(tar gzip xz dpkg-deb fpm 7z genisoimage)
for cmd in "${required_cmds[@]}"; do
  if ! command -v "$cmd" &>/dev/null; then
    echo "Error: '$cmd' is not installed." >&2
    echo "  Ubuntu install: sudo apt update && sudo apt install -y tar gzip xz-utils p7zip-full dpkg-dev ruby-dev build-essential genisoimage" >&2
    echo "  Then: sudo gem install --no-document fpm" >&2
    exit 1
  fi
done

########## PREPARE DIRECTORIES ##########
rm -rf "$BUILD_DIR" "$DIST_DIR"
mkdir -p "$BUILD_DIR" "$DIST_DIR"

########## STAGING & PACKAGING ##########
package_staging() {
  local target=$1
  local arch_label=$2
  local stage="${BUILD_DIR}/${NAME}-${VERSION}-${target}"
  local bin_src="target/${target}/release/${NAME}"
  local install_bin="${stage}/usr/local/bin"
  local doc_dir="${stage}/usr/local/share/doc/${NAME}"

  # ensure binary exists
  if [[ ! -x "$bin_src" ]]; then
    echo "Error: binary not found for target $target" >&2
    exit 1
  fi

  # create staging layout
  mkdir -p "$install_bin" "$doc_dir"

  # copy binary and docs
  cp "$bin_src" "$install_bin/"
  for f in "${DOC_FILES[@]}"; do
    cp "$f" "$doc_dir/"
  done

  # 1) .tar.gz
  tar czf "${DIST_DIR}/${NAME}-${VERSION}-${target}.tar.gz" -C "$stage" .

  # 2) .pkg.tar.xz (generic tar.xz)
  tar cJf "${DIST_DIR}/${NAME}-${VERSION}-${target}.pkg.tar.xz" -C "$stage" .

  # 3) .deb via fpm
  fpm -s dir -t deb \
      -n "$NAME" -v "$VERSION" --architecture "$arch_label" \
      --prefix / \
      -C "$stage" .

  # 4) .rpm via fpm
  fpm -s dir -t rpm \
      -n "$NAME" -v "$VERSION" --architecture "$arch_label" \
      --prefix / \
      -C "$stage" .

  echo "Packaged for $target → .tar.gz .pkg.tar.xz .deb .rpm"
}

# Linux targets
for tgt in "${!LINUX_TARGETS[@]}"; do
  package_staging "$tgt" "${LINUX_TARGETS[$tgt]}"
done

########## WINDOWS .exe SFX ##########
# Use 7z SFX to create self-extracting installer
for tgt in "${WINDOWS_TARGETS[@]}"; do
  stage="${BUILD_DIR}/${NAME}-${VERSION}-${tgt}"
  bin_src="target/${tgt}/release/${NAME}.exe"
  if [[ ! -f "$bin_src" ]]; then
    echo "Warning: Windows binary not found for $tgt, skipping .exe" >&2
    continue
  fi
  tmp_archive="${BUILD_DIR}/${NAME}-${VERSION}-${tgt}.7z"
  out_exe="${DIST_DIR}/${NAME}-${VERSION}-${tgt}.exe"
  # repack staging for Windows
  rm -rf "$stage"
  mkdir -p "$stage"
  cp "$bin_src" "$stage/"
  for f in "${DOC_FILES[@]}"; do cp "$f" "$stage/"; done
  pushd "$stage" >/dev/null
    7z a -mx=9 "$tmp_archive" ./*
    # locate 7z SFX module
    SFX_MODULE=$(dpkg -L p7zip-full | grep '7z\.sfx$' | head -1)
    if [[ -z "$SFX_MODULE" ]]; then
      echo "Error: could not find 7z.sfx module" >&2
      exit 1
    fi
    cat "$SFX_MODULE" "$tmp_archive" > "$PWD/../$(basename "$out_exe")"
  popd >/dev/null
  rm "$tmp_archive"
  echo "Packaged Windows SFX: $out_exe"
done

########## macOS .dmg ##########
# Create a HFS image that macOS will mount as a .dmg
for tgt in "${DARWIN_TARGETS[@]}"; do
  bin_src="target/${tgt}/release/${NAME}"
  if [[ ! -x "$bin_src" ]]; then
    echo "Warning: macOS binary not found for $tgt, skipping .dmg" >&2
    continue
  fi
  stage="${BUILD_DIR}/${NAME}-${VERSION}-${tgt}"
  rm -rf "$stage"
  mkdir -p "$stage"/{Applications,Documents}
  cp "$bin_src" "$stage/Applications/"
  for f in "${DOC_FILES[@]}"; do cp "$f" "$stage/Documents/"; done
  out_dmg="${DIST_DIR}/${NAME}-${VERSION}-${tgt}.dmg"
  genisoimage -V "${NAME}_${VERSION}_${tgt}" -D -hfs -no-pad \
    -o "$out_dmg" "$stage"
  echo "Packaged macOS .dmg: $out_dmg"
done

echo "All done. Packages are in ./${DIST_DIR}/"
