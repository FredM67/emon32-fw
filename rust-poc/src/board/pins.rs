// Simplified pin definitions for POC
// In a full implementation, these would match the actual hardware pins

use atsamd_hal::gpio::*;

pub struct Pins {
    // Status LED (using available pins for POC)
    pub led_status: Pin<PA27, Output<PushPull>>,
}

impl Pins {
    pub fn new(pins: atsamd_hal::gpio::Pins) -> Self {
        Self {
            // Use PA27 which should be available on SAMD21
            led_status: pins.pa27.into_push_pull_output(),
        }
    }
}