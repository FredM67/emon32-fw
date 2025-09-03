//! UART Hardware Demo for SAMD21
//! 
//! Demonstrates real UART output on Arduino Zero hardware
//! Using PA14 (TX) and PA15 (RX) - Arduino Zero pins 2 and 5

#![no_std]
#![no_main]

use panic_halt as _; // Panic handler

use cortex_m_rt::entry;
use atsamd_hal::{
    clock::GenericClockController,
    gpio::Pins,
};
use atsamd21j as pac; // Peripheral Access Crate

// Local modules
mod board;
mod energy;
mod math;
mod uart;

use energy::{EnergyCalculator, SampleBuffer};
use uart::UartOutput;

#[entry]
fn main() -> ! {
    // Initialize peripherals
    let mut peripherals = pac::Peripherals::take().unwrap();
    let pins = Pins::new(peripherals.port);
    
    // Clock configuration
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    // Configure UART pins
    let pa14 = pins.pa14; // TX - Arduino Zero pin 2
    let pa15 = pins.pa15; // RX - Arduino Zero pin 5
    
    // Initialize UART output with hardware implementation
    let mut uart_output = UartOutput::new_hardware(
        peripherals.sercom2,
        pa14,
        pa15,
        &mut clocks,
        &mut peripherals.pm,
    );

    // Send startup banner
    uart_output.send_banner();

    // Initialize energy calculator
    let mut energy_calc = EnergyCalculator::new();
    let mut sample_counter = 0u32;

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
        let timestamp_ms = sample_counter * 100; // Simulate 100ms intervals
        if let Some(power_data) = energy_calc.process_samples(&samples, timestamp_ms) {
            // Send energy data via UART
            uart_output.maybe_output(&power_data, timestamp_ms);
        }

        sample_counter = sample_counter.wrapping_add(1);

        // Simple delay loop (in real implementation, this would be timer-driven)
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
    }
}

/// Generate test samples for demonstration
/// This simulates ADC readings from voltage/current sensors
fn generate_test_sample(sample_index: u32) -> u16 {
    use micromath::F32Ext;
    
    // Simulate 230V RMS voltage and varying current
    let time = sample_index as f32 * 0.001; // Simulate 1kHz sampling
    let frequency = 50.0; // 50Hz mains frequency
    let phase = 2.0 * 3.14159 * frequency * time;
    
    // Voltage channel: 230V RMS sine wave
    let voltage = 230.0 * 1.414 * phase.sin(); // Peak voltage
    let voltage_sample = ((voltage + 400.0) * 16.0) as u16; // Scale to ADC range
    
    // Simulate different samples for voltage/current channels
    match sample_index % 4 {
        0 => voltage_sample, // Voltage channel
        1 => {
            // Current channel 1: 0.65A RMS (150W @ 230V)
            let current = 0.65 * 1.414 * phase.sin();
            ((current + 2.5) * 819.2) as u16
        },
        2 => {
            // Current channel 2: 0.33A RMS (75W @ 230V)  
            let current = 0.33 * 1.414 * (phase + 0.2).sin(); // Slight phase shift
            ((current + 2.5) * 819.2) as u16
        },
        3 => {
            // Current channel 3: 0A (no load)
            (2.5 * 819.2) as u16 // Zero current = mid-scale ADC
        },
        _ => 0,
    }
}