pub mod pins;

pub use pins::*;

// Board configuration constants matching the C version
pub const F_CORE: u32 = 48_000_000;
pub const F_PERIPH: u32 = 8_000_000;
pub const F_TIMER_ADC: u32 = F_PERIPH / 8;

pub const NUM_V: usize = 3;        // Voltage channels
pub const NUM_CT: usize = 12;      // Current transformer channels
pub const VCT_TOTAL: usize = NUM_V + NUM_CT;
pub const SAMPLE_RATE: u32 = 4800; // Hz per channel
pub const SAMPLES_IN_SET: usize = 2;
pub const SAMPLE_BUF_DEPTH: usize = 2;
pub const OVERSAMPLING_RATIO: usize = 2;

pub const ADC_VREF: f32 = 1.024;   // ADC reference voltage
pub const ADC_RES_BITS: u8 = 11;   // ADC resolution
pub const CAL_V: f32 = 8.087;      // Voltage calibration
pub const CAL_CT: f32 = 3.0;       // Current calibration

// UART configuration
pub const UART_BAUD: u32 = 115200;