# Oscilloscope Validation Guide - Siglent SDS1202X-E

> **Prerequisites**: This guide assumes you have already uploaded the debug firmware to your Arduino Zero. See [FIRMWARE_UPLOAD_GUIDE.md](./FIRMWARE_UPLOAD_GUIDE.md) for detailed upload instructions.

## 🔬 **Hardware Setup for Validation**

### **Test Points on emon32 Board**
```
ADC Channels (SAMD21 pins):
- PA02 (AIN0) - Voltage channel
- PA03 (AIN1) - Current channel  
- PA04 (AIN2) - Additional current/voltage
- PA05 (AIN3) - Temperature/auxiliary

GPIO Test Points:
- PA10 - Debug/timing pin (configure as output)
- PA11 - Sample ready indicator
- LED pins - Visual validation
```

### **Oscilloscope Connection**
```
Siglent SDS1202X-E Setup:
- Channel 1: ADC trigger/timing signals
- Channel 2: Sample processing indicators
- External trigger: ADC conversion complete
- Timebase: 10μs/div (for timing analysis)
- Voltage: 1V/div (for 3.3V logic levels)
```

## ⏱️ **Timing Validation Tests**

### **Test 1: ADC Sampling Consistency**
```rust
// Add to main.rs for timing validation
use atsamd_hal::gpio::{Pin, Output, PushPull};

// Configure timing pin
let mut timing_pin = pins.pa10.into_push_pull_output();

// In ADC sampling loop
loop {
    timing_pin.set_high().unwrap();  // Start marker
    let sample = adc.read(&mut adc_pin).unwrap();
    timing_pin.set_low().unwrap();   // End marker
    
    // Process sample...
    calculator.add_sample(sample as f32);
}
```

**Expected Results:**
- **Simple POC**: Regular pulses, ~6μs processing time
- **RTIC Version**: Consistent timing, no jitter from interrupts
- **Frequency**: Should match configured sample rate (e.g., 4800 Hz)

### **Test 2: Interrupt Response Time (RTIC)**
```rust
// In RTIC main_rtic_simple.rs
#[task(binds = ADC, priority = 3, shared = [calculator])]
fn adc_handler(ctx: adc_handler::Context) {
    static mut DEBUG_PIN: Option<Pin<PA11, Output<PushPull>>> = None;
    
    // Set debug pin high on interrupt entry
    if let Some(pin) = DEBUG_PIN {
        pin.set_high().unwrap();
    }
    
    // Process ADC data
    // ... ADC handling code ...
    
    // Set debug pin low on interrupt exit
    if let Some(pin) = DEBUG_PIN {
        pin.set_low().unwrap();
    }
}
```

**Expected Results:**
- **Interrupt latency**: <10μs from trigger to handler entry
- **Handler duration**: <50μs total processing time
- **No missed interrupts**: Consistent pulse train

## 📊 **Measurement Procedures**

### **Procedure 1: Sample Rate Accuracy**
```bash
# Flash firmware with timing pins enabled
cargo objcopy --release --bin emon32-rtic -- -O binary emon32-rtic.bin
# Flash to hardware using your preferred method
```

**Oscilloscope Settings:**
```
Time/Div: 200μs (for 4800 Hz = 208μs period)
Trigger: Rising edge on Channel 1 (timing pin)
Measurement: Frequency, Period, Duty cycle
```

**Expected Measurements:**
- **Period**: 208.33μs (4800 Hz)
- **Jitter**: <1% for RTIC, variable for simple POC
- **Duty cycle**: Depends on processing load

### **Procedure 2: Processing Time Analysis**
**Oscilloscope Settings:**
```
Time/Div: 10μs (for detailed timing analysis)
Trigger: Rising edge, single sweep
Cursors: Manual measurement of pulse width
Math: CH1-CH2 for differential timing
```

**Measurements to Record:**
```
Simple POC:
- Min processing time: _____ μs
- Max processing time: _____ μs  
- Average: _____ μs
- Jitter (max-min): _____ μs

RTIC Version:
- Min processing time: _____ μs
- Max processing time: _____ μs
- Average: _____ μs
- Jitter (max-min): _____ μs
```

### **Procedure 3: Real-Time Guarantee Validation**
```rust
// Add stress test to RTIC version
#[task(priority = 1, shared = [calculator])]
fn background_load(ctx: background_load::Context) {
    // Simulate computational load
    for _ in 0..1000 {
        unsafe { core::ptr::read_volatile(&42u32); }
    }
    // Reschedule immediately
    background_load::spawn().ok();
}
```

**Test Setup:**
- Enable background computational load
- Monitor ADC timing consistency
- Verify high-priority tasks aren't blocked

**Expected Results:**
- ADC timing remains consistent despite background load
- High-priority tasks execute within timing constraints

## 🔍 **Advanced Analysis**

### **FFT Analysis (Using Siglent's Built-in FFT)**
```
Setup:
1. Connect CH1 to ADC timing signal
2. Set timebase for 10-20 cycles capture
3. Enable FFT on CH1
4. Look for:
   - Fundamental frequency (sample rate)
   - Harmonics (should be minimal)
   - Noise floor (indicates timing stability)
```

### **Trigger Delay Measurement**
```
Setup:
1. CH1: External ADC trigger signal
2. CH2: Processing complete signal  
3. Use delayed trigger to measure:
   - Trigger-to-processing delay
   - Processing completion consistency
```

## 📈 **Validation Checklist**

### ✅ **Timing Validation**
- [ ] Sample rate matches configuration (±0.1%)
- [ ] Processing time <50μs consistently
- [ ] RTIC shows lower jitter than simple POC
- [ ] No missed samples under load

### ✅ **Real-Time Validation**  
- [ ] ADC interrupts never delayed >10μs
- [ ] High-priority tasks preempt low-priority
- [ ] Background load doesn't affect critical timing
- [ ] Resource sharing doesn't cause delays

### ✅ **Performance Comparison**
- [ ] RTIC vs Simple timing consistency
- [ ] Power consumption (if measuring current)
- [ ] CPU idle time (observable via GPIO toggles)
- [ ] Interrupt overhead measurement

## 🔧 **Debug Pin Configuration**

Add this to your Rust firmware for comprehensive validation:

```rust
// Debug pin definitions
pub struct DebugPins {
    pub adc_timing: Pin<PA10, Output<PushPull>>,
    pub processing: Pin<PA11, Output<PushPull>>,
    pub interrupt: Pin<PA12, Output<PushPull>>,
}

impl DebugPins {
    pub fn new(pins: &mut Pins) -> Self {
        Self {
            adc_timing: pins.pa10.into_push_pull_output(),
            processing: pins.pa11.into_push_pull_output(), 
            interrupt: pins.pa12.into_push_pull_output(),
        }
    }
    
    pub fn mark_adc_start(&mut self) { self.adc_timing.set_high().unwrap(); }
    pub fn mark_adc_end(&mut self) { self.adc_timing.set_low().unwrap(); }
    pub fn mark_processing_start(&mut self) { self.processing.set_high().unwrap(); }
    pub fn mark_processing_end(&mut self) { self.processing.set_low().unwrap(); }
}
```

## 📋 **Validation Report Template**

```
OSCILLOSCOPE VALIDATION RESULTS
==============================

Hardware: emon32 + Siglent SDS1202X-E
Firmware: [ ] Simple POC  [ ] RTIC Version
Date: ___________

TIMING MEASUREMENTS:
- Sample Rate: _______ Hz (Expected: 4800 Hz)
- Processing Time: Min: _____ μs, Max: _____ μs, Avg: _____ μs
- Jitter: _____ μs (Max - Min)
- Interrupt Latency: _____ μs (RTIC only)

REAL-TIME PERFORMANCE:
- Missed Samples: _____ (should be 0)
- Maximum Delay: _____ μs (should be <50μs)
- Background Load Impact: [ ] None [ ] Minimal [ ] Significant

COMPARISON WITH C FIRMWARE:
- Timing Consistency: [ ] Better [ ] Same [ ] Worse
- Resource Usage: [ ] Lower [ ] Same [ ] Higher
- Real-time Behavior: [ ] Better [ ] Same [ ] Worse

VALIDATION STATUS: [ ] PASS [ ] FAIL
Notes: ________________________________
```

## 🚀 **Next Steps After Validation**

1. **If timing validates**: Proceed with full peripheral integration
2. **If jitter is high**: Optimize critical sections, adjust priorities
3. **If real-time fails**: Review RTIC configuration, check for blocking code
4. **Compare with C baseline**: Ensure Rust version meets/exceeds performance

This comprehensive validation will prove your Rust migration maintains (or improves) the real-time characteristics of the original C firmware! 📊⚡