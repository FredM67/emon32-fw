//! Comprehensive qfplib vs micromath Performance Test for ARM Cortex-M0+
//! 
//! This test provides detailed performance comparison using multiple measurement
//! strategies to accurately assess the performance characteristics of both libraries.

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use rtt_target::{rprintln, rtt_init_print};

#[cfg(feature = "qfplib")]
use emon32_rust_poc::math::FastMath;

#[cfg(feature = "qfplib")]
use qfplib_sys;

use micromath::F32Ext;

const BATCH_ITERATIONS: usize = 1000;   // For batch testing
const WARMUP_ITERATIONS: usize = 10;    // Cache warmup

// Global variable to prevent optimization
static mut RESULT_SINK: f32 = 0.0;

/// Enhanced hardware timer-based performance measurement
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
    
    /// Time a single operation with minimal overhead
    fn time_single_operation<F>(&mut self, operation: F, name: &str) -> u32 
    where 
        F: FnOnce() -> f32,
    {
        // Warm up the pipeline
        for _ in 0..WARMUP_ITERATIONS {
            cortex_m::asm::nop();
        }
        
        // Clear any pending interrupts and ensure clean timing
        cortex_m::interrupt::free(|_| {
            // Time just one operation
            self.syst.clear_current();
            let start_ticks = SYST::get_current();
            
            let result = operation();
            
            // Force memory barrier to prevent reordering
            cortex_m::asm::dmb();
            let end_ticks = SYST::get_current();
            
            // Prevent optimization by storing result in global variable
            unsafe { 
                RESULT_SINK = result;
                // Also use volatile write to ensure it's not optimized away
                core::ptr::write_volatile(&mut RESULT_SINK, result);
            }
            
            let elapsed = if start_ticks >= end_ticks {
                start_ticks - end_ticks
            } else {
                (0xFF_FF_FF - end_ticks) + start_ticks
            };
            
            rprintln!("  {}: {} cycles (result: {:.6})", name, elapsed, result);
            elapsed
        })
    }
    
    /// Time batch operations to amortize overhead
    fn time_batch_operations<F>(&mut self, mut operation: F, name: &str) -> u32 
    where 
        F: FnMut() -> f32,
    {
        // Warmup
        for _ in 0..WARMUP_ITERATIONS {
            let result = operation();
            unsafe { RESULT_SINK = result; }
        }
        
        cortex_m::interrupt::free(|_| {
            self.syst.clear_current();
            let start_ticks = SYST::get_current();
            
            let mut result = 0.0f32;
            for _ in 0..BATCH_ITERATIONS {
                result += operation();
                // Add small perturbation to prevent optimization
                unsafe { RESULT_SINK = result; }
            }
            
            cortex_m::asm::dmb();
            let end_ticks = SYST::get_current();
            
            // Prevent optimization
            unsafe { 
                core::ptr::write_volatile(&mut RESULT_SINK, result);
            }
            
            let elapsed = if start_ticks >= end_ticks {
                start_ticks - end_ticks
            } else {
                (0xFF_FF_FF - end_ticks) + start_ticks
            };
            
            let avg_cycles = elapsed / BATCH_ITERATIONS as u32;
            rprintln!("  {} (batch avg): {} cycles (sum: {:.3})", name, avg_cycles, result);
            avg_cycles
        })
    }
    
    /// Time operations on arrays to test batch efficiency
    fn time_array_operations<F>(&mut self, mut operation: F, name: &str) -> u32 
    where 
        F: FnMut(&[f32], &mut [f32]),
    {
        const ARRAY_SIZE: usize = 100;
        let mut input = [0.0f32; ARRAY_SIZE];
        let mut output = [0.0f32; ARRAY_SIZE];
        
        // Initialize test data with varying values to prevent optimization
        for (i, val) in input.iter_mut().enumerate() {
            *val = 1.0 + (i as f32) * 0.01 + 0.123; // Avoid simple patterns
        }
        
        // Warmup
        for _ in 0..5 {
            operation(&input, &mut output);
            unsafe { RESULT_SINK = output[0]; }
        }
        
        cortex_m::interrupt::free(|_| {
            self.syst.clear_current();
            let start_ticks = SYST::get_current();
            
            operation(&input, &mut output);
            
            cortex_m::asm::dmb();
            let end_ticks = SYST::get_current();
            
            // Prevent optimization by using multiple results
            unsafe { 
                RESULT_SINK = output[0] + output[50] + output[99];
                core::ptr::write_volatile(&mut RESULT_SINK, RESULT_SINK);
            }
            
            let elapsed = if start_ticks >= end_ticks {
                start_ticks - end_ticks
            } else {
                (0xFF_FF_FF - end_ticks) + start_ticks
            };
            
            let avg_cycles = elapsed / ARRAY_SIZE as u32;
            rprintln!("  {} (array avg): {} cycles (checksum: {:.3})", name, avg_cycles, output[0] + output[50]);
            avg_cycles
        })
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("=== Comprehensive Math Library Performance Test ===");
    rprintln!("Hardware: ARM Cortex-M0+ (ATSAMD21J17A)");
    rprintln!("Timer: SysTick hardware counter");
    rprintln!("Anti-optimization measures: Global sink, memory barriers, varying inputs");
    rprintln!("");

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut timer = PerformanceTimer::new(cp.SYST);
    
    // Add LTO effectiveness test first
    test_lto_effectiveness(&mut timer);

    // Test different operation categories
    test_simple_operations(&mut timer);
    test_medium_complexity_operations(&mut timer);
    test_complex_operations(&mut timer);
    test_batch_operations(&mut timer);
    test_array_operations(&mut timer);
    
    rprintln!("");
    rprintln!("=== ANALYSIS & RECOMMENDATIONS ===");
    print_analysis();
    
    rprintln!("");
    rprintln!("Performance testing complete!");
    
    // Show final result to ensure nothing was optimized away
    unsafe {
        rprintln!("Final result sink value: {:.6}", RESULT_SINK);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

fn test_lto_effectiveness(timer: &mut PerformanceTimer) {
    rprintln!("=== LTO EFFECTIVENESS TEST ===");
    rprintln!("Measuring FFI overhead reduction...");
    
    let test_val = 1.5707963f32;
    
    // Test the same operation with both libraries
    rprintln!("Testing sin() function:");
    let micro_sin = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        micromath::F32Ext::sin(input)
    }, "micromath sin");
    
    #[cfg(feature = "qfplib")]
    {
        let qfp_sin = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            // Call qfplib directly, not through FastMath trait (which uses micromath for sin)
            qfplib_sys::LtoOptimized::sin(input)
        }, "qfplib sin (LTO)");
        
        let overhead = if qfp_sin > micro_sin { 
            qfp_sin - micro_sin 
        } else { 
            0 
        };
        
        rprintln!("Performance comparison:");
        rprintln!("  micromath sin: {} cycles", micro_sin);
        rprintln!("  qfplib sin:    {} cycles", qfp_sin);
        rprintln!("  FFI overhead:  {} cycles", overhead);
        
        if overhead < 10 {
            rprintln!("✅ LTO successfully eliminated most FFI overhead!");
        } else if overhead < 30 {
            rprintln!("⚠️  LTO partially effective, {} cycles overhead remaining", overhead);
        } else {
            rprintln!("❌ LTO not effective, {} cycles overhead remaining", overhead);
            rprintln!("   Check LTO settings and inline attributes");
        }
        
        // Test a simpler operation too
        rprintln!("");
        rprintln!("Testing sqrt() function:");
        let micro_sqrt = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            micromath::F32Ext::sqrt(input)
        }, "micromath sqrt");
        
        let qfp_sqrt = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::sqrt(input)
        }, "qfplib sqrt (LTO)");
        
        let sqrt_overhead = if qfp_sqrt > micro_sqrt { 
            qfp_sqrt - micro_sqrt 
        } else { 
            0 
        };
        
        rprintln!("  micromath sqrt: {} cycles", micro_sqrt);
        rprintln!("  qfplib sqrt:    {} cycles", qfp_sqrt);
        rprintln!("  FFI overhead:   {} cycles", sqrt_overhead);
    }
    
    #[cfg(not(feature = "qfplib"))]
    {
        rprintln!("qfplib not available - build with --features qfplib to test LTO");
        rprintln!("micromath sin: {} cycles", micro_sin);
    }
    
    rprintln!("");
}

fn test_simple_operations(timer: &mut PerformanceTimer) {
    rprintln!("=== SIMPLE OPERATIONS TEST ===");
    rprintln!("Testing basic arithmetic operations...");
    
    // Use varying inputs to prevent constant folding
    let test_val = 123.456f32;
    let operand = 2.345f32;
    
    // Micromath tests
    rprintln!("Micromath:");
    let micro_mul = timer.time_single_operation(|| {
        // Use slightly varying input to prevent optimization
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input * operand
    }, "multiply");
    
    let micro_div = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input / operand
    }, "divide");
    
    let micro_add = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input + operand
    }, "add");
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("qfplib:");
        let qfp_mul = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::mul(input, operand)
        }, "multiply");
        
        let qfp_div = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::div(input, operand)
        }, "divide");
        
        let qfp_add = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            input + operand
        }, "add (native)");
        
        rprintln!("Simple Operations Summary:");
        rprintln!("  Multiply - micromath: {} vs qfplib: {} cycles", micro_mul, qfp_mul);
        rprintln!("  Divide   - micromath: {} vs qfplib: {} cycles", micro_div, qfp_div);
        rprintln!("  Add      - micromath: {} vs qfplib: {} cycles", micro_add, qfp_add);
    }
    
    #[cfg(not(feature = "qfplib"))]
    {
        rprintln!("qfplib: Not available (build with --features qfplib)");
        rprintln!("Simple Operations Summary:");
        rprintln!("  Multiply - micromath: {} cycles", micro_mul);
        rprintln!("  Divide   - micromath: {} cycles", micro_div);
        rprintln!("  Add      - micromath: {} cycles", micro_add);
    }
    
    rprintln!("");
}

fn test_medium_complexity_operations(timer: &mut PerformanceTimer) {
    rprintln!("=== MEDIUM COMPLEXITY OPERATIONS TEST ===");
    rprintln!("Testing square root operations...");
    
    let test_val = 123.456f32;
    
    // Micromath tests
    rprintln!("Micromath:");
    let micro_sqrt = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input.sqrt()
    }, "sqrt");
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("qfplib:");
        let qfp_sqrt = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::sqrt(input)
        }, "sqrt");
        
        rprintln!("Medium Complexity Summary:");
        rprintln!("  Sqrt - micromath: {} vs qfplib: {} cycles", micro_sqrt, qfp_sqrt);
    }
    
    #[cfg(not(feature = "qfplib"))]
    {
        rprintln!("qfplib: Not available");
        rprintln!("Medium Complexity Summary:");
        rprintln!("  Sqrt - micromath: {} cycles", micro_sqrt);
    }
    
    rprintln!("");
}

fn test_complex_operations(timer: &mut PerformanceTimer) {
    rprintln!("=== COMPLEX OPERATIONS TEST ===");
    rprintln!("Testing transcendental functions...");
    
    let test_val = 1.5707963f32; // π/2
    
    // Micromath tests
    rprintln!("Micromath:");
    let micro_sin = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input.sin()
    }, "sin");
    
    let micro_cos = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input.cos()
    }, "cos");
    
    let micro_exp = timer.time_single_operation(|| {
        let input = (test_val * 0.5) + unsafe { RESULT_SINK } * 0.001; // Smaller input for exp
        input.exp()
    }, "exp");
    
    let micro_ln = timer.time_single_operation(|| {
        let input = test_val + unsafe { RESULT_SINK } * 0.001;
        input.ln()
    }, "ln");
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("qfplib:");
        let qfp_sin = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::sin(input)
        }, "sin");
        
        let qfp_cos = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::cos(input)
        }, "cos");
        
        let qfp_exp = timer.time_single_operation(|| {
            let input = (test_val * 0.5) + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::exp(input)
        }, "exp");
        
        let qfp_ln = timer.time_single_operation(|| {
            let input = test_val + unsafe { RESULT_SINK } * 0.001;
            qfplib_sys::LtoOptimized::ln(input)
        }, "ln");
        
        rprintln!("Complex Operations Summary:");
        rprintln!("  Sin - micromath: {} vs qfplib: {} cycles", micro_sin, qfp_sin);
        rprintln!("  Cos - micromath: {} vs qfplib: {} cycles", micro_cos, qfp_cos);
        rprintln!("  Exp - micromath: {} vs qfplib: {} cycles", micro_exp, qfp_exp);
        rprintln!("  Ln  - micromath: {} vs qfplib: {} cycles", micro_ln, qfp_ln);
    }
    
    #[cfg(not(feature = "qfplib"))]
    {
        rprintln!("qfplib: Not available");
        rprintln!("Complex Operations Summary:");
        rprintln!("  Sin - micromath: {} cycles", micro_sin);
        rprintln!("  Cos - micromath: {} cycles", micro_cos);
        rprintln!("  Exp - micromath: {} cycles", micro_exp);
        rprintln!("  Ln  - micromath: {} cycles", micro_ln);
    }
    
    rprintln!("");
}

fn test_batch_operations(timer: &mut PerformanceTimer) {
    rprintln!("=== BATCH OPERATIONS TEST ===");
    rprintln!("Testing amortized performance over {} iterations...", BATCH_ITERATIONS);
    
    let base_val = 1.5707963f32;
    
    rprintln!("Micromath (batch average):");
    let micro_sin_batch = timer.time_batch_operations(|| {
        let input = base_val + unsafe { RESULT_SINK } * 0.0001; // Vary input slightly
        input.sin()
    }, "sin");
    
    let micro_exp_batch = timer.time_batch_operations(|| {
        let input = (base_val * 0.5) + unsafe { RESULT_SINK } * 0.0001;
        input.exp()
    }, "exp");
    
    let micro_sqrt_batch = timer.time_batch_operations(|| {
        let input = base_val + unsafe { RESULT_SINK } * 0.0001;
        input.sqrt()
    }, "sqrt");
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("qfplib (batch average):");
        let qfp_sin_batch = timer.time_batch_operations(|| {
            let input = base_val + unsafe { RESULT_SINK } * 0.0001;
            qfplib_sys::LtoOptimized::sin(input)
        }, "sin");
        
        let qfp_exp_batch = timer.time_batch_operations(|| {
            let input = (base_val * 0.5) + unsafe { RESULT_SINK } * 0.0001;
            qfplib_sys::LtoOptimized::exp(input)
        }, "exp");
        
        let qfp_sqrt_batch = timer.time_batch_operations(|| {
            let input = base_val + unsafe { RESULT_SINK } * 0.0001;
            qfplib_sys::LtoOptimized::sqrt(input)
        }, "sqrt");
        
        rprintln!("Batch Operations Summary:");
        rprintln!("  Sin (batch)  - micromath: {} vs qfplib: {} cycles", micro_sin_batch, qfp_sin_batch);
        rprintln!("  Exp (batch)  - micromath: {} vs qfplib: {} cycles", micro_exp_batch, qfp_exp_batch);
        rprintln!("  Sqrt (batch) - micromath: {} vs qfplib: {} cycles", micro_sqrt_batch, qfp_sqrt_batch);
    }
    
    rprintln!("");
}

fn test_array_operations(timer: &mut PerformanceTimer) {
    rprintln!("=== ARRAY OPERATIONS TEST ===");
    rprintln!("Testing bulk processing efficiency...");
    
    // Micromath array operations
    rprintln!("Micromath (array processing):");
    timer.time_array_operations(|input, output| {
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = i.sin();
        }
    }, "sin array");
    
    timer.time_array_operations(|input, output| {
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = i.sqrt();
        }
    }, "sqrt array");
    
    timer.time_array_operations(|input, output| {
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = (*i * 0.5).exp(); // Scale down input for exp
        }
    }, "exp array");
    
    #[cfg(feature = "qfplib")]
    {
        rprintln!("qfplib (array processing):");
        timer.time_array_operations(|input, output| {
            for (i, o) in input.iter().zip(output.iter_mut()) {
                *o = qfplib_sys::LtoOptimized::sin(*i);
            }
        }, "sin array");
        
        timer.time_array_operations(|input, output| {
            for (i, o) in input.iter().zip(output.iter_mut()) {
                *o = qfplib_sys::LtoOptimized::sqrt(*i);
            }
        }, "sqrt array");
        
        timer.time_array_operations(|input, output| {
            for (i, o) in input.iter().zip(output.iter_mut()) {
                *o = qfplib_sys::LtoOptimized::exp(*i * 0.5);
            }
        }, "exp array");
    }
    
    rprintln!("");
}

fn print_analysis() {
    rprintln!("PERFORMANCE ANALYSIS:");
    rprintln!("");
    rprintln!("LTO EFFECTIVENESS:");
    rprintln!("- Good LTO: <10 cycles overhead");
    rprintln!("- Partial LTO: 10-30 cycles overhead");
    rprintln!("- Poor LTO: >30 cycles overhead");
    rprintln!("");
    rprintln!("EXPECTED PATTERNS WITH LTO:");
    rprintln!("- Simple ops: Still micromath advantage (inlining + no complex algorithms)");
    rprintln!("- Complex ops: qfplib should now show its true performance");
    rprintln!("- FFI overhead should be minimal (under 10 cycles)");
    rprintln!("");
    rprintln!("If LTO is working well, qfplib should be competitive or better");
    rprintln!("for complex operations like sin, cos, exp, ln.");
    rprintln!("");
    rprintln!("RECOMMENDATIONS:");
    rprintln!("- Use micromath for simple operations and embedded-friendly code");
    rprintln!("- Use qfplib for complex math when highest performance is needed");
    rprintln!("- Consider hybrid approach: micromath for simple, qfplib for complex");
    rprintln!("- For bulk processing, qfplib may show better amortized performance");
}
