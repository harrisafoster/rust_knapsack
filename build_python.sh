#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXECUTABLE_BASENAME="python-knapsack"

SOURCE_FILE="${PROJECT_ROOT}/optimization.py"
DATA_FILE="${PROJECT_ROOT}/data.csv"
DELIVERABLES_DIR="${PROJECT_ROOT}/deliverables"

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
    echo "Error: optimization.py was not found at:"
    echo "  ${SOURCE_FILE}"
    exit 1
fi

if [[ ! -f "${DATA_FILE}" ]]; then
    echo "Error: data.csv was not found at:"
    echo "  ${DATA_FILE}"
    exit 1
fi

if ! python -m PyInstaller --version >/dev/null 2>&1; then
    echo "Error: PyInstaller is not installed in the active Python environment."
    echo "Install it with:"
    echo "  python -m pip install pyinstaller"
    exit 1
fi

mkdir -p "${DELIVERABLES_DIR}"

cd "${PROJECT_ROOT}"

python -m PyInstaller \
    --noconfirm \
    --clean \
    --onefile \
    --console \
    --name "${EXECUTABLE_BASENAME}" \
    --add-data "data.csv:." \
    "${SOURCE_FILE}"

if [[ ! -f "${BUILD_OUTPUT}" ]]; then
    echo "Error: PyInstaller completed, but the executable was not found at:"
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