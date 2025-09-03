//! RTIC version with oscilloscope debug pins
#![no_std]
#![no_main]

#[cfg(target_arch = "arm")]
use panic_halt as _;

#[rtic::app(device = atsamd21j, dispatchers = [EVSYS, RTC, WDT])]
mod app {
    use atsamd_hal::{
        clock::GenericClockController,
        gpio::{Pin, Pins, PushPullOutput, PA14, PA15, PA17, PA21},
        prelude::*,
    };
    use cortex_m::asm;
    use heapless::Vec;

    use emon32_rust_poc::board::VCT_TOTAL;
    use emon32_rust_poc::energy::{EnergyCalculator, PowerData};

    type LedPin = Pin<PA17, PushPullOutput>; // Pin 13 - Onboard LED
    type DebugPin1 = Pin<PA14, PushPullOutput>; // Pin 2 - ADC timing
    type DebugPin2 = Pin<PA15, PushPullOutput>; // Pin 5 - Processing
    type DebugPin3 = Pin<PA21, PushPullOutput>; // Pin 7 - Interrupt response

    #[shared]
    struct Shared {
        energy_calc: EnergyCalculator,
        sample_count: u32,
        debug_pin1: DebugPin1, // Shared for ADC timing
        debug_pin2: DebugPin2, // Shared for processing
    }

    #[local]
    struct Local {
        led: LedPin,
        debug_pin3: DebugPin3, // Local for interrupt response
        current_samples: Vec<u16, VCT_TOTAL>,
        channel_index: usize,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut peripherals = ctx.device;
        let pins = Pins::new(peripherals.port);

        // Basic clock configuration
        let _clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );

        // LED for status indication (Arduino Zero onboard LED)
        let led: LedPin = pins.pa17.into_push_pull_output();

        // Debug pins for oscilloscope validation (Arduino Zero compatible)
        let debug_pin1: DebugPin1 = pins.pa14.into_push_pull_output(); // Pin 2
        let debug_pin2: DebugPin2 = pins.pa15.into_push_pull_output(); // Pin 5
        let debug_pin3: DebugPin3 = pins.pa21.into_push_pull_output(); // Pin 7

        // Initialize energy calculator
        let energy_calc = EnergyCalculator::new();

        // Start the main tasks
        sample_adc::spawn().ok();
        heartbeat::spawn().ok();

        (
            Shared {
                energy_calc,
                sample_count: 0,
                debug_pin1,
                debug_pin2,
            },
            Local {
                led,
                debug_pin3,
                current_samples: Vec::new(),
                channel_index: 0,
            },
        )
    }

    /// HIGH PRIORITY: Simulated ADC sampling task
    #[task(local = [current_samples, channel_index, debug_pin3], shared = [sample_count, debug_pin1], priority = 3)]
    async fn sample_adc(mut ctx: sample_adc::Context) {
        // === OSCILLOSCOPE MARKER: Interrupt Response Time ===
        ctx.local.debug_pin3.set_high().unwrap();

        // === OSCILLOSCOPE MARKER: ADC Start ===
        ctx.shared.debug_pin1.lock(|pin| pin.set_high().unwrap());

        let current_samples = ctx.local.current_samples;
        let channel_index = ctx.local.channel_index;

        // Simulate ADC reading with realistic timing
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

        // === OSCILLOSCOPE MARKER: ADC End ===
        ctx.shared.debug_pin1.lock(|pin| pin.set_low().unwrap());

        // === OSCILLOSCOPE MARKER: Interrupt Complete ===
        ctx.local.debug_pin3.set_low().unwrap();

        // Realistic sample timing - 4800 Hz = 208μs period
        delay_cycles(9600); // ~200μs at 48MHz

        // Reschedule ourselves for next sample
        sample_adc::spawn().ok();
    }

    /// MEDIUM PRIORITY: Energy calculation
    #[task(shared = [energy_calc, debug_pin2], priority = 2)]
    async fn process_energy(mut ctx: process_energy::Context, samples: [u16; VCT_TOTAL]) {
        // === OSCILLOSCOPE MARKER: Processing Start ===
        ctx.shared.debug_pin2.lock(|pin| pin.set_high().unwrap());

        let result = ctx.shared.energy_calc.lock(|calc| {
            // Create a properly sized Vec for the calculator
            let mut sample_vec: heapless::Vec<u16, 128> = heapless::Vec::new();
            for &sample in samples.iter().take(VCT_TOTAL) {
                sample_vec.push(sample).ok();
            }

            let timestamp_ms = 0u32; // Simplified timestamp
            calc.process_samples(&sample_vec, timestamp_ms)
        });

        // === OSCILLOSCOPE MARKER: Processing End ===
        ctx.shared.debug_pin2.lock(|pin| pin.set_low().unwrap());

        if let Some(power_data) = result {
            // Send to output task
            output_data::spawn(power_data).ok();
        }
    }

    /// LOW PRIORITY: Data output (doesn't affect real-time behavior)
    #[task(priority = 1)]
    async fn output_data(_ctx: output_data::Context, _power_data: PowerData) {
        // Simulate UART/USB output processing time
        delay_cycles(2400); // ~50μs processing time

        // In real implementation:
        // - Send UART data
        // - Update display
        // - Log to EEPROM
        // - Transmit via RF69
    }

    /// LOW PRIORITY: System heartbeat
    #[task(local = [led], shared = [sample_count], priority = 1)]
    async fn heartbeat(mut ctx: heartbeat::Context) {
        // Toggle LED for visual indication
        ctx.local.led.toggle().ok();

        // Check system health
        let _count = ctx.shared.sample_count.lock(|c| *c);

        // Heartbeat every ~1 second
        delay_cycles(48_000_000); // ~1 second at 48MHz

        // Reschedule
        heartbeat::spawn().ok();
    }

    /// Background computational load for stress testing
    #[task(priority = 1)]
    async fn background_load(_ctx: background_load::Context) {
        // Simulate background computational work
        for _ in 0..1000 {
            unsafe {
                core::ptr::read_volatile(&42u32);
            }
        }

        // Small delay then reschedule
        delay_cycles(1000);
        background_load::spawn().ok();
    }

    /// IDLE: CPU sleep when no tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Put CPU to sleep - saves power!
            // This is observable on oscilloscope as reduced current consumption
            asm::wfi();
        }
    }

    // Helper function for precise timing
    fn delay_cycles(cycles: u32) {
        for _ in 0..cycles {
            asm::nop();
        }
    }
}
