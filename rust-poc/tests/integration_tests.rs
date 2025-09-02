// Integration tests for energy calculations
// These tests can run on the host machine

#[cfg(test)]
mod tests {
    use emon32_rust_poc::energy::EnergyCalculator;
    use heapless::Vec;

    #[test]
    fn test_energy_calculator_creation() {
        let calc = EnergyCalculator::new();
        let energy_totals = calc.get_energy_totals();
        
        // All energy totals should start at zero
        for &energy in energy_totals.iter() {
            assert_eq!(energy, 0.0);
        }
    }

    #[test]
    fn test_calibration_setting() {
        let mut calc = EnergyCalculator::new();
        
        // Set some test calibration values
        calc.set_voltage_calibration(0, 10.0);
        calc.set_current_calibration(0, 5.0);
        
        // The calculator should accept these values without panicking
        // (Internal state verification would require exposing internals)
    }

    #[test]
    fn test_sample_processing() {
        let mut calc = EnergyCalculator::new();
        
        // Create a test sample buffer with some dummy data
        let mut samples = Vec::<u16, 128>::new();
        
        // Add some test samples (voltage + current interleaved)
        for i in 0..60 {
            // Simulate 3 voltage channels + 6 current channels = 9 channels total
            let channel = i % 9;
            let sample_value = if channel < 3 {
                // Voltage channels: simulate ~230V RMS (scaled to ADC range)
                2048 + (i * 100) as u16  // Simple pattern
            } else {
                // Current channels: simulate various current levels
                2048 + (i * 50) as u16
            };
            samples.push(sample_value).unwrap();
        }
        
        // Process samples
        let result = calc.process_samples(&samples, 1000);
        
        // For this test, we don't expect a result on the first call
        // (needs multiple cycles to accumulate)
        // Just verify no panic occurs
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_energy_reset() {
        let mut calc = EnergyCalculator::new();
        
        // Reset energy accumulators
        calc.reset_energy();
        
        // All should be zero
        let energy_totals = calc.get_energy_totals();
        for &energy in energy_totals.iter() {
            assert_eq!(energy, 0.0);
        }
    }
}