//! qfplib Complex Math Performance Test
//! 
//! This test focuses on complex mathematical operations where qfplib should
//! demonstrate significant performance advantages over micromath.

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use rtt_target::{rprintln, rtt_init_print};

use emon32_rust_poc::math::FastMath;

const COMPLEX_OPERATIONS: usize = 1000;   // Fewer operations for complex math

/// Hardware timer-based performance measurement using SysTick
struct PerformanceTimer {
    syst: SYST,
}

impl PerformanceTimer {
    fn new(mut syst: SYST) -> Self {
        // Configure SysTick for maximum frequency
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(0xFF_FF_FF); // Maximum reload value for 24-bit counter
        syst.clear_current();
        syst.enable_counter();
        
        Self { syst }
    }
    
    /// Time complex mathematical operations
    fn time_complex_operations<F>(&mut self, mut operation: F, name: &str) -> u32 
    where 
        F: FnMut() -> f32,
    {
        rprintln!("Timing {} complex operations...", name);
        
        // Clear and start timing
        self.syst.clear_current();
        let start_ticks = SYST::get_current();
        
        // Execute complex operations
        let mut result = 0.0f32;
        for i in 0..COMPLEX_OPERATIONS {
            let input = 0.1f32 + (i as f32) * 0.01f32; // Varying input
            result += operation();
            
            // Prevent compiler from optimizing away the computation
            cortex_m::asm::nop();
        }
        
        // Stop timing
        let end_ticks = SYST::get_current();
        
        // Calculate elapsed ticks (SysTick counts down)
        let elapsed = if start_ticks >= end_ticks {
            start_ticks - end_ticks
        } else {
            // Handle counter wraparound
            (0xFF_FF_FF - end_ticks) + start_ticks
        };
        
        rprintln!("  {} operations completed, result checksum: {}", 
                 COMPLEX_OPERATIONS, result as u32);
        rprintln!("  Elapsed SysTick cycles: {}", elapsed);
        
        elapsed
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("qfplib Complex Math Performance Test");
    rprintln!("====================================");
    rprintln!("Testing operations where qfplib should excel:");
    rprintln!("- Trigonometric functions (sin, cos, tan)");
    rprintln!("- Exponential and logarithmic functions");
    rprintln!("- Complex mathematical combinations");
    rprintln!("");

    // Get the core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut timer = PerformanceTimer::new(cp.SYST);
    
    rprintln!("Each test performs {} complex operations", COMPLEX_OPERATIONS);
    rprintln!("Using SysTick hardware timer for accurate measurement");
    rprintln!("");

    // Test micromath complex operations
    rprintln!("=== Testing micromath complex operations ===");
    run_micromath_complex_test(&mut timer);
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("");
        rprintln!("=== Testing qfplib complex operations ===");
        run_qfplib_complex_test(&mut timer);
        
        rprintln!("");
        rprintln!("=== COMPLEX MATH PERFORMANCE COMPARISON ===");
        rprintln!("IMPORTANT: qfplib should show significant advantages for:");
        rprintln!("- Trigonometric functions (sin, cos, tan)");
        rprintln!("- Exponential and logarithmic operations");
        rprintln!("- Complex mathematical expressions");
        rprintln!("Lower SysTick cycles = better performance");
    }

    rprintln!("");
    rprintln!("Complex math performance testing complete!");
    
    #[cfg(feature = "qfplib")]
    rprintln!("Compare the cycle counts above to see qfplib's advantages!");
    
    #[cfg(not(feature = "qfplib"))]
    rprintln!("Build with --features qfplib to compare against optimized qfplib.");

    // Small delay to ensure all RTT output is transmitted
    for _ in 0..1000 {
        cortex_m::asm::nop();
    }

    rprintln!("Test completed successfully - entering low power mode");

    loop {
        cortex_m::asm::wfi();
    }
}

fn run_micromath_complex_test(timer: &mut PerformanceTimer) {
    use micromath::F32Ext;
    
    let base_val = 1.5f32;
    
    // Test trigonometric functions - where qfplib should excel
    let sin_cycles = timer.time_complex_operations(|| {
        base_val.sin()
    }, "micromath sin");
    
    let cos_cycles = timer.time_complex_operations(|| {
        base_val.cos()
    }, "micromath cos");
    
    let tan_cycles = timer.time_complex_operations(|| {
        base_val.tan()
    }, "micromath tan");
    
    // Test exponential functions - qfplib's strong suit
    let exp_cycles = timer.time_complex_operations(|| {
        base_val.exp()
    }, "micromath exp");
    
    let ln_cycles = timer.time_complex_operations(|| {
        (base_val + 1.0).ln()
    }, "micromath ln");
    
    // Test complex mathematical expression
    let complex_cycles = timer.time_complex_operations(|| {
        // Energy calculation-like expression
        let angle = base_val * 0.5;
        let power_factor = angle.cos();
        let reactive = angle.sin();
        let magnitude = (power_factor * power_factor + reactive * reactive).sqrt();
        magnitude
    }, "micromath complex expression");
    
    rprintln!("Micromath complex math performance (SysTick cycles):");
    rprintln!("  sin:     {} cycles", sin_cycles);
    rprintln!("  cos:     {} cycles", cos_cycles);
    rprintln!("  tan:     {} cycles", tan_cycles);
    rprintln!("  exp:     {} cycles", exp_cycles);
    rprintln!("  ln:      {} cycles", ln_cycles);
    rprintln!("  complex: {} cycles", complex_cycles);
}

#[cfg(feature = "qfplib")]
fn run_qfplib_complex_test(timer: &mut PerformanceTimer) {
    let base_val = 1.5f32;
    
    // Test trigonometric functions with qfplib - should be much faster
    let sin_cycles = timer.time_complex_operations(|| {
        base_val.fast_sin()
    }, "qfplib fast_sin");
    
    let cos_cycles = timer.time_complex_operations(|| {
        base_val.fast_cos()
    }, "qfplib fast_cos");
    
    let tan_cycles = timer.time_complex_operations(|| {
        base_val.fast_tan()
    }, "qfplib fast_tan");
    
    // Test exponential functions with qfplib - should show major improvements
    let exp_cycles = timer.time_complex_operations(|| {
        base_val.fast_exp()
    }, "qfplib fast_exp");
    
    let ln_cycles = timer.time_complex_operations(|| {
        (base_val + 1.0).fast_ln()
    }, "qfplib fast_ln");
    
    // Test complex mathematical expression with qfplib
    let complex_cycles = timer.time_complex_operations(|| {
        // Same energy calculation-like expression using qfplib
        let angle = base_val.fast_mul(0.5);
        let power_factor = angle.fast_cos();
        let reactive = angle.fast_sin();
        let magnitude = power_factor.fast_mul(power_factor)
            .fast_add(reactive.fast_mul(reactive))
            .fast_sqrt();
        magnitude
    }, "qfplib complex expression");
    
    rprintln!("qfplib complex math performance (SysTick cycles):");
    rprintln!("  sin:     {} cycles", sin_cycles);
    rprintln!("  cos:     {} cycles", cos_cycles);
    rprintln!("  tan:     {} cycles", tan_cycles);
    rprintln!("  exp:     {} cycles", exp_cycles);
    rprintln!("  ln:      {} cycles", ln_cycles);
    rprintln!("  complex: {} cycles", complex_cycles);
    
    rprintln!("");
    rprintln!("Expected results:");
    rprintln!("- qfplib trigonometric functions should be 2-5x faster");
    rprintln!("- qfplib exponential/logarithmic should be 3-10x faster");
    rprintln!("- Complex expressions should show overall improvement");
}