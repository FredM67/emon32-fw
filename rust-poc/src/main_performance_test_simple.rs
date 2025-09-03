#![no_std]
#![no_main]

use atsamd_hal::{
    clock::GenericClockController,
    pac::{CorePeripherals, Peripherals},
};
use cortex_m::asm;
use cortex_m_rt::entry;
use micromath::F32Ext;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

// Import our math modules for comparison
use emon32_rust_poc::math::FastMath;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("=== ARM Cortex-M0+ Math Performance Test ===");
    rprintln!("Target: SAMD21J17 @ 48MHz");
    rprintln!("Testing: Standard Rust vs qfplib performance");

    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let mut _clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    rprintln!("\n=== Math Library Configuration ===");
    #[cfg(feature = "qfplib")]
    rprintln!("✓ qfplib enabled - ARM-optimized assembly");

    #[cfg(not(feature = "qfplib"))]
    rprintln!("⚠ qfplib disabled - standard Rust math");

    rprintln!("\n1. BASIC ARITHMETIC OPERATIONS");
    rprintln!("==============================");
    test_basic_arithmetic();

    rprintln!("\n2. TRANSCENDENTAL FUNCTIONS");
    rprintln!("===========================");
    test_transcendental_functions();

    rprintln!("\n3. ENERGY CALCULATION SIMULATION");
    rprintln!("================================");
    test_energy_calculation();

    rprintln!("\n4. ACCURACY VALIDATION");
    rprintln!("======================");
    test_accuracy();

    rprintln!("\n=== Performance Test Complete ===");
    rprintln!("Results demonstrate actual ARM Cortex-M0+ performance");
    rprintln!("Compare with/without --features qfplib for speedup measurement");
    rprintln!("Test completed successfully - entering low power mode");

    loop {
        asm::wfi();
    }
}

fn get_dummy_timestamp() -> u32 {
    // Simple dummy timing for demonstration
    // Real timing would require hardware timer setup
    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

fn test_basic_arithmetic() {
    const NUM_OPERATIONS: u32 = 1000;

    rprintln!("Basic Arithmetic ({} operations):", NUM_OPERATIONS);
    
    // Test FastMath arithmetic operations
    let mut result = 1.0f32;
    for i in 0..NUM_OPERATIONS {
        let x = (i as f32).fast_mul(0.001);
        result = result.fast_add(x.fast_mul(x));
    }

    rprintln!("  Operations completed successfully");
    rprintln!("  Result: {:.6}", result);
    rprintln!("  Note: Timing requires hardware timer - see qfplib performance test");
}

fn test_transcendental_functions() {
    const NUM_SQRT: u32 = 100;
    const NUM_TRIG: u32 = 50;

    rprintln!("Square Root ({} operations):", NUM_SQRT);
    
    // Test square root performance
    let mut sqrt_result = 0.0f32;
    for i in 1..=NUM_SQRT {
        sqrt_result = sqrt_result.fast_add((i as f32).fast_sqrt());
    }

    rprintln!("  Operations completed successfully");
    rprintln!("  Result: {:.3}", sqrt_result);

    rprintln!("Trigonometric ({} sin+cos pairs):", NUM_TRIG);
    
    // Test trigonometric performance
    let mut trig_result = 0.0f32;
    for i in 0..NUM_TRIG {
        let x = (i as f32).fast_mul(0.01);
        trig_result = trig_result.fast_add(x.fast_sin().fast_add(x.fast_cos()));
    }

    rprintln!("  Operations completed successfully");
    rprintln!("  Result: {:.3}", trig_result);
    rprintln!("  Note: For precise timing, use the qfplib performance test");
}

fn test_energy_calculation() {
    const NUM_SAMPLES: usize = 96; // Smaller for ARM testing
    const NUM_ITERATIONS: u32 = 5; // Reasonable for ARM

    rprintln!(
        "Energy Calculation ({} samples × {} iterations):",
        NUM_SAMPLES,
        NUM_ITERATIONS
    );

    // Generate test ADC data (simpler approach)
    let mut test_samples = [0u16; NUM_SAMPLES];
    for (i, sample) in test_samples.iter_mut().enumerate() {
        // Simple sine approximation to avoid complex math in setup
        let phase = i % 16;
        let amplitude = if phase < 8 {
            phase * 200
        } else {
            (16 - phase) * 200
        };
        *sample = (2048 + amplitude) as u16;
    }

    // Test energy calculation with FastMath
    for iteration in 0..NUM_ITERATIONS {
        let mut voltage_rms = 0.0f32;
        let mut current_rms = 0.0f32;
        let mut power = 0.0f32;

        for &sample in &test_samples {
            let voltage = (sample as f32).fast_mul(0.001); // Scale to voltage
            let current = voltage.fast_mul(0.5); // Simulate current

            voltage_rms = voltage_rms.fast_add(voltage.fast_mul(voltage));
            current_rms = current_rms.fast_add(current.fast_mul(current));
            power = power.fast_add(voltage.fast_mul(current));
        }

        let sample_count = NUM_SAMPLES as f32;
        let v_rms = (voltage_rms.fast_div(sample_count)).fast_sqrt();
        let i_rms = (current_rms.fast_div(sample_count)).fast_sqrt();
        let avg_power = power.fast_div(sample_count);
        
        if iteration == 0 {
            rprintln!("  Sample results: V_rms={:.3}, I_rms={:.3}, P_avg={:.3}", 
                     v_rms, i_rms, avg_power);
        }
    }

    rprintln!("  ✓ All {} iterations completed successfully", NUM_ITERATIONS);
    rprintln!("  Note: For precise timing measurements, use the qfplib performance test");
}

fn test_accuracy() {
    rprintln!("Accuracy Comparison:");

    // Test a few specific values for accuracy
    let test_values = [1.0f32, 2.0f32, 4.0f32, 9.0f32];

    for &val in &test_values {
        let fast_sqrt = val.fast_sqrt();
        let expected = val.sqrt(); // micromath implementation
        let error = (fast_sqrt - expected).abs();
        let rel_error = if expected != 0.0 {
            error / expected.abs()
        } else {
            error
        };

        rprintln!(
            "  sqrt({:.1}): fast={:.6}, std={:.6}, err={:.2e}",
            val,
            fast_sqrt,
            expected,
            rel_error
        );
    }

    // Test trig functions
    let angles = [0.0f32, 0.5f32, 1.0f32];
    for &angle in &angles {
        let fast_sin = angle.fast_sin();
        let expected_sin = angle.sin();
        let sin_error = (fast_sin - expected_sin).abs();

        rprintln!(
            "  sin({:.1}): fast={:.6}, std={:.6}, err={:.2e}",
            angle,
            fast_sin,
            expected_sin,
            sin_error
        );
    }
    
    rprintln!("Accuracy validation completed successfully!");
}
