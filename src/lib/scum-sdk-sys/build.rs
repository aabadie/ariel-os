fn main() {
    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-changed=shim");
    println!("cargo:rerun-if-changed=build.rs");

    if !is_in_current_contexts(&["scum"]) {
        // Platform-independent tooling.
        return;
    }

    // Only the chip initialization and optical calibration code is compiled
    // from the vendored SDK, plus a small stdio shim (the SDK code prints
    // its calibration progress). This mirrors the RIOT port of SCuM.
    // startup.c and syscalls.c must never be compiled: startup and the C
    // library integration are provided by Ariel OS and by the shim.
    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .file("vendor/bsp/scm3c_hw_interface.c")
        .file("vendor/bsp/optical.c")
        .file("shim/printf.c")
        .include("vendor/bsp")
        .include("vendor/bsp/cmsis")
        .define("MODULE_OPTICAL", None)
        // Same target and optimization flags as the SDK build (MinSizeRel),
        // to stay as close as possible to the known-working SDK behavior.
        // The calibration ISR is timing-sensitive (see the "1.1V/VDDD tap
        // fix" comments in optical.c). -fshort-enums and -fshort-wchar are
        // not used to remain ABI-compatible with the Rust side, and NDEBUG
        // is not defined so that the SDK boot/calibration prints are kept.
        .flag("-mcpu=cortex-m0")
        .flag("-march=armv6s-m")
        .flag("-mthumb")
        .flag("-mlittle-endian")
        .flag("-mfloat-abi=soft")
        .flag("-std=c17")
        .flag("-Os")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .flag("-fwrapv")
        .flag("-fno-common")
        .flag("-fomit-frame-pointer")
        .flag("-fno-delete-null-pointer-checks")
        .flag("-fno-builtin")
        // The final link uses rust-lld with the Rust compiler-builtins and
        // no libgcc: jump tables would emit __gnu_thumb1_case_* helper
        // calls that nothing provides.
        .flag("-fno-jump-tables")
        .warnings(false)
        .compile("scum-sdk");
}

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| contexts.contains(&c))
}
