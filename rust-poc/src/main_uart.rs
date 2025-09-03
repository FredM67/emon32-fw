//! emon32 Rust POC with UART Serial Output
//! 
//! This demonstrates the complete energy monitoring system with:
//! - Energy calculation algorithms
//! - UART output at 115200 baud (via RTT for demo)
//! - Format: "1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W"

#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
use panic_halt as _; // Panic handler

use cortex_m_rt::entry;

// Local modules
use emon32_rust_poc::{
    energy::{EnergyCalculator, SampleBuffer},
    uart::UartOutput,
};

#[entry]
fn main() -> ! {
    // Initialize RTT for output if available
    #[cfg(feature = "rtt")]
    {
        use rtt_target::rtt_init_print;
        rtt_init_print!();
    }
    
    // Setup UART for serial output (simplified for demo)
    let mut uart_output = UartOutput::new();
    
    // Send startup banner
    uart_output.send_banner();
    uart_output.send_status("Initializing energy calculator...");
    
    // Initialize energy calculator
    let mut energy_calc = EnergyCalculator::new();
    let mut sample_counter = 0u32;
    
    uart_output.send_status("Starting energy monitoring...");
    
    loop {
        // Simulate ADC sampling and energy calculation
        let mut samples = SampleBuffer::new();
        
        // Generate test samples (simulating real ADC data)
        for i in 0..60 {
            let sample_value = generate_test_sample(sample_counter + i);
            if samples.push(sample_value).is_err() {
                break;
            }
        }
        
        // Process samples with energy calculator
        let timestamp_ms = sample_counter * 1000; // 1 second intervals for demo
        if let Some(power_data) = energy_calc.process_samples(&samples, timestamp_ms) {
            // Output energy data via UART in the specified format
            uart_output.maybe_output(&power_data, timestamp_ms);
        }
        
        sample_counter = sample_counter.wrapping_add(1);
        
        // Delay to simulate realistic timing (1 second intervals)
        delay_ms(1000);
    }
}

/// Generate test ADC samples (simulating real measurements)
fn generate_test_sample(counter: u32) -> u16 {
    use micromath::F32Ext;
    
    // Generate realistic energy monitoring test pattern
    let time = (counter as f32) * 0.02; // 50Hz simulation
    
    // Simulate different channels
    let channel = (counter as usize) % 15; // 15 channels total (3V + 12CT)
    
    if channel < 3 {
        // Voltage channels - simulate 230V RMS
        let amplitude = 600.0; // ADC amplitude for ~230V
        let offset = 2048.0;    // ADC midpoint
        let voltage_sample = offset + amplitude * (time * 2.0 * core::f32::consts::PI).sin();
        voltage_sample.max(0.0).min(4095.0) as u16
    } else {
        // Current channels - simulate different loads
        let ct_channel = channel - 3;
        let base_currents = [10.0, 5.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]; // 12 CT channels
        let current_amplitude = base_currents[ct_channel] * 20.0; // Scale for ADC
        let offset = 2048.0;
        
        // Add some phase shift for realistic power factor
        let phase_shift = ct_channel as f32 * 0.1;
        let current_sample = offset + current_amplitude * (time * 2.0 * core::f32::consts::PI + phase_shift).sin();
        current_sample.max(0.0).min(4095.0) as u16
    }
}

/// Simple delay function (blocking)
fn delay_ms(ms: u32) {
    // At 48MHz, approximately 48,000 cycles per millisecond
    for _ in 0..ms {
        for _ in 0..48_000 {
            cortex_m::asm::nop();
        }
    }
}