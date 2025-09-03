// Build script for emon32 Rust POC
// Integrates qfplib assembly for optimal floating-point performance

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    let is_arm = target.starts_with("thumbv") || target.contains("arm");
    let qfplib_enabled = env::var("CARGO_FEATURE_QFPLIB").is_ok();

    println!("cargo:rerun-if-changed=../third_party/qfplib/qfplib-m0-full.s");
    println!("cargo:rerun-if-changed=../third_party/qfplib/qfplib-m0-full.h");

    // Only build qfplib for ARM targets with qfplib feature enabled
    if is_arm && qfplib_enabled {
        let qfplib_asm = "../third_party/qfplib/qfplib-m0-full.s";
        
        if !Path::new(qfplib_asm).exists() {
            panic!("qfplib assembly file not found: {}", qfplib_asm);
        }

        println!("cargo:warning=Building qfplib for ARM target: {}", target);

        let out_dir = env::var("OUT_DIR").unwrap();
        let obj_file = PathBuf::from(&out_dir).join("qfplib.o");

        // Find ARM GCC toolchain
        let gcc = which::which("arm-none-eabi-gcc")
            .or_else(|_| which::which("arm-none-eabi-as"))
            .expect("ARM GCC toolchain not found. Please install arm-none-eabi-gcc");

        // Assemble the qfplib source
        let mut gcc_cmd = Command::new(&gcc);
        gcc_cmd
            .arg("-c")                    // Compile only, don't link
            .arg("-mcpu=cortex-m0plus")   // Target CPU
            .arg("-mthumb")               // Use Thumb instruction set
            .arg("-mfloat-abi=soft")      // Soft float ABI
            .arg("-ffunction-sections")   // Each function in its own section
            .arg("-fdata-sections")       // Each data item in its own section
            .arg("-o").arg(&obj_file)     // Output object file
            .arg(qfplib_asm);             // Input assembly file

        println!("cargo:warning=Running: {:?}", gcc_cmd);
        
        let output = gcc_cmd.output().expect("Failed to execute arm-none-eabi-gcc");
        
        if !output.status.success() {
            println!("cargo:warning=GCC stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("cargo:warning=GCC stderr: {}", String::from_utf8_lossy(&output.stderr));
            panic!("Failed to assemble qfplib");
        }

        // Instead of creating a library, just tell Rust to link the object file directly
        println!("cargo:rustc-link-arg={}", obj_file.display());
        println!("cargo:rustc-cfg=qfplib_available");
        
    } else if qfplib_enabled && !is_arm {
        println!("cargo:warning=qfplib feature enabled but not building for ARM - using standard math");
    }
}
