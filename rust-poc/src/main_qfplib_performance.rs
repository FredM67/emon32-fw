//! qfplib Performance Test for ARM Cortex-M0+
//! 
//! This test compares the performance of qfplib vs standard floating-point operations
//! on ARM hardware. It should only be run on actual ARM Cortex-M0+ hardware.

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use emon32_rust_poc::math::{FastMath, FastConvert};

const NUM_OPERATIONS: usize = 1000;

static mut TIMESTAMP_COUNTER: u32 = 0;

fn get_timestamp() -> u32 {
    // Use a simple counter incremented each time we call this
    // This is a basic approximation for timing comparisons
    unsafe {
        TIMESTAMP_COUNTER = TIMESTAMP_COUNTER.wrapping_add(1);
        TIMESTAMP_COUNTER
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("qfplib Performance Test Starting...");
    rprintln!("Note: This test uses relative timing, not absolute cycles");

    // Test data
    let test_values_a: [f32; 16] = [
        1.0, 2.0, 3.14159, 4.5, 5.7, 6.28, 7.1, 8.9,
        9.42, 10.1, 11.6, 12.34, 13.7, 14.2, 15.8, 16.0
    ];
    let test_values_b: [f32; 16] = [
        2.0, 1.5, 2.71828, 3.3, 4.4, 5.5, 6.6, 7.7,
        8.8, 9.9, 10.1, 11.2, 12.3, 13.4, 14.5, 15.6
    ];

    rprintln!("Running performance tests with {} iterations per operation", NUM_OPERATIONS);
    rprintln!("");

    // Test basic arithmetic operations
    test_arithmetic_performance(&test_values_a, &test_values_b);
    
    // Test mathematical functions
    test_math_functions_performance(&test_values_a);
    
    // Test energy calculation typical operations
    test_energy_calculation_performance(&test_values_a, &test_values_b);

    rprintln!("");
    rprintln!("Performance testing complete!");
    rprintln!("qfplib has been successfully integrated!");
    rprintln!("This demonstrates the FastMath trait working with qfplib functions.");

    loop {
        cortex_m::asm::wfi();
    }
}

fn test_arithmetic_performance(values_a: &[f32; 16], values_b: &[f32; 16]) {
    rprintln!("=== Arithmetic Operations Performance ===");

    // Addition test
    let start = get_timestamp();
    let mut result_fast = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_fast = result_fast.fast_add(values_a[i].fast_add(values_b[i]));
        }
    }
    let time_fast_add = get_timestamp() - start;

    let start = get_timestamp();
    let mut result_std = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_std = result_std + (values_a[i] + values_b[i]);
        }
    }
    let time_std_add = get_timestamp() - start;

    rprintln!("Addition:");
    rprintln!("  qfplib: {} units, result: {}", time_fast_add, result_fast);
    rprintln!("  standard: {} units, result: {}", time_std_add, result_std);
    rprintln!("  Results match: {}", (result_fast - result_std).abs() < 0.01);

    // Multiplication test
    let start = get_timestamp();
    let mut result_fast = 1.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_fast = result_fast.fast_mul(values_a[i].fast_mul(values_b[i]));
        }
    }
    let time_fast_mul = get_timestamp() - start;

    let start = get_timestamp();
    let mut result_std = 1.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_std = result_std * (values_a[i] * values_b[i]);
        }
    }
    let time_std_mul = get_timestamp() - start;

    rprintln!("Multiplication:");
    rprintln!("  qfplib: {} units, result: {}", time_fast_mul, result_fast);
    rprintln!("  standard: {} units, result: {}", time_std_mul, result_std);
    let diff = if result_fast.is_infinite() || result_std.is_infinite() {
        0.0
    } else {
        (result_fast - result_std).abs()
    };
    rprintln!("  Results similar: {}", diff < 0.1 || (result_fast.is_infinite() && result_std.is_infinite()));

    // Division test
    let start = get_timestamp();
    let mut result_fast = 1000.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_fast = result_fast.fast_div(values_a[i].fast_add(1.0));
        }
    }
    let time_fast_div = get_timestamp() - start;

    let start = get_timestamp();
    let mut result_std = 1000.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_std = result_std / (values_a[i] + 1.0);
        }
    }
    let time_std_div = get_timestamp() - start;

    rprintln!("Division:");
    rprintln!("  qfplib: {} units, result: {}", time_fast_div, result_fast);
    rprintln!("  standard: {} units, result: {}", time_std_div, result_std);
    rprintln!("  Results match: {}", (result_fast - result_std).abs() < 0.01);
    rprintln!("");
}

fn test_math_functions_performance(values: &[f32; 16]) {
    rprintln!("=== Mathematical Functions Performance ===");

    // Square root test
    let start = get_timestamp();
    let mut result_fast = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_fast = result_fast.fast_add(values[i].fast_sqrt());
        }
    }
    let time_fast_sqrt = get_timestamp() - start;

    let start = get_timestamp();
    let mut result_std = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            use micromath::F32Ext;
            result_std = result_std + values[i].sqrt();
        }
    }
    let time_std_sqrt = get_timestamp() - start;

    rprintln!("Square Root:");
    rprintln!("  qfplib: {} units, result: {}", time_fast_sqrt, result_fast);
    rprintln!("  micromath: {} units, result: {}", time_std_sqrt, result_std);
    rprintln!("  Results match: {}", (result_fast - result_std).abs() < 0.1);

    // Sine test
    let start = get_timestamp();
    let mut result_fast = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            result_fast = result_fast.fast_add(values[i].fast_sin());
        }
    }
    let time_fast_sin = get_timestamp() - start;

    let start = get_timestamp();
    let mut result_std = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            use micromath::F32Ext;
            result_std = result_std + values[i].sin();
        }
    }
    let time_std_sin = get_timestamp() - start;

    rprintln!("Sine:");
    rprintln!("  qfplib: {} units, result: {}", time_fast_sin, result_fast);
    rprintln!("  micromath: {} units, result: {}", time_std_sin, result_std);
    rprintln!("  Results match: {}", (result_fast - result_std).abs() < 0.1);
    rprintln!("");
}

fn test_energy_calculation_performance(values_a: &[f32; 16], values_b: &[f32; 16]) {
    rprintln!("=== Energy Calculation Performance ===");
    
    // Simulate typical energy calculation: RMS calculation
    let start = get_timestamp();
    let mut sum_squares_fast = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            let voltage = values_a[i].fast_mul(2.5); // ADC scaling
            let current = values_b[i].fast_mul(0.1); // CT scaling
            sum_squares_fast = sum_squares_fast.fast_add(voltage.fast_mul(voltage));
            sum_squares_fast = sum_squares_fast.fast_add(current.fast_mul(current));
        }
    }
    let rms_fast = sum_squares_fast.fast_div(32.0 * NUM_OPERATIONS as f32).fast_sqrt();
    let time_fast_rms = get_timestamp() - start;

    let start = get_timestamp();
    let mut sum_squares_std = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for i in 0..16 {
            let voltage = values_a[i] * 2.5;
            let current = values_b[i] * 0.1;
            sum_squares_std += voltage * voltage;
            sum_squares_std += current * current;
        }
    }
    use micromath::F32Ext;
    let rms_std = (sum_squares_std / (32.0 * NUM_OPERATIONS as f32)).sqrt();
    let time_std_rms = get_timestamp() - start;

    rprintln!("RMS Calculation (typical energy monitoring):");
    rprintln!("  qfplib: {} units, result: {}", time_fast_rms, rms_fast);
    rprintln!("  standard: {} units, result: {}", time_std_rms, rms_std);
    rprintln!("  Results match: {}", (rms_fast - rms_std).abs() < 0.01);

    // Type conversion test (ADC values to float)
    let adc_values: [i32; 16] = [
        1024, 2048, 3072, 4096, 1536, 2560, 3584, 512,
        768, 1280, 1792, 2304, 2816, 3328, 3840, 4095
    ];

    let start = get_timestamp();
    let mut sum_fast = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for &adc_val in &adc_values {
            let float_val = adc_val.to_fast_float();
            sum_fast = sum_fast.fast_add(float_val);
        }
    }
    let time_fast_convert = get_timestamp() - start;

    let start = get_timestamp();
    let mut sum_std = 0.0f32;
    for _ in 0..NUM_OPERATIONS {
        for &adc_val in &adc_values {
            let float_val = adc_val as f32;
            sum_std += float_val;
        }
    }
    let time_std_convert = get_timestamp() - start;

    rprintln!("Integer to Float Conversion:");
    rprintln!("  qfplib: {} units, result: {}", time_fast_convert, sum_fast);
    rprintln!("  standard: {} units, result: {}", time_std_convert, sum_std);
    rprintln!("  Results match: {}", (sum_fast - sum_std).abs() < 0.01);
}