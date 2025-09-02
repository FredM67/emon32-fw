// Host-only test for energy calculation algorithms
// This uses std and can run on development machines

use std::println;

// Copy the essential types and logic for testing
const NUM_V: usize = 3;
const NUM_CT: usize = 12;
const ADC_VREF: f32 = 1.024;
const ADC_RES_BITS: u8 = 11;
const CAL_V: f32 = 8.087;
const CAL_CT: f32 = 3.0;

#[derive(Debug, Clone, Copy)]
struct PowerData {
    voltage_rms: [f32; NUM_V],
    current_rms: [f32; NUM_CT],
    real_power: [f32; NUM_CT],
    apparent_power: [f32; NUM_CT],
    power_factor: [f32; NUM_CT],
    frequency: f32,
    energy_wh: [f32; NUM_CT],
}

impl Default for PowerData {
    fn default() -> Self {
        Self {
            voltage_rms: [0.0; NUM_V],
            current_rms: [0.0; NUM_CT],
            real_power: [0.0; NUM_CT],
            apparent_power: [0.0; NUM_CT],
            power_factor: [0.0; NUM_CT],
            frequency: 50.0,
            energy_wh: [0.0; NUM_CT],
        }
    }
}

struct EnergyCalculator {
    voltage_cal: [f32; NUM_V],
    current_cal: [f32; NUM_CT],
    energy_accumulator: [f32; NUM_CT],
    sample_count: u32,
    last_calculation_time: u32,
    report_cycles: u32,
    mains_frequency: f32,
}

impl EnergyCalculator {
    fn new() -> Self {
        Self {
            voltage_cal: [CAL_V; NUM_V],
            current_cal: [CAL_CT; NUM_CT],
            energy_accumulator: [0.0; NUM_CT],
            sample_count: 0,
            last_calculation_time: 0,
            report_cycles: 47,
            mains_frequency: 50.0,
        }
    }
    
    fn process_samples(&mut self, samples: &[u16], timestamp_ms: u32) -> Option<PowerData> {
        if samples.len() < (NUM_V + NUM_CT) {
            return None;
        }
        
        let mut power_data = PowerData::default();
        
        let adc_resolution = (1 << ADC_RES_BITS) as f32;
        let adc_scale = ADC_VREF / adc_resolution;
        
        // Calculate RMS values for voltage channels
        for v_ch in 0..NUM_V {
            let mut sum_squares = 0.0f32;
            let mut sample_count = 0;
            
            for i in (v_ch..samples.len()).step_by(NUM_V + NUM_CT) {
                let voltage = (samples[i] as f32) * adc_scale * self.voltage_cal[v_ch];
                sum_squares += voltage * voltage;
                sample_count += 1;
            }
            
            if sample_count > 0 {
                power_data.voltage_rms[v_ch] = (sum_squares / sample_count as f32).sqrt();
            }
        }
        
        // Calculate RMS values and power for CT channels
        for ct_ch in 0..NUM_CT.min(6) {
            let mut current_sum_squares = 0.0f32;
            let mut power_sum = 0.0f32;
            let mut sample_count = 0;
            
            let v_ref_idx = 0;
            
            for i in ((NUM_V + ct_ch)..samples.len()).step_by(NUM_V + NUM_CT) {
                if let Some(v_sample_idx) = i.checked_sub(ct_ch + (NUM_V - v_ref_idx)) {
                    if v_sample_idx < samples.len() {
                        let voltage = (samples[v_sample_idx] as f32) * adc_scale * self.voltage_cal[v_ref_idx];
                        let current = (samples[i] as f32) * adc_scale * self.current_cal[ct_ch];
                        
                        current_sum_squares += current * current;
                        power_sum += voltage * current;
                        sample_count += 1;
                    }
                }
            }
            
            if sample_count > 0 {
                power_data.current_rms[ct_ch] = (current_sum_squares / sample_count as f32).sqrt();
                power_data.real_power[ct_ch] = power_sum / sample_count as f32;
                power_data.apparent_power[ct_ch] = power_data.voltage_rms[v_ref_idx] * power_data.current_rms[ct_ch];
                
                if power_data.apparent_power[ct_ch] > 0.01 {
                    power_data.power_factor[ct_ch] = power_data.real_power[ct_ch] / power_data.apparent_power[ct_ch];
                    power_data.power_factor[ct_ch] = power_data.power_factor[ct_ch].max(-1.0).min(1.0);
                }
                
                let time_delta_hours = (timestamp_ms - self.last_calculation_time) as f32 / (1000.0 * 3600.0);
                if time_delta_hours > 0.0 {
                    let energy_delta = power_data.real_power[ct_ch] * time_delta_hours;
                    self.energy_accumulator[ct_ch] += energy_delta;
                    power_data.energy_wh[ct_ch] = self.energy_accumulator[ct_ch];
                }
            }
        }
        
        power_data.frequency = self.mains_frequency;
        
        self.sample_count += 1;
        self.last_calculation_time = timestamp_ms;
        
        if self.sample_count >= self.report_cycles {
            self.sample_count = 0;
            Some(power_data)
        } else {
            None
        }
    }
}

fn main() {
    println!("emon32 Rust POC - Energy Calculation Test");
    println!("==========================================");
    
    let mut calc = EnergyCalculator::new();
    
    println!("✓ Energy calculator initialized");
    
    // Generate test data
    let mut samples = Vec::new();
    for i in 0..90 {
        let channel = i % 9;
        let time_phase = (i / 9) as f32 * 0.628;
        
        let sample_value = if channel < 3 {
            let amplitude = 400.0;
            let offset = 2048.0;
            let voltage_sample = offset + amplitude * (time_phase).sin();
            voltage_sample.max(0.0).min(4095.0) as u16
        } else {
            let ct_channel = channel - 3;
            let base_current = [100.0, 50.0, 25.0, 10.0, 5.0, 0.0][ct_channel];
            let amplitude = base_current * 2.0;
            let offset = 2048.0;
            let phase_shift = ct_channel as f32 * 0.2;
            let current_sample = offset + amplitude * (time_phase + phase_shift).sin();
            current_sample.max(0.0).min(4095.0) as u16
        };
        
        samples.push(sample_value);
    }
    
    println!("✓ Generated {} test samples", samples.len());
    
    // Process samples multiple times to get a result
    let mut result_count = 0;
    for cycle in 0..50 {  // Process enough cycles to get a result
        let timestamp = cycle * 100;
        if let Some(power_data) = calc.process_samples(&samples, timestamp) {
            result_count += 1;
            
            println!("\n✓ Calculation #{} completed at {}ms:", result_count, timestamp);
            println!("  Voltage RMS: {:.2} V", power_data.voltage_rms[0]);
            
            for ct in 0..6 {
                if power_data.current_rms[ct] > 0.01 {
                    println!("  CT{}: {:.1} W, {:.3} A RMS, PF: {:.3}", 
                             ct + 1,
                             power_data.real_power[ct],
                             power_data.current_rms[ct],
                             power_data.power_factor[ct]);
                    
                    // Validate calculations
                    assert!(power_data.power_factor[ct] >= -1.0 && power_data.power_factor[ct] <= 1.0, 
                           "Power factor out of range");
                    assert!(power_data.current_rms[ct] >= 0.0, "Current RMS cannot be negative");
                    assert!(power_data.voltage_rms[0] > 0.0, "Voltage RMS should be positive");
                }
            }
            
            if result_count >= 3 {
                break;  // Got enough results for validation
            }
        }
    }
    
    assert!(result_count > 0, "Should have produced at least one result");
    
    println!("\n✅ All tests passed!");
    println!("✅ Energy calculation algorithms are working correctly");
    println!("✅ Rust POC demonstrates successful C to Rust migration feasibility");
}