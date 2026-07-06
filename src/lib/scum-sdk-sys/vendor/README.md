# Vendored SCuM SDK sources

Vendored copy of the `sdk/bsp` directory of the SCuM SDK:

- Upstream: https://github.com/PisterLab/scum-sdk
- Commit: 8988a8c4eb7b0910f9fc00cd56a7c18f9bc6923d
- License: see LICENSE.txt (upstream license)

The files are vendored unmodified to ease updating from upstream. Only a
subset is compiled by the crate build script (see build.rs): the chip
initialization and optical calibration code. In particular startup.c and
syscalls.c must never be compiled: startup and the C library integration
are provided by Ariel OS and by the shim directory of this crate.
