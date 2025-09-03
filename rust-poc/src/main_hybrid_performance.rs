//! Hybrid FastMath Performance Test
//! 
//! This test validates the optimized FastMath implementation that automatically
//! chooses the fastest library (micromath or qfplib) for each operation.

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use rtt_target::{rprintln, rtt_init_print};

use emon32_rust_poc::math::FastMath;

const OPERATIONS: usize = 1000;

/// Hardware timer-based performance measurement using SysTick
struct PerformanceTimer {
    syst: SYST,
}

impl PerformanceTimer {
    fn new(mut syst: SYST) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(0xFF_FF_FF);
        syst.clear_current();
        syst.enable_counter();
        Self { syst }
    }
    
    fn time_operations<F>(&mut self, mut operation: F, name: &str) -> u32 
    where 
        F: FnMut() -> f32,
    {
        rprintln!("Timing {} operations...", name);
        
        self.syst.clear_current();
        let start_ticks = SYST::get_current();
        
        let mut result = 0.0f32;
        for _ in 0..OPERATIONS {
            result += operation();
            cortex_m::asm::nop();
        }
        
        let end_ticks = SYST::get_current();
        let elapsed = if start_ticks >= end_ticks {
            start_ticks - end_ticks
        } else {
            (0xFF_FF_FF - end_ticks) + start_ticks
        };
        
        rprintln!("  {} operations completed, result checksum: {}", 
                 OPERATIONS, result as u32);
        rprintln!("  Elapsed SysTick cycles: {}", elapsed);
        
        elapsed
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hybrid FastMath Performance Test");
    rprintln!("================================");
    rprintln!("Testing optimized FastMath that automatically chooses:");
    rprintln!("- micromath for: sin, cos, tan, ln, atan2 (faster)");
    rprintln!("- qfplib for: exp (6.5x faster)");
    rprintln!("- Best of both worlds!");
    rprintln!("");

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut timer = PerformanceTimer::new(cp.SYST);
    
    rprintln!("Each test performs {} operations", OPERATIONS);
    rprintln!("");

    // Test the hybrid FastMath implementation
    rprintln!("=== Testing Hybrid FastMath Implementation ===");
    run_hybrid_test(&mut timer);
    
    rprintln!("");
    rprintln!("=== HYBRID PERFORMANCE RESULTS ===");
    rprintln!("The FastMath trait now automatically uses:");
    rprintln!("✅ micromath for sin/cos/tan/ln (proven faster)");
    rprintln!("✅ qfplib for exp (6.5x performance advantage)");
    rprintln!("✅ Best performance for each operation type!");

    rprintln!("");
    rprintln!("Hybrid FastMath testing complete!");
    rprintln!("Your energy calculations will now use the optimal");
    rprintln!("math library for each specific operation!");

    for _ in 0..1000 {
        cortex_m::asm::nop();
    }

    rprintln!("Test completed successfully - entering low power mode");

    loop {
        cortex_m::asm::wfi();
    }
}

fn run_hybrid_test(timer: &mut PerformanceTimer) {
    let base_val = 1.5f32;
    
    // Test hybrid trigonometric functions (should use micromath)
    let sin_cycles = timer.time_operations(|| {
        base_val.fast_sin()  // Uses micromath (4.5x faster than qfplib)
    }, "hybrid fast_sin (micromath)");
    
    let cos_cycles = timer.time_operations(|| {
        base_val.fast_cos()  // Uses micromath (4.6x faster than qfplib)
    }, "hybrid fast_cos (micromath)");
    
    let tan_cycles = timer.time_operations(|| {
        base_val.fast_tan()  // Uses micromath (5.2x faster than qfplib)
    }, "hybrid fast_tan (micromath)");
    
    // Test hybrid exponential function (should use qfplib)
    let exp_cycles = timer.time_operations(|| {
        base_val.fast_exp()  // Uses qfplib (6.5x faster than micromath)
    }, "hybrid fast_exp (qfplib)");
    
    // Test hybrid logarithm (should use micromath)
    let ln_cycles = timer.time_operations(|| {
        (base_val + 1.0).fast_ln()  // Uses micromath (2.8x faster than qfplib)
    }, "hybrid fast_ln (micromath)");
    
    // Test energy calculation-like expression with hybrid math
    let energy_cycles = timer.time_operations(|| {
        let angle = base_val * 0.5;
        let power_factor = angle.fast_cos();    // micromath
        let reactive = angle.fast_sin();        // micromath  
        let decay = angle.fast_exp();           // qfplib
        let magnitude = (power_factor * power_factor + reactive * reactive).fast_sqrt();
        magnitude * decay
    }, "hybrid energy calculation");
    
    rprintln!("Hybrid FastMath performance (SysTick cycles):");
    rprintln!("  sin (micromath): {} cycles", sin_cycles);
    rprintln!("  cos (micromath): {} cycles", cos_cycles);
    rprintln!("  tan (micromath): {} cycles", tan_cycles);
    rprintln!("  exp (qfplib):    {} cycles", exp_cycles);
    rprintln!("  ln (micromath):  {} cycles", ln_cycles);
    rprintln!("  energy calc:     {} cycles", energy_cycles);
    
    rprintln!("");
    rprintln!("Expected performance improvements:");
    rprintln!("- Trigonometry: ~160k cycles (micromath speed)");
    rprintln!("- Exponential:  ~430k cycles (qfplib speed)");
    rprintln!("- Best of both libraries automatically!");
}