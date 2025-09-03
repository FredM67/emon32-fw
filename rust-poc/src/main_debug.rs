//! Enhanced main.rs with oscilloscope debug pins
#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
use panic_halt as _; // Panic handler

// Import required crates
use atsamd_hal::{
    clock::GenericClockController,
    gpio::{Pin, Pins, PushPullOutput, PA14, PA15, PA17, PA21},
    pac::Peripherals,
    prelude::*,
};
use cortex_m_rt::entry;

// Local modules
mod board;
mod energy;
mod math;

use energy::{EnergyCalculator, SampleBuffer};
use math::FastMath;

// Debug pins for oscilloscope validation (Arduino Zero compatible)
struct DebugPins {
    adc_timing: Pin<PA14, PushPullOutput>, // Digital Pin 2 - ADC timing
    processing: Pin<PA15, PushPullOutput>, // Digital Pin 5 - Processing duration
    heartbeat: Pin<PA21, PushPullOutput>,  // Digital Pin 7 - System heartbeat
    status_led: Pin<PA17, PushPullOutput>, // Digital Pin 13 - Onboard LED
}

impl DebugPins {
    fn new(pins: Pins) -> Self {
        Self {
            adc_timing: pins.pa14.into_push_pull_output(), // Pin 2
            processing: pins.pa15.into_push_pull_output(), // Pin 5
            heartbeat: pins.pa21.into_push_pull_output(),  // Pin 7
            status_led: pins.pa17.into_push_pull_output(), // Pin 13 (LED)
        }
    }

    fn mark_adc_start(&mut self) {
        self.adc_timing.set_high().unwrap();
    }

    fn mark_adc_end(&mut self) {
        self.adc_timing.set_low().unwrap();
    }

    fn mark_processing_start(&mut self) {
        self.processing.set_high().unwrap();
    }

    fn mark_processing_end(&mut self) {
        self.processing.set_low().unwrap();
    }

    fn toggle_heartbeat(&mut self) {
        self.heartbeat.toggle().unwrap();
    }

    fn toggle_status_led(&mut self) {
        self.status_led.toggle().unwrap();
    }
}

#[entry]
fn main() -> ! {
    // Initialize hardware
    let mut peripherals = Peripherals::take().unwrap();
    let pins = Pins::new(peripherals.port);

    // Initialize clocks (basic setup)
    let _clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    // Setup debug pins for oscilloscope
    let mut debug_pins = DebugPins::new(pins);

    // Initialize energy calculator
    let mut energy_calc = EnergyCalculator::new();
    let mut sample_counter = 0u32;
    let mut heartbeat_counter = 0u32;

    loop {
        // === OSCILLOSCOPE MARKER: ADC Sampling Start ===
        debug_pins.mark_adc_start();

        // Simulate ADC sampling
        let mut samples = SampleBuffer::new();

        // Generate test samples (simulating real ADC data)
        for i in 0..60 {
            let sample_value = generate_test_sample(sample_counter + i);
            if samples.push(sample_value).is_err() {
                break;
            }
        }

        // === OSCILLOSCOPE MARKER: ADC Sampling End ===
        debug_pins.mark_adc_end();

        // === OSCILLOSCOPE MARKER: Processing Start ===
        debug_pins.mark_processing_start();

        // Process samples with energy calculator
        let timestamp_ms = sample_counter * 100; // Simulate 100ms intervals
        if let Some(_power_data) = energy_calc.process_samples(&samples, timestamp_ms) {
            // In a real implementation, this would be sent via UART
            // For POC with oscilloscope validation, we just continue
        }

        // === OSCILLOSCOPE MARKER: Processing End ===
        debug_pins.mark_processing_end();

        sample_counter = sample_counter.wrapping_add(1);
        heartbeat_counter = heartbeat_counter.wrapping_add(1);

        // Heartbeat every 100 cycles (~10 seconds at 10Hz)
        if heartbeat_counter >= 100 {
            debug_pins.toggle_heartbeat();
            debug_pins.toggle_status_led(); // Arduino Zero onboard LED
            heartbeat_counter = 0;
        }

        // === TIMING VALIDATION: Controlled delay ===
        // This creates a known sample rate for oscilloscope measurement
        // At 4800 Hz: period = 208.33μs
        // Delay calculation: 208μs - processing_time ≈ 200μs
        delay_microseconds_approx(200);
    }
}

// Generate test ADC samples (simulating real measurements)
fn generate_test_sample(counter: u32) -> u16 {
    use micromath::F32Ext;

    // Generate a sine wave pattern for testing
    let phase = (counter as f32) * 0.1;
    let amplitude = 500.0; // Simulate ADC range
    let offset = 2048; // ADC midpoint for 12-bit

    let sample = offset as f32 + amplitude * phase.sin();
    sample.max(0.0).min(4095.0) as u16
}

// Approximate microsecond delay (depends on CPU frequency)
// For SAMD21 at 48MHz: 48 cycles ≈ 1μs
fn delay_microseconds_approx(us: u32) {
    let cycles = us * 48; // Approximate for 48MHz
    for _ in 0..cycles {
        cortex_m::asm::nop();
    }
}
