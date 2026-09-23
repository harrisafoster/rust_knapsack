#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUST_PROJECT="${PROJECT_ROOT}/rust_modules"
SOURCE_FILE="${PROJECT_ROOT}/rust_connect.py"
DELIVERABLES_DIR="${PROJECT_ROOT}/deliverables"

EXECUTABLE_BASENAME="hybrid-knapsack"

case "${OSTYPE:-}" in
    msys* | cygwin* | win32*)
        EXECUTABLE_NAME="${EXECUTABLE_BASENAME}.exe"
        ;;
    *)
        EXECUTABLE_NAME="${EXECUTABLE_BASENAME}"
        ;;
esac

BUILD_OUTPUT="${PROJECT_ROOT}/dist/${EXECUTABLE_NAME}"
DELIVERABLE="${DELIVERABLES_DIR}/${EXECUTABLE_NAME}"

echo "Building ${EXECUTABLE_NAME}..."

if [[ ! -f "${SOURCE_FILE}" ]]; then
    echo "Error: rust_connect.py was not found:"
    echo "  ${SOURCE_FILE}"
    exit 1
fi

if [[ ! -f "${RUST_PROJECT}/Cargo.toml" ]]; then
    echo "Error: Cargo.toml was not found:"
    echo "  ${RUST_PROJECT}/Cargo.toml"
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Cargo was not found in PATH."
    echo "Install Rust and Cargo before running this script."
    exit 1
fi

if ! command -v python >/dev/null 2>&1; then
    echo "Error: Python was not found in PATH."
    exit 1
fi

if [[ -z "${VIRTUAL_ENV:-}" ]]; then
    echo "Error: No Python virtual environment is active."
    echo
    echo "Activate the project environment first:"
    echo "  source ./.venv/bin/activate"
    exit 1
fi

if ! command -v maturin >/dev/null 2>&1; then
    echo "Error: Maturin is not installed in the active environment."
    echo "Install it with:"
    echo "  python -m pip install maturin"
    exit 1
fi

if ! python -m PyInstaller --version >/dev/null 2>&1; then
    echo "Error: PyInstaller is not installed in the active environment."
    echo "Install it with:"
    echo "  python -m pip install pyinstaller"
    exit 1
fi

mkdir -p "${DELIVERABLES_DIR}"

cd "${PROJECT_ROOT}"

echo
echo "Building and installing the Rust Python extension..."

maturin develop \
    --release \
    --manifest-path "${RUST_PROJECT}/Cargo.toml"

echo
echo "Verifying the py_connect package and native extension..."

PY_CONNECT_PACKAGE="$(
    python -c '
import pathlib
import py_connect

package_path = pathlib.Path(py_connect.__file__).resolve().parent
print(package_path)
'
)"

if [[ -z "${PY_CONNECT_PACKAGE}" || ! -d "${PY_CONNECT_PACKAGE}" ]]; then
    echo "Error: The py_connect package could not be located."
    exit 1
fi

echo "Python package:"
echo "  ${PY_CONNECT_PACKAGE}"

PY_CONNECT_NATIVE="$(
    python -c '
import importlib.machinery
import pathlib
import py_connect

package_path = pathlib.Path(py_connect.__file__).resolve().parent

for suffix in importlib.machinery.EXTENSION_SUFFIXES:
    native_extensions = list(package_path.glob(f"*{suffix}"))

    if native_extensions:
        print(native_extensions[0].resolve())
        break
'
)"

if [[ -z "${PY_CONNECT_NATIVE}" || ! -f "${PY_CONNECT_NATIVE}" ]]; then
    echo "Error: No compiled py_connect extension was found in:"
    echo "  ${PY_CONNECT_PACKAGE}"
    echo
    echo "Package contents:"
    find "${PY_CONNECT_PACKAGE}" -maxdepth 2 -type f -print
    exit 1
fi

echo "Compiled extension:"
echo "  ${PY_CONNECT_NATIVE}"

echo
echo "Testing py_connect.knapsack_algo..."

python -c '
import py_connect

if not hasattr(py_connect, "knapsack_algo"):
    raise RuntimeError(
        "py_connect was imported, but knapsack_algo was not found."
    )

print("py_connect.knapsack_algo is available.")
'

echo
echo "Creating the standalone hybrid executable..."

python -m PyInstaller \
    --noconfirm \
    --clean \
    --onefile \
    --console \
    --name "${EXECUTABLE_BASENAME}" \
    --collect-all "py_connect" \
    --hidden-import "py_connect" \
    --hidden-import "py_connect.py_connect" \
    "${SOURCE_FILE}"

if [[ ! -f "${BUILD_OUTPUT}" ]]; then
    echo "Error: PyInstaller completed, but the executable was not found:"
    echo "  ${BUILD_OUTPUT}"
    exit 1
fi

mv -f "${BUILD_OUTPUT}" "${DELIVERABLE}"

if [[ "${EXECUTABLE_NAME}" != *.exe ]]; then
    chmod +x "${DELIVERABLE}"
fi

rm -rf "${PROJECT_ROOT}/build"
rm -rf "${PROJECT_ROOT}/dist"
rm -f "${PROJECT_ROOT}/${EXECUTABLE_BASENAME}.spec"

echo
echo "Build completed successfully."
echo "Standalone executable:"
echo "  ${DELIVERABLE}"