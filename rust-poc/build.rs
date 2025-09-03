// Build script for emon32 Rust POC
// Integrates qfplib assembly for optimal floating-point performance

fn main() {
    // Only build qfplib for ARM targets
    if cfg!(target_arch = "arm") && cfg!(feature = "qfplib") {
        println!("cargo:rerun-if-changed=../third_party/qfplib/qfplib-m0-full.s");

        // Build qfplib assembly
        cc::Build::new()
            .file("../third_party/qfplib/qfplib-m0-full.s")
            .flag("-mcpu=cortex-m0plus")
            .flag("-mthumb")
            .compile("qfplib");

        println!("cargo:rustc-link-lib=qfplib");
        println!("cargo:rustc-cfg=qfplib_available");
    }
}
