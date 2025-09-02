pub mod calculator;

pub use calculator::*;

use heapless::Vec;
use crate::board::{NUM_V, NUM_CT};

// Power measurement data structure
#[derive(Debug, Clone, Copy)]
pub struct PowerData {
    pub voltage_rms: [f32; NUM_V],
    pub current_rms: [f32; NUM_CT],
    pub real_power: [f32; NUM_CT],
    pub apparent_power: [f32; NUM_CT],
    pub power_factor: [f32; NUM_CT],
    pub frequency: f32,
    pub energy_wh: [f32; NUM_CT],
}

impl Default for PowerData {
    fn default() -> Self {
        Self {
            voltage_rms: [0.0; NUM_V],
            current_rms: [0.0; NUM_CT],
            real_power: [0.0; NUM_CT],
            apparent_power: [0.0; NUM_CT],
            power_factor: [0.0; NUM_CT],
            frequency: 50.0,  // Default to 50Hz
            energy_wh: [0.0; NUM_CT],
        }
    }
}

// Raw ADC sample buffer
pub type SampleBuffer = Vec<u16, 128>;  // Adjust size as needed

// Events for the energy monitoring system
#[derive(Debug, Clone, Copy)]
pub enum EnergyEvent {
    SamplesReady,
    CalculationComplete,
    ReportReady,
}