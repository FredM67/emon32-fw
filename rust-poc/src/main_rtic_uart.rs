//! RTIC-based emon32 with UART Serial Output
//! 
//! Real-time energy monitoring with:
//! - High-priority ADC sampling simulation
//! - Medium-priority energy calculation  
//! - Low-priority UART output
//! - Format: "1000 ms: V1=230.5V P1=150.2W P2=75.1W P3=0.0W"

#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
use panic_halt as _;

#[rtic::app(device = atsamd21j, dispatchers = [EVSYS, RTC, WDT])]
mod app {
    use atsamd_hal::{
        gpio::{Pin, Pins, PushPullOutput, PA17},
        prelude::*,
    };
    use cortex_m::asm;
    use heapless::Vec;

    use emon32_rust_poc::{
        board::VCT_TOTAL,
        energy::{EnergyCalculator, PowerData},
        uart::UartOutput,
    };

    type LedPin = Pin<PA17, PushPullOutput>; // Onboard LED

    #[shared]
    struct Shared {
        energy_calc: EnergyCalculator,
        uart_output: UartOutput,
        sample_count: u32,
        timestamp_ms: u32,
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

        // Initialize RTT for output if available
        #[cfg(feature = "rtt")]
        {
            use rtt_target::rtt_init_print;
            rtt_init_print!();
        }

        // LED for status indication
        let led: LedPin = pins.pa17.into_push_pull_output();

        // Setup UART for serial output (simplified)
        let mut uart_output = UartOutput::new();

        // Send startup messages
        uart_output.send_banner();
        uart_output.send_status("RTIC Real-Time Energy Monitor");
        uart_output.send_status("High-priority ADC sampling active");

        // Initialize energy calculator
        let energy_calc = EnergyCalculator::new();

        // Start tasks
        sample_adc::spawn().ok();
        heartbeat::spawn().ok();

        (
            Shared {
                energy_calc,
                uart_output,
                sample_count: 0,
                timestamp_ms: 0,
            },
            Local {
                led,
                current_samples: Vec::new(),
                channel_index: 0,
            },
        )
    }

    /// HIGH PRIORITY: ADC sampling simulation
    #[task(local = [current_samples, channel_index], shared = [sample_count, timestamp_ms], priority = 3)]
    async fn sample_adc(mut ctx: sample_adc::Context) {
        let current_samples = ctx.local.current_samples;
        let channel_index = ctx.local.channel_index;

        // Generate realistic test sample
        let sample_value = ctx.shared.sample_count.lock(|count| {
            generate_test_sample(*count + *channel_index as u32)
        });

        if current_samples.push(sample_value).is_ok() {
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

                // Get current timestamp
                let timestamp = ctx.shared.timestamp_ms.lock(|ts| {
                    *ts = ts.wrapping_add(1000); // 1 second intervals
                    *ts
                });

                // Send to processing task
                process_energy::spawn(sample_array, timestamp).ok();

                // Reset for next sample set
                current_samples.clear();
                *channel_index = 0;

                // Update sample count
                ctx.shared.sample_count.lock(|count| {
                    *count = count.wrapping_add(1);
                });
            }
        }

        // Realistic sample timing - simulate 4800 Hz ADC
        delay_cycles(10_000); // ~200μs at 48MHz

        // Reschedule for next sample
        sample_adc::spawn().ok();
    }

    /// MEDIUM PRIORITY: Energy calculation
    #[task(shared = [energy_calc], priority = 2)]
    async fn process_energy(mut ctx: process_energy::Context, samples: [u16; VCT_TOTAL], timestamp_ms: u32) {
        let result = ctx.shared.energy_calc.lock(|calc| {
            // Create sample buffer for the calculator
            let mut sample_vec: heapless::Vec<u16, 128> = heapless::Vec::new();
            for &sample in samples.iter().take(VCT_TOTAL) {
                sample_vec.push(sample).ok();
            }

            calc.process_samples(&sample_vec, timestamp_ms)
        });

        if let Some(power_data) = result {
            // Send to UART output task
            uart_output::spawn(power_data, timestamp_ms).ok();
        }
    }

    /// LOW PRIORITY: UART output (doesn't affect real-time sampling)
    #[task(shared = [uart_output], priority = 1)]
    async fn uart_output(mut ctx: uart_output::Context, power_data: PowerData, timestamp_ms: u32) {
        ctx.shared.uart_output.lock(|uart| {
            uart.maybe_output(&power_data, timestamp_ms);
        });
    }

    /// LOW PRIORITY: System heartbeat and status
    #[task(local = [led], shared = [sample_count, uart_output], priority = 1)]
    async fn heartbeat(mut ctx: heartbeat::Context) {
        // Toggle LED
        ctx.local.led.toggle().ok();

        // Send periodic status
        let count = ctx.shared.sample_count.lock(|c| *c);
        if count % 10 == 0 && count > 0 {
            ctx.shared.uart_output.lock(|uart| {
                uart.send_status("System heartbeat - sampling active");
            });
        }

        // Heartbeat every ~5 seconds
        delay_cycles(240_000_000); // ~5 seconds at 48MHz

        // Reschedule
        heartbeat::spawn().ok();
    }

    /// IDLE: CPU sleep when no tasks running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi(); // Sleep to save power
        }
    }

    // Helper functions
    fn delay_cycles(cycles: u32) {
        for _ in 0..cycles {
            asm::nop();
        }
    }

    fn generate_test_sample(counter: u32) -> u16 {
        use micromath::F32Ext;
        
        let time = (counter as f32) * 0.02;
        let channel = (counter as usize) % 15;
        
        if channel < 3 {
            // Voltage channels
            let amplitude = 600.0;
            let offset = 2048.0;
            let sample = offset + amplitude * (time * 2.0 * core::f32::consts::PI).sin();
            sample.max(0.0).min(4095.0) as u16
        } else {
            // Current channels with different loads
            let ct_channel = channel - 3;
            let base_currents = [10.0, 5.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
            let current_amplitude = base_currents[ct_channel] * 20.0;
            let offset = 2048.0;
            let phase_shift = ct_channel as f32 * 0.1;
            let sample = offset + current_amplitude * (time * 2.0 * core::f32::consts::PI + phase_shift).sin();
            sample.max(0.0).min(4095.0) as u16
        }
    }
}