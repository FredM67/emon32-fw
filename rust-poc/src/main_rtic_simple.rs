//! Simple working RTIC implementation for SAMD21
//! Focuses on basic task structure without complex timer configuration

#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
use panic_halt as _;

// Use the PAC directly instead of HAL for RTIC
#[rtic::app(device = atsamd21j, dispatchers = [EVSYS, RTC, WDT])]
mod app {
    use atsamd_hal::{
        // clock::GenericClockController, // Not used in simple example
        gpio::{Pins, Pin, PushPullOutput, PA27},
        prelude::*,
    };
    use cortex_m::asm;
    use heapless::Vec;
    
    use emon32_rust_poc::energy::{EnergyCalculator, PowerData};
    use emon32_rust_poc::board::VCT_TOTAL;

    type LedPin = Pin<PA27, PushPullOutput>;

    #[shared]
    struct Shared {
        energy_calc: EnergyCalculator,
        sample_count: u32,
    }

    #[local]
    struct Local {
        led: LedPin,
        current_samples: Vec<u16, VCT_TOTAL>,
        channel_index: usize,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let peripherals = ctx.device;
        let pins = Pins::new(peripherals.port);

                // Basic clock configuration\n        let _clocks = GenericClockController::with_external_32kosc(\n            peripherals.gclk,\n            &mut peripherals.pm,\n            &mut peripherals.sysctrl,\n            &mut peripherals.nvmctrl,\n        );

        // LED for status indication
        let led: LedPin = pins.pa27.into_push_pull_output();

        // Initialize energy calculator
        let energy_calc = EnergyCalculator::new();

        // Start the main tasks
        sample_adc::spawn().ok();
        heartbeat::spawn().ok();

        (
            Shared {
                energy_calc,
                sample_count: 0,
            },
            Local {
                led,
                current_samples: Vec::new(),
                channel_index: 0,
            },
        )
    }

    /// HIGH PRIORITY: Simulated ADC sampling task
    #[task(local = [current_samples, channel_index], shared = [sample_count], priority = 3)]
    async fn sample_adc(mut ctx: sample_adc::Context) {
        let current_samples = ctx.local.current_samples;
        let channel_index = ctx.local.channel_index;

        // Simulate ADC reading
        let sample = 2048u16 + (*channel_index as u16 * 100);
        
        if current_samples.push(sample).is_ok() {
            *channel_index += 1;
            
            // When we have samples for all channels
            if *channel_index >= VCT_TOTAL {
                // Convert to array for processing
                let mut sample_array = [0u16; VCT_TOTAL];
                for (i, &sample) in current_samples.iter().enumerate() {
                    if i < VCT_TOTAL {
                        sample_array[i] = sample;
                    }
                }
                
                // Send to processing task
                process_energy::spawn(sample_array).ok();
                
                // Reset for next sample set
                current_samples.clear();
                *channel_index = 0;
                
                // Update count
                ctx.shared.sample_count.lock(|count| {
                    *count = count.wrapping_add(1);
                });
            }
        }

        // Simple delay simulation - in real version would be timer interrupt
        for _ in 0..10000 {
            asm::nop();
        }
        
        // Reschedule ourselves
        sample_adc::spawn().ok();
    }

    /// MEDIUM PRIORITY: Energy calculation
    #[task(shared = [energy_calc], priority = 2)]
    async fn process_energy(mut ctx: process_energy::Context, samples: [u16; VCT_TOTAL]) {
        ctx.shared.energy_calc.lock(|calc| {
            // Create a properly sized Vec for the calculator
            let mut sample_vec: heapless::Vec<u16, 128> = heapless::Vec::new();
            for &sample in samples.iter().take(VCT_TOTAL) {
                sample_vec.push(sample).ok();
            }
            
            let timestamp_ms = 0u32; // Simplified timestamp
            if let Some(power_data) = calc.process_samples(&sample_vec, timestamp_ms) {
                // Send to output task
                output_data::spawn(power_data).ok();
            }
        });
    }

    /// LOW PRIORITY: Data output
    #[task(priority = 1)]
    async fn output_data(_ctx: output_data::Context, _power_data: PowerData) {
        // Simulate UART/USB output processing time
        for _ in 0..50000 {
            asm::nop();
        }
        
        // In real implementation:
        // - Send UART data
        // - Update display  
        // - Log to EEPROM
        // - Transmit via RF69
    }

    /// LOW PRIORITY: System heartbeat
    #[task(local = [led], shared = [sample_count], priority = 1)]
    async fn heartbeat(mut ctx: heartbeat::Context) {
        // Toggle LED
        ctx.local.led.toggle().ok();

        // Check system health
        let _count = ctx.shared.sample_count.lock(|c| *c);
        
        // Simple delay for heartbeat
        for _ in 0..1000000 {
            asm::nop();
        }
        
        // Reschedule
        heartbeat::spawn().ok();
    }

    /// IDLE: CPU sleep when no tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Put CPU to sleep - saves power!
            asm::wfi();
        }
    }
}