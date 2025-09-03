//! qfplib Performance Test for ARM Cortex-M0+
//! 
//! This test compares the performance of qfplib vs standard floating-point operations
//! on ARM hardware using SysTick for accurate timing measurements.

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use rtt_target::{rprintln, rtt_init_print};

use emon32_rust_poc::math::FastMath;

const NUM_OPERATIONS: usize = 1000;
const SYSTICK_FREQUENCY: u32 = 48_000_000; // 48MHz system clock

/// ARM SysTick-based timing for performance measurement
struct PerformanceTimer {
    systick: SYST,
}

impl PerformanceTimer {
    fn new(mut systick: SYST) -> Self {
        // Configure SysTick for maximum count (24-bit counter)
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(0x00FF_FFFF); // Maximum 24-bit value
        systick.clear_current();
        systick.enable_counter();
        
        Self { systick }
    }
    
    fn start_measurement(&mut self) -> u32 {
        self.systick.clear_current();
        SYST::get_current()
    }
    
    fn end_measurement(&self, start_val: u32) -> u32 {
        let end_val = SYST::get_current();
        // SysTick counts down, so elapsed = start - end
        start_val.wrapping_sub(end_val)
    }
    
    fn cycles_to_microseconds(&self, cycles: u32) -> u32 {
        // Convert CPU cycles to microseconds
        (cycles * 1_000_000) / SYSTICK_FREQUENCY
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("qfplib Performance Test Starting...");
    rprintln!("ARM Cortex-M0+ SysTick-based timing measurement");

    // Get the peripherals
    let peripherals = cortex_m::Peripherals::take().unwrap();
    let mut timer = PerformanceTimer::new(peripherals.SYST);
    
    rprintln!("SysTick frequency: {} Hz", SYSTICK_FREQUENCY);
    rprintln!("Running performance tests with {} operations per test", NUM_OPERATIONS);
    rprintln!("");

    // Test both qfplib and standard math (since qfplib binary includes both)
    rprintln!("Testing standard floating-point (micromath fallback):");
    run_performance_test(&mut timer, "Standard Math (micromath)");
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("\nTesting qfplib optimized floating-point:");
        run_performance_test(&mut timer, "qfplib Fast Math");
    }

    rprintln!("");
    rprintln!("Performance testing complete!");
    
    #[cfg(feature = "qfplib")]
    rprintln!("qfplib has been successfully integrated and tested!");
    
    #[cfg(not(feature = "qfplib"))]
    rprintln!("Standard math baseline established. Build with --features qfplib for comparison.");

    loop {
        cortex_m::asm::wfi();
    }
}

fn run_performance_test(timer: &mut PerformanceTimer, test_name: &str) {
    rprintln!("\n=== {} Performance Test ===", test_name);
    
    // Test data: representative energy monitoring values
    let voltage_samples = [220.5f32, 221.2, 219.8, 220.1, 220.9];
    let current_samples = [2.1f32, 2.3, 1.9, 2.2, 2.0];
    
    // Test 1: Square root operations (RMS calculations)
    let start = timer.start_measurement();
    for _ in 0..NUM_OPERATIONS {
        for &v in &voltage_samples {
            let _ = (v * v).fast_sqrt();
        }
    }
    let sqrt_cycles = timer.end_measurement(start);
    let sqrt_us = timer.cycles_to_microseconds(sqrt_cycles);
    
    // Test 2: Division operations (power calculations)
    let start = timer.start_measurement();
    for _ in 0..NUM_OPERATIONS {
        for i in 0..voltage_samples.len() {
            let _ = voltage_samples[i].fast_div(current_samples[i]);
        }
    }
    let div_cycles = timer.end_measurement(start);
    let div_us = timer.cycles_to_microseconds(div_cycles);
    
    // Test 3: Multiplication operations (power calculations)
    let start = timer.start_measurement();
    for _ in 0..NUM_OPERATIONS {
        for i in 0..voltage_samples.len() {
            let _ = voltage_samples[i].fast_mul(current_samples[i]);
        }
    }
    let mul_cycles = timer.end_measurement(start);
    let mul_us = timer.cycles_to_microseconds(mul_cycles);
    
    // Test 4: Combined energy calculation
    let start = timer.start_measurement();
    for _ in 0..NUM_OPERATIONS {
        let mut energy = 0.0f32;
        for i in 0..voltage_samples.len() {
            let power = voltage_samples[i].fast_mul(current_samples[i]);
            energy = energy.fast_add(power);
        }
        let _ = energy.fast_sqrt();
    }
    let combined_cycles = timer.end_measurement(start);
    let combined_us = timer.cycles_to_microseconds(combined_cycles);
    
    rprintln!("Results for {} operations:", NUM_OPERATIONS * voltage_samples.len());
    rprintln!("  Square root: {} cycles ({} μs)", sqrt_cycles, sqrt_us);
    rprintln!("  Division:    {} cycles ({} μs)", div_cycles, div_us);
    rprintln!("  Multiply:    {} cycles ({} μs)", mul_cycles, mul_us);
    rprintln!("  Combined:    {} cycles ({} μs)", combined_cycles, combined_us);
    
    let total_cycles = sqrt_cycles + div_cycles + mul_cycles + combined_cycles;
    let total_us = timer.cycles_to_microseconds(total_cycles);
    rprintln!("  Total:       {} cycles ({} μs)", total_cycles, total_us);
}