// Example: Energy calculation simulation
// This can run on a development machine to test the algorithms

#![no_std]
#![no_main]

use emon32_rust_poc::energy::EnergyCalculator;
use heapless::Vec;
use micromath::F32Ext;
use core::iter::Iterator;
use panic_halt as _;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut calc = EnergyCalculator::new();
    
    // Set some realistic calibration values
    calc.set_voltage_calibration(0, 8.087);  // Voltage calibration from C version
    calc.set_current_calibration(0, 3.0);    // Current calibration from C version
    
    // Simulate some measurement cycles
    for cycle in 0..10 {
        let mut samples = Vec::<u16, 128>::new();
        
        // Generate test data simulating real electrical measurements
        // This simulates 3 voltage channels + 6 current channels
        for sample_idx in 0..90 {  // 10 samples per channel
            let channel = sample_idx % 9;
            let time_phase = (sample_idx / 9) as f32 * 0.628; // ~36 degrees per sample
            
            let sample_value = if channel < 3 {
                // Voltage channels: simulate 230V RMS at 50Hz
                let amplitude = 400.0;  // Scaled for ADC
                let offset = 2048.0;    // ADC midpoint
                let voltage_sample: f32 = offset + amplitude * (time_phase + cycle as f32 * 0.1).sin();
                voltage_sample.max(0.0).min(4095.0) as u16
            } else {
                // Current channels: simulate different loads
                let ct_channel = channel - 3;
                let base_current = [100.0, 50.0, 25.0, 10.0, 5.0, 0.0][ct_channel];
                let amplitude = base_current * 2.0;  // Peak current
                let offset = 2048.0;
                
                // Add some phase shift for power factor simulation
                let phase_shift = ct_channel as f32 * 0.2;
                let current_sample: f32 = offset + amplitude * (time_phase + phase_shift).sin();
                current_sample.max(0.0).min(4095.0) as u16
            };
            
            if samples.push(sample_value).is_err() {
                break;
            }
        }
        
        // Process the samples
        let timestamp = cycle * 1000;  // 1 second per cycle
        if let Some(power_data) = calc.process_samples(&samples, timestamp) {
            // In a no_std environment, we can't print to stdout
            // The energy calculations are still performed and validated
            
            // Basic validation - check that calculations produce reasonable values
            assert!(power_data.voltage_rms[0] > 0.0);
            assert!(power_data.voltage_rms[0] < 1000.0); // Reasonable voltage range
            
            for ct in 0..6 {
                if power_data.current_rms[ct] > 0.01 {
                    // Power factor should be between -1 and 1
                    assert!(power_data.power_factor[ct] >= -1.0);
                    assert!(power_data.power_factor[ct] <= 1.0);
                }
            }
        }
    }
    
    // Show accumulated energy (validation only)
    let energy_totals = calc.get_energy_totals();
    for (_ct, &energy) in energy_totals.iter().enumerate() {
        if energy > 0.0 {
            // Energy should be positive and reasonable
            assert!(energy >= 0.0);
            assert!(energy < 1000.0); // Reasonable energy range for test
        }
    }
    
    // Success - enter infinite loop
    loop {
        cortex_m::asm::nop();
    }
}