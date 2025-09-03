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

// Performance measurement using SysTick
static mut SYSTICK_COUNTER: u32 = 0;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("=== ARM Cortex-M0+ Math Performance Test ===");
    rprintln!("Target: SAMD21J17 @ 48MHz");
    rprintln!("Testing: Standard Rust vs qfplib performance");

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut _clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    // Configure SysTick for precise timing (1μs resolution)
    let mut systick = core.SYST;
    systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    systick.set_reload(47); // 48MHz / 48 = 1MHz = 1μs per tick
    systick.clear_current();
    systick.enable_counter();
    systick.enable_interrupt();

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

    loop {
        asm::wfi();
    }
}

fn get_timestamp() -> u32 {
    unsafe { SYSTICK_COUNTER }
}

fn test_basic_arithmetic() {
    const NUM_OPERATIONS: u32 = 1000;

    // Test FastMath arithmetic operations
    let start = get_timestamp();
    let mut result = 1.0f32;
    for i in 0..NUM_OPERATIONS {
        let x = (i as f32).fast_mul(0.001);
        result = result.fast_add(x.fast_mul(x));
    }
    let fast_time = get_timestamp() - start;

    rprintln!("Basic Arithmetic ({} operations):", NUM_OPERATIONS);
    rprintln!("  Time: {}μs", fast_time);
    rprintln!(
        "  Rate: {} ops/ms",
        NUM_OPERATIONS * 1000 / fast_time.max(1)
    );
    rprintln!("  Result: {:.6}", result);
}

fn test_transcendental_functions() {
    const NUM_SQRT: u32 = 100;
    const NUM_TRIG: u32 = 50;

    // Test square root performance
    let start = get_timestamp();
    let mut sqrt_result = 0.0f32;
    for i in 1..=NUM_SQRT {
        sqrt_result = sqrt_result.fast_add((i as f32).fast_sqrt());
    }
    let sqrt_time = get_timestamp() - start;

    // Test trigonometric performance
    let start = get_timestamp();
    let mut trig_result = 0.0f32;
    for i in 0..NUM_TRIG {
        let x = (i as f32).fast_mul(0.01);
        trig_result = trig_result.fast_add(x.fast_sin().fast_add(x.fast_cos()));
    }
    let trig_time = get_timestamp() - start;

    rprintln!("Square Root ({} operations):", NUM_SQRT);
    rprintln!("  Time: {}μs", sqrt_time);
    rprintln!("  Rate: {} ops/ms", NUM_SQRT * 1000 / sqrt_time.max(1));

    rprintln!("Trigonometric ({} sin+cos pairs):", NUM_TRIG);
    rprintln!("  Time: {}μs", trig_time);
    rprintln!("  Rate: {} pairs/ms", NUM_TRIG * 1000 / trig_time.max(1));

    rprintln!(
        "  Results: sqrt={:.3}, trig={:.3}",
        sqrt_result,
        trig_result
    );
}

fn test_energy_calculation() {
    const NUM_SAMPLES: usize = 96; // Smaller for ARM testing
    const NUM_ITERATIONS: u32 = 5; // Reasonable for ARM

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
    let start = get_timestamp();
    for _ in 0..NUM_ITERATIONS {
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
        let _v_rms = (voltage_rms.fast_div(sample_count)).fast_sqrt();
        let _i_rms = (current_rms.fast_div(sample_count)).fast_sqrt();
        let _avg_power = power.fast_div(sample_count);
    }
    let energy_time = get_timestamp() - start;

    rprintln!(
        "Energy Calculation ({} samples × {} iterations):",
        NUM_SAMPLES,
        NUM_ITERATIONS
    );
    rprintln!("  Total time: {}μs", energy_time);
    rprintln!("  Per iteration: {}μs", energy_time / NUM_ITERATIONS);

    // Real-time capability analysis
    let per_iteration_us = energy_time / NUM_ITERATIONS;
    let max_sample_rate = 1_000_000 / per_iteration_us.max(1); // Hz
    rprintln!("  Max sample rate: {} Hz", max_sample_rate);

    // Energy monitoring requirement: 4.8kHz per channel
    if max_sample_rate >= 4800 {
        rprintln!("  ✓ Sufficient for 4.8kHz energy monitoring");
    } else {
        rprintln!("  ⚠ May be insufficient for 4.8kHz requirement");
        rprintln!("    Need {} Hz, have {} Hz", 4800, max_sample_rate);
    }
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
}

// SysTick interrupt handler for timing
#[cortex_m_rt::exception]
fn SysTick() {
    unsafe {
        SYSTICK_COUNTER += 1;
    }
}
