//! RTIC Hardware UART Demo for SAMD21
//! 
//! Real-time energy monitoring with UART output on Arduino Zero
//! Using PA14 (TX) and PA15 (RX) - Arduino Zero pins 2 and 5

#![no_std]
#![no_main]

use panic_halt as _;
use micromath::F32Ext;

/// Generate test samples for demonstration
fn generate_test_sample(sample_index: u32, channel: usize) -> u16 {
    // Simulate 230V RMS voltage and varying current
    let time = sample_index as f32 * 0.001; // Simulate 1kHz sampling
    let frequency = 50.0; // 50Hz mains frequency
    let phase = 2.0 * 3.14159 * frequency * time;
    
    match channel {
        0 => {
            // Voltage channel: 230V RMS sine wave
            let voltage = 230.0 * 1.414 * phase.sin(); // Peak voltage
            ((voltage + 400.0) * 16.0) as u16 // Scale to ADC range
        },
        1 => {
            // Current channel 1: 0.65A RMS (150W @ 230V)
            let current = 0.65 * 1.414 * phase.sin();
            ((current + 2.5) * 819.2) as u16
        },
        2 => {
            // Current channel 2: 0.33A RMS (75W @ 230V)  
            let current = 0.33 * 1.414 * (phase + 0.2).sin(); // Slight phase shift
            ((current + 2.5) * 819.2) as u16
        },
        3 => {
            // Current channel 3: 0A (no load)
            (2.5 * 819.2) as u16 // Zero current = mid-scale ADC
        },
        _ => 0,
    }
}

#[rtic::app(device = atsamd21j, peripherals = true, dispatchers = [EVSYS, RTC])]
mod app {
    use atsamd_hal::{
        clock::GenericClockController,
        gpio::{Pins, Pin, PushPullOutput, PA27},
        prelude::*,
        timer::TimerCounter,
    };
    use cortex_m::asm;
    use micromath::F32Ext;
    
    use emon32_rust_poc::energy::{EnergyCalculator, SampleBuffer};
    use emon32_rust_poc::uart::UartOutput;

    type LedPin = Pin<PA27, PushPullOutput>;

    #[shared]
    struct Shared {
        energy_calc: EnergyCalculator,
        uart_output: UartOutput,
        sample_count: u32,
        led: LedPin,
    }

    #[local]
    struct Local {
        sample_timer: TimerCounter<atsamd21j::Tc3>,
        current_samples: SampleBuffer,
        channel_index: usize,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut peripherals = ctx.device;
        let pins = Pins::new(peripherals.port);

        // Clock configuration
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );

        // LED for status indication
        let led: LedPin = pins.pa27.into_push_pull_output();

        // Configure UART pins
        let pa14 = pins.pa14; // TX - Arduino Zero pin 2
        let pa15 = pins.pa15; // RX - Arduino Zero pin 5
        
        // Initialize UART output with hardware implementation
        let mut uart_output = UartOutput::new_hardware(
            peripherals.sercom2,
            pa14,
            pa15,
            &mut clocks,
            &mut peripherals.pm,
        );

        // Send startup banner
        uart_output.send_banner();

        // Timer for ADC sampling
        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut sample_timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.pm);
        
        // Start timer for periodic ADC sampling
        use atsamd_hal::prelude::InterruptDrivenTimer;
        InterruptDrivenTimer::start(&mut sample_timer, 1_000_000.micros()); // 1ms intervals
        sample_timer.enable_interrupt();

        // Initialize empty sample buffer
        let current_samples = SampleBuffer::new();

        // Schedule heartbeat task
        heartbeat::spawn().ok();

        (
            Shared {
                energy_calc: EnergyCalculator::new(),
                uart_output,
                sample_count: 0,
                led,
            },
            Local {
                sample_timer,
                current_samples,
                channel_index: 0,
            },
        )
    }

    /// Heartbeat task - blinks LED and sends periodic status
    #[task(shared = [led, uart_output], priority = 1)]
    async fn heartbeat(mut ctx: heartbeat::Context) {
        let mut counter = 0u32;
        
        loop {
            // Toggle LED
            ctx.shared.led.lock(|led| {
                led.toggle().ok();
            });

            // Send periodic status via UART
            if counter % 10 == 0 {
                ctx.shared.uart_output.lock(|uart| {
                    uart.send_status("Heartbeat - System running");
                });
            }

            counter = counter.wrapping_add(1);
            
            // Simple delay using busy loop
            for _ in 0..10_000_000 {
                cortex_m::asm::nop();
            }
        }
    }

    /// ADC sampling task triggered by timer interrupt
    #[task(binds = TC3, local = [sample_timer, current_samples, channel_index], shared = [energy_calc, uart_output, sample_count], priority = 2)]
    fn sample_adc(mut ctx: sample_adc::Context) {
        let sample_timer = ctx.local.sample_timer;
        let current_samples = ctx.local.current_samples;
        let channel_index = ctx.local.channel_index;

        // Clear interrupt flag 
        use atsamd_hal::prelude::InterruptDrivenTimer;
        InterruptDrivenTimer::wait(sample_timer).ok();

        // Get current sample count
        let sample_count = ctx.shared.sample_count.lock(|c| *c);

        // Generate test sample for current channel
        let sample_value = crate::generate_test_sample(sample_count, *channel_index);
        
        // Add sample to buffer
        if current_samples.push(sample_value).is_err() {
            // Buffer full - process samples
            ctx.shared.energy_calc.lock(|calc| {
                ctx.shared.uart_output.lock(|uart| {
                    ctx.shared.sample_count.lock(|count| {
                        let timestamp_ms = *count * 100;
                        if let Some(power_data) = calc.process_samples(current_samples, timestamp_ms) {
                            // Send energy data via UART
                            uart.maybe_output(&power_data, timestamp_ms);
                        }
                        *count = count.wrapping_add(1);
                    });
                });
            });
            
            // Clear samples for next batch
            current_samples.clear();
        }

        // Move to next channel (round-robin)
        *channel_index = (*channel_index + 1) % 4;
    }

    /// Idle task
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi(); // Wait for interrupt
        }
    }
}