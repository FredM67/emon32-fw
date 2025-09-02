# Arduino Zero (ABX00003) Oscilloscope Validation Guide

## 🔧 **IMPORTANT: Arduino Zero Pin Limitations**

The **Arduino Zero (ABX00003)** does NOT expose all SAMD21 pins. Many PA/PB pins are internal-only or used for board functions. 

### ✅ **Available GPIO Pins for Oscilloscope Debug**

Based on Arduino Zero pinout and HAL availability:

```
Arduino Zero Exposed Pins:
- Digital Pin 2  → PA14 (available, good for debug)
- Digital Pin 5  → PA15 (available, good for debug)  
- Digital Pin 7  → PA21 (available, good for debug)
- Digital Pin 13 → PA17 (onboard LED, visible indicator)
- Analog Pin A0  → PA02 (ADC channel, could monitor)
- Analog Pin A1  → PA03 (ADC channel, could monitor)
```

### 🚫 **Pins NOT Available on Arduino Zero**
```
PA10, PA11, PA12 - These are INTERNAL ONLY or not exposed
Many other PA/PB pins - Used for USB, crystal, or not broken out
```

## 🔌 **Updated Hardware Connections**

### **Arduino Zero → Siglent SDS1202X-E**
```
Pin 2  (PA14) → Channel 1 (ADC timing)
Pin 5  (PA15) → Channel 2 (Processing duration)
Pin 7  (PA21) → External Trigger (Interrupt response)  
Pin 13 (PA17) → Visual LED indicator (status)
GND           → Ground reference
```

### **Alternative: Minimal 2-Channel Setup**
```
Pin 2  (PA14) → Channel 1 (Complete cycle timing)
Pin 5  (PA15) → Channel 2 (Processing only)
Pin 13 (LED)  → Visual heartbeat indicator
```

## ⚙️ **Updated Scope Settings**
```
Time/Div: 50μs (for 4800 Hz = 208μs period)
Voltage:  1V/div (3.3V logic levels)
Trigger:  Pin 2 (PA14) rising edge
Acquisition: Normal, no averaging
```

## 📊 **What You Can Still Validate**

### ✅ **Full Validation Possible**
- **Sample rate accuracy**: 4800 Hz timing on Pin 2
- **Processing time**: Pulse width measurement on Pin 5  
- **System health**: LED blink pattern on Pin 13
- **Jitter analysis**: Period variation between simple/RTIC
- **Power efficiency**: Current consumption differences

### ✅ **Measurements Available**
- **Timing consistency**: CH1 period stability
- **Processing load**: CH2 duty cycle
- **Real-time behavior**: No dropped pulses under load
- **RTIC benefits**: Lower jitter, more consistent timing

## 🔧 **Arduino Zero Compatible Debug Pins**

The debug firmware needs to use **Arduino Zero compatible pins**:

```rust
// Arduino Zero compatible debug pins
struct DebugPins {
    adc_timing: Pin<PA14, PushPullOutput>,    // Digital Pin 2
    processing: Pin<PA15, PushPullOutput>,    // Digital Pin 5  
    heartbeat: Pin<PA21, PushPullOutput>,     // Digital Pin 7
    status_led: Pin<PA17, PushPullOutput>,    // Digital Pin 13 (onboard LED)
}
```

## 🚀 **Still Comprehensive Validation**

Even with Arduino Zero limitations, you can **fully validate**:

✅ **Algorithm Performance**: Timing measurements prove correctness  
✅ **Real-time Capability**: No missed cycles under load  
✅ **RTIC Benefits**: Demonstrable timing improvements  
✅ **Production Readiness**: Professional oscilloscope validation  

The **core validation objectives remain achievable** with the available pins!

## 📋 **Updated Test Procedures**

### **Test 1: Sample Rate (Pin 2 - PA14)**
```
Expected: 4800 Hz period (208.33μs)
Measure: Frequency/period on CH1
Validation: ±0.1% accuracy
```

### **Test 2: Processing Time (Pin 5 - PA15)**  
```
Expected: <50μs pulse width
Measure: HIGH duration on CH2
Validation: Consistent processing time
```

### **Test 3: System Health (Pin 13 - LED)**
```
Expected: Regular blink pattern
Measure: Visual + scope if needed
Validation: No lock-ups or crashes
```

### **Test 4: Jitter Comparison**
```
Expected: RTIC < Simple POC variation
Measure: Period statistics over time
Validation: Improved timing consistency
```

**The Arduino Zero pin limitations don't prevent comprehensive validation - they just require pin remapping!** 🔧⚡📊