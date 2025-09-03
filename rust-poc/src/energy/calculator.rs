use super::{PowerData, SampleBuffer};
use crate::board::{ADC_RES_BITS, ADC_VREF, CAL_CT, CAL_V, NUM_CT, NUM_V};
use micromath::F32Ext;

// Conditionally import FastMath trait when qfplib is available
#[cfg(feature = "qfplib")]
use crate::math::FastMath;

/// Energy calculation engine
/// This is a simplified version of the C emon_CM module
pub struct EnergyCalculator {
    // Calibration factors
    voltage_cal: [f32; NUM_V],
    current_cal: [f32; NUM_CT],
    phase_cal: [f32; NUM_CT],

    // Running accumulations
    energy_accumulator: [f32; NUM_CT],
    sample_count: u32,
    last_calculation_time: u32,

    // Configuration
    report_cycles: u32,
    mains_frequency: f32,
}

impl EnergyCalculator {
    pub fn new() -> Self {
        Self {
            voltage_cal: [CAL_V; NUM_V],
            current_cal: [CAL_CT; NUM_CT],
            phase_cal: [0.0; NUM_CT], // No phase correction for POC
            energy_accumulator: [0.0; NUM_CT],
            sample_count: 0,
            last_calculation_time: 0,
            report_cycles: 47, // ~10 second reports at 4.8kHz
            mains_frequency: 50.0,
        }
    }

    /// Process a buffer of ADC samples and calculate power metrics
    pub fn process_samples(
        &mut self,
        samples: &SampleBuffer,
        timestamp_ms: u32,
    ) -> Option<PowerData> {
        if samples.len() < (NUM_V + NUM_CT) {
            return None;
        }

        let mut power_data = PowerData::default();

        // Convert ADC counts to actual voltage/current values
        let adc_resolution = (1 << ADC_RES_BITS) as f32;
        let adc_scale = ADC_VREF / adc_resolution;

        // Calculate RMS values for voltage channels
        for v_ch in 0..NUM_V {
            let mut sum_squares = 0.0f32;
            let mut sample_count = 0;

            // Extract voltage samples (interleaved with CT samples)
            for i in (v_ch..samples.len()).step_by(NUM_V + NUM_CT) {
                #[cfg(feature = "qfplib")]
                let voltage =
                    ((samples[i] as f32).fast_mul(adc_scale)).fast_mul(self.voltage_cal[v_ch]);

                #[cfg(not(feature = "qfplib"))]
                let voltage = (samples[i] as f32) * adc_scale * self.voltage_cal[v_ch];

                #[cfg(feature = "qfplib")]
                {
                    sum_squares = sum_squares.fast_add(voltage.fast_mul(voltage));
                }

                #[cfg(not(feature = "qfplib"))]
                {
                    sum_squares += voltage * voltage;
                }

                sample_count += 1;
            }

            if sample_count > 0 {
                #[cfg(feature = "qfplib")]
                {
                    power_data.voltage_rms[v_ch] =
                        (sum_squares.fast_div(sample_count as f32)).fast_sqrt();
                }

                #[cfg(not(feature = "qfplib"))]
                {
                    power_data.voltage_rms[v_ch] = (sum_squares / sample_count as f32).sqrt();
                }
            }
        }

        // Calculate RMS values and power for CT channels
        for ct_ch in 0..NUM_CT.min(6) {
            // Limit to 6 CTs for POC
            let mut current_sum_squares = 0.0f32;
            let mut power_sum = 0.0f32;
            let mut sample_count = 0;

            // Use first voltage channel as reference (single phase for POC)
            let v_ref_idx = 0;
            let mut v_samples = heapless::Vec::<f32, 64>::new();
            let mut i_samples = heapless::Vec::<f32, 64>::new();

            // Extract samples for this CT channel
            for i in ((NUM_V + ct_ch)..samples.len()).step_by(NUM_V + NUM_CT) {
                if let Some(v_sample_idx) = i.checked_sub(ct_ch + (NUM_V - v_ref_idx)) {
                    if v_sample_idx < samples.len() {
                        #[cfg(feature = "qfplib")]
                        let voltage = ((samples[v_sample_idx] as f32).fast_mul(adc_scale))
                            .fast_mul(self.voltage_cal[v_ref_idx]);

                        #[cfg(not(feature = "qfplib"))]
                        let voltage = (samples[v_sample_idx] as f32)
                            * adc_scale
                            * self.voltage_cal[v_ref_idx];

                        #[cfg(feature = "qfplib")]
                        let current = ((samples[i] as f32).fast_mul(adc_scale))
                            .fast_mul(self.current_cal[ct_ch]);

                        #[cfg(not(feature = "qfplib"))]
                        let current = (samples[i] as f32) * adc_scale * self.current_cal[ct_ch];

                        if v_samples.push(voltage).is_ok() && i_samples.push(current).is_ok() {
                            #[cfg(feature = "qfplib")]
                            {
                                current_sum_squares =
                                    current_sum_squares.fast_add(current.fast_mul(current));
                                power_sum = power_sum.fast_add(voltage.fast_mul(current));
                            }

                            #[cfg(not(feature = "qfplib"))]
                            {
                                current_sum_squares += current * current;
                                power_sum += voltage * current;
                            }

                            sample_count += 1;
                        }
                    }
                }
            }

            if sample_count > 0 {
                // Calculate RMS current
                #[cfg(feature = "qfplib")]
                {
                    power_data.current_rms[ct_ch] =
                        (current_sum_squares.fast_div(sample_count as f32)).fast_sqrt();
                }

                #[cfg(not(feature = "qfplib"))]
                {
                    power_data.current_rms[ct_ch] =
                        (current_sum_squares / sample_count as f32).sqrt();
                }

                // Calculate real power
                #[cfg(feature = "qfplib")]
                {
                    power_data.real_power[ct_ch] = power_sum.fast_div(sample_count as f32);
                }

                #[cfg(not(feature = "qfplib"))]
                {
                    power_data.real_power[ct_ch] = power_sum / sample_count as f32;
                }

                // Calculate apparent power
                #[cfg(feature = "qfplib")]
                {
                    power_data.apparent_power[ct_ch] =
                        power_data.voltage_rms[v_ref_idx].fast_mul(power_data.current_rms[ct_ch]);
                }

                #[cfg(not(feature = "qfplib"))]
                {
                    power_data.apparent_power[ct_ch] =
                        power_data.voltage_rms[v_ref_idx] * power_data.current_rms[ct_ch];
                }

                // Calculate power factor
                if power_data.apparent_power[ct_ch] > 0.01 {
                    // Avoid division by zero
                    #[cfg(feature = "qfplib")]
                    {
                        power_data.power_factor[ct_ch] =
                            power_data.real_power[ct_ch].fast_div(power_data.apparent_power[ct_ch]);
                    }

                    #[cfg(not(feature = "qfplib"))]
                    {
                        power_data.power_factor[ct_ch] =
                            power_data.real_power[ct_ch] / power_data.apparent_power[ct_ch];
                    }

                    // Clamp power factor to [-1, 1]
                    power_data.power_factor[ct_ch] =
                        power_data.power_factor[ct_ch].max(-1.0).min(1.0);
                }

                // Accumulate energy (Wh)
                let time_delta_hours =
                    (timestamp_ms - self.last_calculation_time) as f32 / (1000.0 * 3600.0);
                if time_delta_hours > 0.0 {
                    #[cfg(feature = "qfplib")]
                    let energy_delta = power_data.real_power[ct_ch].fast_mul(time_delta_hours);

                    #[cfg(not(feature = "qfplib"))]
                    let energy_delta = power_data.real_power[ct_ch] * time_delta_hours;

                    #[cfg(feature = "qfplib")]
                    {
                        self.energy_accumulator[ct_ch] =
                            self.energy_accumulator[ct_ch].fast_add(energy_delta);
                    }

                    #[cfg(not(feature = "qfplib"))]
                    {
                        self.energy_accumulator[ct_ch] += energy_delta;
                    }

                    power_data.energy_wh[ct_ch] = self.energy_accumulator[ct_ch];
                }
            }
        }

        // Simple frequency estimation (placeholder)
        power_data.frequency = self.mains_frequency;

        self.sample_count += 1;
        self.last_calculation_time = timestamp_ms;

        // Return data every report_cycles
        if self.sample_count >= self.report_cycles {
            self.sample_count = 0;
            Some(power_data)
        } else {
            None
        }
    }

    /// Set calibration values
    pub fn set_voltage_calibration(&mut self, channel: usize, cal: f32) {
        if channel < NUM_V {
            self.voltage_cal[channel] = cal;
        }
    }

    pub fn set_current_calibration(&mut self, channel: usize, cal: f32) {
        if channel < NUM_CT {
            self.current_cal[channel] = cal;
        }
    }

    /// Reset energy accumulators
    pub fn reset_energy(&mut self) {
        self.energy_accumulator = [0.0; NUM_CT];
    }

    /// Get current energy totals
    pub fn get_energy_totals(&self) -> [f32; NUM_CT] {
        self.energy_accumulator
    }
}
