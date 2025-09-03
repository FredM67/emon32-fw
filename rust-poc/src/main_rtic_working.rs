//! Working RTIC implementation for SAMD21 (Cortex-M0+)
//! This version avoids SysTick monotonic and uses simpler task scheduling

#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = atsamd21j, peripherals = true, dispatchers = [EVSYS, RTC])]
mod app {
    use atsamd_hal::{
        clock::GenericClockController,
        gpio::{Pins, Pin, PushPullOutput, PA27},
        prelude::*,
        timer::TimerCounter,
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
        led: LedPin,
    }

    #[local]
    struct Local {
        sample_timer: TimerCounter<atsamd21j::Tc3>,
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

        // Timer for ADC sampling - using proper duration
        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut sample_timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.pm);
        
        // Start timer for periodic ADC sampling (using duration instead of rate)
        use atsamd_hal::prelude::InterruptDrivenTimer;
        sample_timer.start(1000u32.Hz()); // 1kHz
        sample_timer.enable_interrupt();

        // Initialize energy calculator
        let energy_calc = EnergyCalculator::new();

        // Start background tasks
        heartbeat::spawn().ok();

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

    /// HIGH PRIORITY: ADC sampling task (interrupt-driven)
    #[task(binds = TC3, local = [sample_timer, current_samples, channel_index], shared = [sample_count], priority = 3)]
    fn adc_sample(mut ctx: adc_sample::Context) {
        let timer = ctx.local.sample_timer;
        let current_samples = ctx.local.current_samples;
        let channel_index = ctx.local.channel_index;
        
        // Clear timer interrupt
        use atsamd_hal::prelude::InterruptDrivenTimer;
        timer.wait().ok();

        // Simulate ADC reading (replace with real ADC code)
        let sample = 2048u16 + (*channel_index as u16 * 100); // Simulated data
        
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
                
                // Send to processing task (medium priority)
                process_energy::spawn(sample_array).ok();
                
                // Reset for next sample set
                current_samples.clear();
                *channel_index = 0;
                
                // Update statistics
                ctx.shared.sample_count.lock(|count| {
                    *count = count.wrapping_add(1);
                });
            }
        }
    }

    /// MEDIUM PRIORITY: Energy calculation
    #[task(shared = [energy_calc], priority = 2)]
    async fn process_energy(mut ctx: process_energy::Context, samples: [u16; VCT_TOTAL]) {
        ctx.shared.energy_calc.lock(|calc| {
            // Convert array to slice for compatibility with existing code
            let sample_slice: &[u16] = &samples;
            if sample_slice.len() >= VCT_TOTAL {
                // Create a properly sized Vec for the calculator
                let mut sample_vec: heapless::Vec<u16, 128> = heapless::Vec::new();
                for &sample in sample_slice.iter().take(VCT_TOTAL) {
                    sample_vec.push(sample).ok();
                }
                
                let timestamp_ms = 0u32; // Simplified timestamp
                if let Some(power_data) = calc.process_samples(&sample_vec, timestamp_ms) {
                    // Send to output task (low priority)
                    output_data::spawn(power_data).ok();
                }
            }
        });
    }

    /// LOW PRIORITY: Data output and communication
    #[task(priority = 1)]
    async fn output_data(_ctx: output_data::Context, _power_data: PowerData) {
        // Real implementation would:
        // - Send UART data
        // - Update display  
        // - Log to EEPROM
        // - Transmit via RF69
        
        // Simulate processing time
        for _ in 0..1000 {
            asm::nop();
        }
    }

    /// LOW PRIORITY: System heartbeat and monitoring
    #[task(shared = [led, sample_count], priority = 1)]
    async fn heartbeat(mut ctx: heartbeat::Context) {
        // Toggle LED
        ctx.shared.led.lock(|led| {
            led.toggle().ok();
        });

        // Monitor system health
        let _count = ctx.shared.sample_count.lock(|c| *c);
        
        // In production: check for errors, timeouts, etc.
        
        // Reschedule after delay (simplified - would use timer in production)
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