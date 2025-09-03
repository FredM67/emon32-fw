//! Simple qfplib test to verify function calls work correctly
//! This focuses on correctness rather than performance

#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[cfg(feature = "qfplib")]
use emon32_rust_poc::math::FastMath;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("qfplib Simple Function Test");
    rprintln!("===========================");

    #[cfg(feature = "qfplib")]
    {
        rprintln!("Testing qfplib functions for correctness...");
        
        // Basic arithmetic tests
        let a = 6.0f32;
        let b = 2.0f32;
        
        rprintln!("Input values: a={}, b={}", a, b);
        
        // Test basic operations
        let add_result = a.fast_add(b);
        let sub_result = a.fast_sub(b);
        let mul_result = a.fast_mul(b);
        let div_result = a.fast_div(b);
        
        rprintln!("qfplib results:");
        rprintln!("  {} + {} = {}", a, b, add_result);
        rprintln!("  {} - {} = {}", a, b, sub_result);
        rprintln!("  {} * {} = {}", a, b, mul_result);
        rprintln!("  {} / {} = {}", a, b, div_result);
        
        // Expected results
        rprintln!("Expected results:");
        rprintln!("  {} + {} = {}", a, b, a + b);
        rprintln!("  {} - {} = {}", a, b, a - b);
        rprintln!("  {} * {} = {}", a, b, a * b);
        rprintln!("  {} / {} = {}", a, b, a / b);
        
        // Test sqrt
        let sqrt_input = 9.0f32;
        let sqrt_result = sqrt_input.fast_sqrt();
        rprintln!("sqrt({}) = {} (expected: 3.0)", sqrt_input, sqrt_result);
        
        // Test if results are reasonable
        let add_ok = (add_result - 8.0).abs() < 0.1;
        let mul_ok = (mul_result - 12.0).abs() < 0.1;
        let div_ok = (div_result - 3.0).abs() < 0.1;
        let sqrt_ok = (sqrt_result - 3.0).abs() < 0.1;
        
        rprintln!("Validation:");
        rprintln!("  Addition OK: {}", add_ok);
        rprintln!("  Multiplication OK: {}", mul_ok);
        rprintln!("  Division OK: {}", div_ok);
        rprintln!("  Square root OK: {}", sqrt_ok);
        
        if add_ok && mul_ok && div_ok && sqrt_ok {
            rprintln!("✅ All qfplib functions working correctly!");
        } else {
            rprintln!("❌ qfplib function errors detected!");
        }
    }
    
    #[cfg(not(feature = "qfplib"))]
    {
        rprintln!("qfplib feature not enabled.");
        rprintln!("Build with --features qfplib to test qfplib functions.");
    }

    rprintln!("Test complete - entering low power mode");
    
    loop {
        cortex_m::asm::wfi();
    }
}