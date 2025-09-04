// qfplib-sys build script with advanced LTO optimization
// Provides multiple optimization profiles for maximum performance

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    let is_arm_cortex_m0plus = target.starts_with("thumbv6m") || 
                               target.contains("cortex-m0") ||
                               target.contains("arm") && env::var("CARGO_FEATURE_ARM_CORTEX_M0PLUS").is_ok();

    println!("cargo:rerun-if-changed=../../third_party/qfplib/qfplib-m0-full.s");
    println!("cargo:rerun-if-changed=../../third_party/qfplib/qfplib-m0-full.h");
    println!("cargo:rerun-if-env-changed=QFPLIB_LTO_LEVEL");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_LTO_AGGRESSIVE");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_LTO_SIZE");

    // Only build for ARM Cortex-M0+ targets
    if !is_arm_cortex_m0plus {
        println!("cargo:warning=qfplib-sys: Skipping build for non-ARM target: {}", target);
        return;
    }

    let qfplib_asm = "../../third_party/qfplib/qfplib-m0-full.s";
    
    if !Path::new(qfplib_asm).exists() {
        panic!("qfplib assembly file not found: {}", qfplib_asm);
    }

    println!("cargo:warning=qfplib-sys: Building optimized qfplib for ARM target: {}", target);

    let out_dir = env::var("OUT_DIR").unwrap();
    let obj_file = PathBuf::from(&out_dir).join("qfplib.o");
    let ar_file = PathBuf::from(&out_dir).join("libqfplib.a");

    // Find ARM GCC toolchain
    let gcc = which::which("arm-none-eabi-gcc")
        .or_else(|_| which::which("arm-none-eabi-as"))
        .expect("ARM GCC toolchain not found. Please install arm-none-eabi-gcc");

    let ar = which::which("arm-none-eabi-ar")
        .expect("ARM ar not found. Please install arm-none-eabi-binutils");

    // Determine optimization level
    let profile = env::var("PROFILE").unwrap_or_default();
    let lto_level = determine_lto_level(&profile);
    
    println!("cargo:warning=qfplib-sys: Using LTO level: {:?}", lto_level);

    // Build qfplib object file with chosen optimization level
    build_qfplib_object(&gcc, qfplib_asm, &obj_file, lto_level);
    
    // Create static library for better LTO integration
    create_static_library(&ar, &obj_file, &ar_file);
    
    // Link the library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=qfplib");
    
    // Enable configuration flags
    println!("cargo:rustc-cfg=qfplib_available");
    
    // Provide link arguments for direct object linking (alternative approach)
    println!("cargo:rustc-env=QFPLIB_OBJ_PATH={}", obj_file.display());
}

#[derive(Debug, Clone, Copy)]
enum LtoLevel {
    Debug,       // No optimization, debug symbols
    Minimal,     // Basic optimization with LTO
    Aggressive,  // Maximum optimization with all flags
    Size,        // Optimize for minimum binary size
}

fn determine_lto_level(profile: &str) -> LtoLevel {
    // Check for explicit feature flags first
    if env::var("CARGO_FEATURE_LTO_AGGRESSIVE").is_ok() {
        return LtoLevel::Aggressive;
    }
    if env::var("CARGO_FEATURE_LTO_SIZE").is_ok() {
        return LtoLevel::Size;
    }
    if env::var("CARGO_FEATURE_LTO_MINIMAL").is_ok() {
        return LtoLevel::Minimal;
    }

    // Check environment variable override
    if let Ok(level) = env::var("QFPLIB_LTO_LEVEL") {
        match level.as_str() {
            "debug" => return LtoLevel::Debug,
            "minimal" => return LtoLevel::Minimal,
            "aggressive" => return LtoLevel::Aggressive,
            "size" => return LtoLevel::Size,
            _ => println!("cargo:warning=Unknown QFPLIB_LTO_LEVEL: {}", level),
        }
    }

    // Fall back to profile-based selection
    match profile {
        "dev" => LtoLevel::Debug,
        "release" => LtoLevel::Aggressive,  // Use aggressive LTO for maximum performance
        "lto-max" => LtoLevel::Aggressive,
        "lto-size" => LtoLevel::Size,
        _ => LtoLevel::Aggressive,  // Default to aggressive for unknown profiles
    }
}

fn build_qfplib_object(gcc: &Path, asm_file: &str, obj_file: &Path, lto_level: LtoLevel) {
    let mut cmd = Command::new(gcc);
    
    // Base ARM Cortex-M0+ configuration
    cmd.arg("-c")                    // Compile only, don't link
       .arg("-mcpu=cortex-m0plus")   // Target CPU
       .arg("-mthumb")               // Use Thumb instruction set
       .arg("-mfloat-abi=soft")      // Soft float ABI
       .arg("-ffunction-sections")   // Each function in its own section
       .arg("-fdata-sections");      // Each data item in its own section

    // Apply optimization level
    match lto_level {
        LtoLevel::Debug => {
            cmd.arg("-Og")                    // Optimize for debugging
               .arg("-g3")                    // Maximum debug info
               .arg("-fno-omit-frame-pointer"); // Keep frame pointer for debugging
        },
        
        LtoLevel::Minimal => {
            cmd.arg("-Os")                    // Optimize for size
               .arg("-flto")                  // Enable LTO
               .arg("-g1")                    // Minimal debug info
               .arg("-fomit-frame-pointer");  // Remove frame pointer
        },
        
        LtoLevel::Aggressive => {
            cmd.arg("-O3")                         // Maximum speed optimization
               .arg("-flto")                       // Enable LTO
               .arg("-fomit-frame-pointer")        // Remove frame pointer
               .arg("-fno-unwind-tables")          // Remove unwind tables
               .arg("-fno-asynchronous-unwind-tables") // Remove async unwind tables
               .arg("-finline-functions")          // Aggressive inlining
               .arg("-finline-small-functions")    // Inline small functions
               .arg("-fipa-sra")                   // Scalar replacement of aggregates
               .arg("-fdevirtualize-at-ltrans")    // Devirtualize at link time
               .arg("-fwhole-program")             // Whole program optimization
               .arg("-ftree-vectorize")            // Enable vectorization
               .arg("-ffast-math")                 // Fast math optimizations
               .arg("-mno-unaligned-access");      // Cortex-M0+ specific
        },
        
        LtoLevel::Size => {
            cmd.arg("-Oz")                    // Maximum size optimization  
               .arg("-flto")                  // Enable LTO
               .arg("-fomit-frame-pointer")   // Remove frame pointer
               .arg("-fno-unwind-tables")     // Remove unwind tables
               .arg("-fno-asynchronous-unwind-tables") // Remove async unwind tables
               .arg("-fdata-sections")        // Enable dead code elimination
               .arg("-ffunction-sections")    // Enable dead code elimination
               .arg("-fno-stack-protector")   // Remove stack protection overhead
               .arg("-fno-ident")             // Remove compiler identification
               .arg("-fmerge-all-constants"); // Merge identical constants
        },
    }
    
    cmd.arg("-o").arg(obj_file)       // Output object file
       .arg(asm_file);                // Input assembly file

    println!("cargo:warning=qfplib-sys: Running: {:?}", cmd);
    
    let output = cmd.output().expect("Failed to execute arm-none-eabi-gcc");
    
    if !output.status.success() {
        println!("cargo:warning=GCC stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("cargo:warning=GCC stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to compile qfplib with LTO level: {:?}", lto_level);
    }
    
    println!("cargo:warning=qfplib-sys: Successfully compiled qfplib object");
}

fn create_static_library(ar: &Path, obj_file: &Path, ar_file: &Path) {
    let mut cmd = Command::new(ar);
    cmd.arg("rcs")           // Create archive with symbol table
       .arg(ar_file)         // Output archive file
       .arg(obj_file);       // Input object file

    println!("cargo:warning=qfplib-sys: Creating static library: {:?}", cmd);
    
    let output = cmd.output().expect("Failed to execute arm-none-eabi-ar");
    
    if !output.status.success() {
        println!("cargo:warning=AR stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("cargo:warning=AR stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to create qfplib static library");
    }
    
    println!("cargo:warning=qfplib-sys: Successfully created static library");
}