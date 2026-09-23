#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUST_PROJECT="${PROJECT_ROOT}/rust_modules"
DELIVERABLES_DIR="${PROJECT_ROOT}/deliverables"

CARGO_BINARY_NAME="rust-knapsack"
DELIVERABLE_BASENAME="rust-knapsack"

case "${OSTYPE:-}" in
    msys* | cygwin* | win32*)
        PLATFORM_EXTENSION=".exe"
        ;;
    *)
        PLATFORM_EXTENSION=""
        ;;
esac

SOURCE_EXECUTABLE="${RUST_PROJECT}/target/release/${CARGO_BINARY_NAME}${PLATFORM_EXTENSION}"
DELIVERABLE_NAME="${DELIVERABLE_BASENAME}${PLATFORM_EXTENSION}"
DELIVERABLE_PATH="${DELIVERABLES_DIR}/${DELIVERABLE_NAME}"

echo "Building ${DELIVERABLE_NAME}..."

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Cargo was not found in PATH."
    echo "Install Rust and Cargo before running this script."
    exit 1
fi

if [[ ! -f "${RUST_PROJECT}/Cargo.toml" ]]; then
    echo "Error: Cargo.toml was not found:"
    echo "  ${RUST_PROJECT}/Cargo.toml"
    exit 1
fi

if [[ ! -f "${PROJECT_ROOT}/data.csv" ]]; then
    echo "Error: Embedded CSV source was not found:"
    echo "  ${PROJECT_ROOT}/data.csv"
    exit 1
fi

mkdir -p "${DELIVERABLES_DIR}"

cargo build \
    --manifest-path "${RUST_PROJECT}/Cargo.toml" \
    --release \
    --bin "${CARGO_BINARY_NAME}"

if [[ ! -f "${SOURCE_EXECUTABLE}" ]]; then
    echo "Error: Cargo completed, but the executable was not found:"
    echo "  ${SOURCE_EXECUTABLE}"
    exit 1
fi

cp -f "${SOURCE_EXECUTABLE}" "${DELIVERABLE_PATH}"

if [[ -z "${PLATFORM_EXTENSION}" ]]; then
    chmod +x "${DELIVERABLE_PATH}"
fi

echo
echo "Build completed successfully."
echo "Standalone executable:"
echo "  ${DELIVERABLE_PATH}"