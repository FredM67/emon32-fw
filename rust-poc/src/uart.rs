//! UART module for SAMD21 serial output
//! 
//! Provides formatted energy monitoring output via UART at 115200 baud
//! Compatible with emonPi3 hardware and Arduino Zero

use atsamd_hal::{
    clock::GenericClockController,
    gpio::{Disabled, Floating, Pin, PA14, PA15, AlternateC},
    prelude::*,
    sercom::{
        uart::{self, Pads, Uart},
        Sercom2,
    },
};
use heapless::String;
use nb::block;

use crate::energy::PowerData;

// UART pads configuration for SAMD21
// Using SERCOM2 with PA14 (TX) and PA15 (RX) - Arduino Zero compatible
type UartPads = Pads<Sercom2, Pin<PA15, AlternateC>, Pin<PA14, AlternateC>>;
type UartDriver = Uart<uart::Config<UartPads>, uart::Duplex>;

pub struct UartOutput {
    uart: Option<UartDriver>,
    last_output_time: u32,
    output_interval_ms: u32,
}

impl UartOutput {
    /// Initialize UART for energy monitoring output (hardware implementation)
    /// 
    /// # Arguments
    /// * `sercom2` - SERCOM2 peripheral
    /// * `pa14` - PA14 pin for TX (Arduino Zero pin 2)
    /// * `pa15` - PA15 pin for RX (Arduino Zero pin 5)  
    /// * `clocks` - Clock controller
    /// * `pm` - Power manager for peripheral enabling
    pub fn new_hardware(
        sercom2: Sercom2,
        pa14: Pin<PA14, Disabled<Floating>>,
        pa15: Pin<PA15, Disabled<Floating>>,
        clocks: &mut GenericClockController,
        pm: &mut atsamd21j::Pm,
    ) -> Self {
        // Configure pins for SERCOM2 UART
        let rx = pa15.into_mode::<AlternateC>();
        let tx = pa14.into_mode::<AlternateC>();
        let pads = uart::Pads::default().rx(rx).tx(tx);

        // Configure UART at 115200 baud
        let gclk0 = clocks.gclk0();
        let uart = uart::Config::new(
            pm,
            sercom2,
            pads,
            clocks.sercom2_core(&gclk0).unwrap(),
        )
        .baud(115200.Hz(), uart::BaudMode::Fractional(uart::Oversampling::Bits16))
        .enable();

        Self {
            uart: Some(uart),
            last_output_time: 0,
            output_interval_ms: 1000, // Output every 1 second
        }
    }

    /// Initialize UART for energy monitoring output (demo/RTT fallback)
    pub fn new_demo() -> Self {
        Self {
            uart: None,
            last_output_time: 0,
            output_interval_ms: 1000,
        }
    }

    /// Simplified constructor for backward compatibility
    pub fn new() -> Self {
        Self::new_demo()
    }

    /// Send energy monitoring data if interval has elapsed
    pub fn maybe_output(&mut self, power_data: &PowerData, timestamp_ms: u32) {
        if timestamp_ms.wrapping_sub(self.last_output_time) >= self.output_interval_ms {
            self.output_energy_data(power_data, timestamp_ms);
            self.last_output_time = timestamp_ms;
        }
    }

    /// Format and send energy monitoring data via UART
    /// 
    /// Output format: "1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W"
    fn output_energy_data(&mut self, power_data: &PowerData, timestamp_ms: u32) {
        // Create formatted string using heapless for no_std compatibility
        let mut output: String<256> = String::new();
        
        // Format timestamp and voltage - using simple concatenation
        self.append_number(&mut output, timestamp_ms);
        let _ = output.push_str(" ms: V1=");
        self.append_float(&mut output, power_data.voltage_rms[0], 1);
        let _ = output.push('V');

        // Format power values for first 3 CT channels
        for i in 0..3 {
            let power = if i < power_data.real_power.len() {
                power_data.real_power[i]
            } else {
                0.0
            };
            let _ = output.push_str(" P");
            self.append_number(&mut output, (i + 1) as u32);
            let _ = output.push('=');
            self.append_float(&mut output, power, 1);
            let _ = output.push('W');
        }

        // Add newline
        let _ = output.push_str("\r\n");

        // Send via UART
        self.send_string(&output);
    }

    /// Append a u32 number to string
    fn append_number(&self, s: &mut String<256>, mut num: u32) {
        if num == 0 {
            let _ = s.push('0');
            return;
        }

        let mut digits = [0u8; 10];
        let mut count = 0;
        while num > 0 {
            digits[count] = (num % 10) as u8 + b'0';
            num /= 10;
            count += 1;
        }

        // Reverse the digits
        for i in (0..count).rev() {
            let _ = s.push(digits[i] as char);
        }
    }

    /// Append a f32 number with decimal places to string
    fn append_float(&self, s: &mut String<256>, mut num: f32, decimal_places: u8) {
        // Handle negative numbers
        if num < 0.0 {
            let _ = s.push('-');
            num = -num;
        }

        // Integer part
        let integer_part = num as u32;
        self.append_number(s, integer_part);

        if decimal_places > 0 {
            let _ = s.push('.');
            
            // Decimal part
            let mut fractional = num - integer_part as f32;
            for _ in 0..decimal_places {
                fractional *= 10.0;
                let digit = (fractional as u32) % 10;
                let _ = s.push((digit as u8 + b'0') as char);
            }
        }
    }

    /// Send a string via UART (hardware or RTT fallback)
    fn send_string(&mut self, s: &str) {
        if let Some(ref mut uart) = self.uart {
            // Hardware UART implementation
            for byte in s.bytes() {
                // Block until we can send the byte
                let _ = block!(uart.write(byte));
            }
        } else {
            // RTT fallback for demonstration
            #[cfg(feature = "rtt")]
            {
                use rtt_target::rprintln;
                rprintln!("{}", s.trim_end());
            }
            
            #[cfg(not(feature = "rtt"))]
            {
                // Prevent unused variable warning
                let _ = s;
            }
        }
    }

    /// Send a simple status message
    pub fn send_status(&mut self, message: &str) {
        let mut output: String<128> = String::new();
        let _ = output.push_str("Status: ");
        let _ = output.push_str(message);
        let _ = output.push_str("\r\n");
        self.send_string(&output);
    }

    /// Send startup banner
    pub fn send_banner(&mut self) {
        self.send_string("emon32 Rust Energy Monitor v0.1.0\r\n");
        if self.uart.is_some() {
            self.send_string("Hardware UART Output at 115200 baud\r\n");
            self.send_string("Connected on PA14(TX)/PA15(RX) - Arduino Zero pins 2/5\r\n");
        } else {
            self.send_string("RTT Demo Output (115200 baud format)\r\n");
        }
        self.send_string("Format: timestamp ms: V1=voltage P1=power P2=power P3=power\r\n");
        self.send_string("Ready...\r\n\r\n");
    }
}