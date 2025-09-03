//! RTIC-based emon32 energy monitoring firmware
//!
//! This demonstrates proper real-time task structure for energy monitoring:
//! - High-priority ADC sampling task (triggered by timer)
//! - Medium-priority data processing task  
//! - Low-priority communication and display tasks

#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = atsamd_hal::pac, peripherals = true, dispatchers = [EVSYS, RTC])]
mod app {
    use atsamd_hal::{
        clock::GenericClockController,
        gpio::{Pin, Pins, PushPullOutput, PA27},
        prelude::*,
        timer::TimerCounter,
    };
    use cortex_m::asm;
    use heapless::Vec;

    use emon32_rust_poc::board::VCT_TOTAL;
    use emon32_rust_poc::energy::{EnergyCalculator, PowerData};

    type LedPin = Pin<PA27, PushPullOutput>;

    #[shared]
    struct Shared {
        energy_calc: EnergyCalculator,
        sample_count: u32,
        led: LedPin,
    }

    #[local]
    struct Local {
        sample_timer: TimerCounter<atsamd_hal::pac::Tc3>,
        current_samples: Vec<u16, VCT_TOTAL>,
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

        // Timer for ADC sampling (simplified for compilation)
        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let sample_timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.pm);
        // Note: Timer configuration will be added once HAL compatibility is resolved

        // Initialize energy calculator
        let energy_calc = EnergyCalculator::new();

        // Start periodic tasks
        heartbeat_task::spawn().ok();
        data_processing_task::spawn().ok();

        (
            Shared {
                energy_calc,
                sample_count: 0,
                led,
            },
            Local {
                sample_timer,
                current_samples: Vec::new(),
                channel_index: 0,
            },
        )
    }

    /// High-priority ADC sampling task
    /// Triggered by timer interrupt at SAMPLE_RATE * VCT_TOTAL Hz
    #[task(binds = TC3, shared = [energy_calc, sample_count], local = [sample_timer, current_samples, channel_index], priority = 3)]
    fn sample_adc(mut ctx: sample_adc::Context) {
        let _timer = ctx.local.sample_timer;
        let current_samples = ctx.local.current_samples;
        let channel_index = ctx.local.channel_index;

        // Note: Timer interrupt clearing will be added with proper HAL setup
        // For now, just simulate the ADC sampling        // Simulate ADC sample (in real implementation would read from ADC)
        let sample = 2048u16; // Simulated ADC reading

        if current_samples.push(sample).is_ok() {
            *channel_index += 1;

            // When we have a complete set of samples for all channels
            if *channel_index >= VCT_TOTAL {
                // Convert to array and send to processing task
                let mut sample_array = [0u16; VCT_TOTAL];
                for (i, &sample) in current_samples.iter().enumerate() {
                    if i < VCT_TOTAL {
                        sample_array[i] = sample;
                    }
                }

                // Send samples to processing task
                process_samples::spawn(sample_array).ok();

                // Reset for next sample set
                current_samples.clear();
                *channel_index = 0;

                // Update sample count
                ctx.shared.sample_count.lock(|count| {
                    *count = count.wrapping_add(1);
                });
            }
        }
    }

    /// Medium-priority data processing task
    /// Processes ADC samples and calculates energy
    #[task(shared = [energy_calc], priority = 2)]
    async fn process_samples(mut ctx: process_samples::Context, samples: [u16; VCT_TOTAL]) {
        // Simple timestamp (in real implementation would use proper timer)
        let timestamp_ms = 0u32;

        ctx.shared.energy_calc.lock(|calc| {
            // Convert local Vec to SampleBuffer for compatibility with process_samples
            let mut sample_buffer = emon32_rust_poc::energy::SampleBuffer::new();
            for &sample in &samples[..] {
                sample_buffer.push(sample).ok();
            }
            if let Some(power_data) = calc.process_samples(&sample_buffer, timestamp_ms) {
                // Send results to output task
                output_results::spawn(power_data).ok();
            }
        });
    }

    /// Low-priority output task
    /// Handles UART/USB communication and display updates
    #[task(priority = 1)]
    async fn output_results(_ctx: output_results::Context, _power_data: PowerData) {
        // In real implementation, this would:
        // - Send data over UART/USB
        // - Update display
        // - Log to EEPROM
        // - Send over RF69 radio

        // For now, just simulate processing time
        for _ in 0..1000 {
            asm::nop();
        }
    }

    /// Heartbeat task - blinks LED and provides system status
    #[task(shared = [led, sample_count], priority = 1)]
    async fn heartbeat_task(mut ctx: heartbeat_task::Context) {
        // Toggle LED
        ctx.shared.led.lock(|led| {
            led.toggle().ok();
        });

        // Check system health
        let _count = ctx.shared.sample_count.lock(|c| *c);

        // In real implementation, could check for:
        // - ADC sampling rate consistency
        // - Memory usage
        // - Communication timeouts
        // - Sensor failures

        // Schedule next heartbeat (simplified - in real implementation would use timer)
        heartbeat_task::spawn().ok();
    }

    /// Background data processing task
    /// Handles non-critical periodic operations
    #[task(shared = [energy_calc], priority = 1)]
    async fn data_processing_task(mut ctx: data_processing_task::Context) {
        // Periodic energy calculations, calibration updates, etc.
        ctx.shared.energy_calc.lock(|_calc| {
            // Could implement:
            // - Energy total updates
            // - Calibration drift compensation
            // - Statistical analysis
            // - Data logging preparation
        });

        // Schedule next run (simplified)
        data_processing_task::spawn().ok();
    }

    /// Idle task - runs when no other tasks are active
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Put CPU to sleep until next interrupt
            asm::wfi();
        }
    }
}
