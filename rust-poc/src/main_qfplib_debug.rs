//! Debug test to verify qfplib is actually being called
//! This will test a single operation and report if qfplib or fallback is used

#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use emon32_rust_poc::math::FastMath;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("qfplib Debug Test");
    rprintln!("=================");
    
    rprintln!("Target architecture: ARM (thumbv6m-none-eabi)");
    
    #[cfg(feature = "qfplib")]
    rprintln!("qfplib feature: ENABLED");
    
    #[cfg(not(feature = "qfplib"))]
    rprintln!("qfplib feature: DISABLED");
    
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    rprintln!("Condition met: ARM + qfplib = qfplib functions should be called");
    
    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    rprintln!("Condition NOT met: falling back to micromath/std");
    
    rprintln!("");
    
    let test_val = 123.456f32;
    
    // Test sqrt with explicit debugging
    rprintln!("Testing sqrt({}):", test_val);
    
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    {
        rprintln!("  Calling qfplib qfp_fsqrt directly...");
        let direct_result = unsafe { 
            extern "C" { fn qfp_fsqrt(x: f32) -> f32; }
            qfp_fsqrt(test_val) 
        };
        rprintln!("  Direct qfplib result: {}", direct_result);
    }
    
    rprintln!("  Calling via FastMath trait...");
    let trait_result = test_val.fast_sqrt();
    rprintln!("  FastMath trait result: {}", trait_result);
    
    rprintln!("  Calling micromath directly...");
    use micromath::F32Ext;
    let micromath_result = test_val.sqrt();
    rprintln!("  Micromath result: {}", micromath_result);
    
    rprintln!("");
    rprintln!("Results comparison:");
    rprintln!("  FastMath:  {}", trait_result);
    rprintln!("  Micromath: {}", micromath_result);
    
    if (trait_result - micromath_result).abs() < 1e-6 {
        rprintln!("  WARNING: Results are identical - qfplib may not be working!");
    } else {
        rprintln!("  GOOD: Results differ - qfplib is likely working correctly");
    }
    
    rprintln!("");
    rprintln!("Debug test complete!");
    
    loop {
        cortex_m::asm::wfi();
    }
}