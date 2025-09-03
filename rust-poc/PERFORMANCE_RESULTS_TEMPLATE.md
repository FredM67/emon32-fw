# qfplib Performance Test Results

## Test Environment

- **Hardware:** Arduino Zero (ATSAMD21J17A)
- **Clock Speed:** 48 MHz
- **Compiler:** rustc with ARM target `thumbv6m-none-eabi`
- **Test Date:** [DATE]
- **Firmware Version:** [VERSION]
- **Test Iterations:** 1,000 operations per test

## Test Methodology

### Timing Measurement
- **Timer:** ARM SysTick (24-bit counter, 48 MHz)
- **Precision:** CPU cycle accuracy
- **Output:** RTT (Real-Time Transfer) for minimal overhead

### Test Operations
Realistic energy monitoring workloads:
1. **Square Root Operations:** RMS voltage/current calculations
2. **Division Operations:** Power efficiency calculations  
3. **Multiplication Operations:** Power calculations
4. **Combined Operations:** Integrated energy monitoring simulation

### Test Data
```rust
// Representative energy monitoring values
let voltage_samples = [220.5f32, 221.2, 219.8, 220.1, 220.9];
let current_samples = [2.1f32, 2.3, 1.9, 2.2, 2.0];
```

## Performance Results

### Standard Math Baseline (micromath)

```
=== Standard Math (micromath) Performance Test ===
Results for 5000 operations:
  Square root: [CYCLES] cycles ([MICROSECONDS] μs)
  Division:    [CYCLES] cycles ([MICROSECONDS] μs) 
  Multiply:    [CYCLES] cycles ([MICROSECONDS] μs)
  Combined:    [CYCLES] cycles ([MICROSECONDS] μs)
  Total:       [CYCLES] cycles ([MICROSECONDS] μs)
```

### qfplib Optimized Results

```
=== qfplib Fast Math Performance Test ===
Results for 5000 operations:
  Square root: [CYCLES] cycles ([MICROSECONDS] μs)
  Division:    [CYCLES] cycles ([MICROSECONDS] μs)
  Multiply:    [CYCLES] cycles ([MICROSECONDS] μs) 
  Combined:    [CYCLES] cycles ([MICROSECONDS] μs)
  Total:       [CYCLES] cycles ([MICROSECONDS] μs)
```

## Performance Comparison

| Operation Type | Standard (cycles) | qfplib (cycles) | Improvement | Time Saved (μs) |
|---------------|-------------------|-----------------|-------------|------------------|
| Square Root   | [BASELINE]        | [OPTIMIZED]     | [X.Xx]      | [TIME_DIFF]      |
| Division      | [BASELINE]        | [OPTIMIZED]     | [X.Xx]      | [TIME_DIFF]      |
| Multiplication| [BASELINE]        | [OPTIMIZED]     | [X.Xx]      | [TIME_DIFF]      |
| Combined      | [BASELINE]        | [OPTIMIZED]     | [X.Xx]      | [TIME_DIFF]      |
| **Total**     | [BASELINE]        | [OPTIMIZED]     | **[X.Xx]**  | **[TIME_DIFF]**  |

## Analysis

### Performance Improvements

- **Square Root Operations:** [ANALYSIS]
- **Division Operations:** [ANALYSIS]  
- **Multiplication Operations:** [ANALYSIS]
- **Overall Energy Calculations:** [ANALYSIS]

### Real-World Impact

**For Energy Monitoring Applications:**
- **Sampling Rate Impact:** [ANALYSIS]
- **Power Consumption:** [ANALYSIS]
- **Response Time:** [ANALYSIS]
- **Calculation Throughput:** [ANALYSIS]

### Memory Usage

- **Code Size Impact:** [ANALYZE .bin file sizes]
- **RAM Usage:** [ANALYSIS]
- **Flash Usage:** [ANALYSIS]

## Conclusions

### Recommendations

1. **Use qfplib for production:** [RATIONALE]
2. **Optimization priorities:** [LIST]
3. **Trade-offs:** [ANALYSIS]

### Next Steps

1. **Integration into main energy calculator**
2. **Real-time performance validation**
3. **Power consumption measurement**
4. **Long-term stability testing**

## Raw Test Data

### Standard Math RTT Output
```
[PASTE COMPLETE RTT OUTPUT HERE]
```

### qfplib RTT Output  
```
[PASTE COMPLETE RTT OUTPUT HERE]
```

## Test Verification

### Build Commands Used
```bash
cd /home/fredm67/git/emon32-fw/rust-poc
./build_qfplib_performance.sh
```

### Firmware Files
- `bin/emon32-performance-standard.uf2` - [FILE SIZE] bytes
- `bin/emon32-qfplib-performance.uf2` - [FILE SIZE] bytes

### RTT Connection
```bash
probe-rs rtt attach --chip ATSAMD21J17A
```

---

**Test conducted by:** [TESTER NAME]  
**Review date:** [DATE]  
**Approved by:** [REVIEWER]
