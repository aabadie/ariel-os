#!/bin/bash

# Cargo runner for the SCuM target.
#
# Converts the ELF produced by cargo to a raw binary, then loads it onto
# SCuM through the scum-programmer tool (nRF52840-DK based, see the
# scum_programmer directory of https://github.com/PisterLab/scum-sdk).
#
# Environment variables:
#   SCUM_PROGRAMMER           programmer command (default: scum-programmer)
#   SCUM_PROGRAMMER_PORT      serial port of the nRF52840-DK (default: auto)
#   SCUM_PROGRAMMER_BAUDRATE  programmer baudrate (default: tool default)
#   SCUM_NO_CALIBRATE         set to 1 to skip the optical calibration
#                             sequence after flashing

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "usage: $0 <elf-file>" >&2
    exit 1
fi

ELF="$1"
shift
BIN="${ELF}.bin"

OBJCOPY=""
for candidate in rust-objcopy llvm-objcopy arm-none-eabi-objcopy; do
    if command -v "$candidate" > /dev/null 2>&1; then
        OBJCOPY="$candidate"
        break
    fi
done

if [ -z "$OBJCOPY" ]; then
    echo "error: no objcopy found (tried rust-objcopy, llvm-objcopy, arm-none-eabi-objcopy)" >&2
    echo "hint: install cargo-binutils (cargo install cargo-binutils) or the ARM GNU toolchain" >&2
    exit 1
fi

PROGRAMMER="${SCUM_PROGRAMMER:-scum-programmer}"

if ! command -v "$PROGRAMMER" > /dev/null 2>&1; then
    echo "error: '$PROGRAMMER' not found" >&2
    echo "hint: pip install scum-programmer, or set SCUM_PROGRAMMER" >&2
    exit 1
fi

PROGRAMMER_ARGS=()
if [ -n "${SCUM_PROGRAMMER_PORT:-}" ]; then
    PROGRAMMER_ARGS+=(--port "$SCUM_PROGRAMMER_PORT")
fi
if [ -n "${SCUM_PROGRAMMER_BAUDRATE:-}" ]; then
    PROGRAMMER_ARGS+=(--baudrate "$SCUM_PROGRAMMER_BAUDRATE")
fi
if [ "${SCUM_NO_CALIBRATE:-0}" != "1" ]; then
    PROGRAMMER_ARGS+=(--calibrate)
fi

set -x
"$OBJCOPY" -O binary "$ELF" "$BIN"
exec "$PROGRAMMER" "${PROGRAMMER_ARGS[@]}" "$BIN"
