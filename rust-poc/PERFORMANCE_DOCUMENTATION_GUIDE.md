# Quick Performance Testing Documentation Guide

## When You're Ready to Document Your Results

After you upload the firmware to your Arduino Zero and collect the performance data, here's how to document it:

### Step 0: Build Performance Binaries

First, ensure you have the performance test binaries built:
```bash
./build_unified.sh performance  # Builds micromath and qfplib performance tests
# This creates:
# - bin/emon32-performance-micromath.uf2
# - bin/emon32-performance-qfplib.uf2
```

### Step 1: Collect Raw Data

1. **Deploy Standard Firmware:**
   ```bash
   # Option A: UF2 Bootloader Upload (Easier)
   # 1. Double-press Arduino Zero reset button
   # 2. Copy bin/emon32-performance-micromath.uf2 to EMONBOOT drive
   # 3. Use serial monitor (115200 baud) to capture output
   
   # Option B: RTT with probe-rs (Advanced)
   probe-rs run --chip ATSAMD21J17A bin/emon32-performance-micromath.elf
   # Copy all RTT output to a text file
   ```

2. **Deploy qfplib Firmware:**
   ```bash
   # Option A: UF2 Bootloader Upload (Easier)
   # 1. Double-press Arduino Zero reset button  
   # 2. Copy bin/emon32-performance-qfplib.uf2 to EMONBOOT drive
   # 3. Use serial monitor (115200 baud) to capture output
   
   # Option B: RTT with probe-rs (Advanced)
   probe-rs run --chip ATSAMD21J17A bin/emon32-performance-qfplib.elf
   # Copy all RTT output to a text file
   ```

### Step 2: Fill Out the Results Template

1. **Copy the template:**
   ```bash
   cp PERFORMANCE_RESULTS_TEMPLATE.md PERFORMANCE_RESULTS_[DATE].md
   ```

2. **Replace placeholders** with your actual measurements:
   - `[CYCLES]` → actual cycle counts from RTT output
   - `[MICROSECONDS]` → actual timing measurements  
   - `[BASELINE]` → standard math results
   - `[OPTIMIZED]` → qfplib results
   - `[X.Xx]` → improvement ratios (optimized/baseline)
   - `[TIME_DIFF]` → time savings in microseconds

### Step 3: Calculate Performance Improvements

**Example calculation:**
```
Standard Math Square Root: 12000 cycles
qfplib Square Root: 4000 cycles
Improvement = 12000 / 4000 = 3.0x faster
Time Saved = (12000 - 4000) / 48MHz = 167 μs per 1000 operations
```

### Step 4: Add Analysis

Fill in the analysis sections with:
- **Performance Impact:** What the improvements mean for energy monitoring
- **Real-world Benefits:** How this affects sampling rates, power consumption
- **Recommendations:** Whether to use qfplib in production

### Step 5: Update Project Documentation

Once you have results:

1. **Update QFPLIB_INTEGRATION_COMPLETE.md:**
   - Change status from "Awaiting Hardware Validation" to "Complete"
   - Add actual performance numbers

2. **Update README.md:**
   - Add link to your results file
   - Include performance summary

3. **Update PROJECT_STATUS.md:**
   - Mark qfplib performance testing as complete

### Example RTT Output Format

Your RTT output will look like this:
```
=== ARM Cortex-M0+ Math Performance Test ===
Target: SAMD21J17 @ 48MHz
Testing: qfplib vs micromath performance

=== Math Library Configuration ===
✓ qfplib enabled - ARM-optimized assembly

1. SINGLE OPERATION TIMING
==========================
qfplib Operations:
  sqrt(2.0): 84 cycles (result: 1.414214)
  2.5 / 1.5: 156 cycles (result: 1.666667)
  exp(1.0): 312 cycles (result: 2.718282)
  sin(1.57): 245 cycles (result: 0.999999)

2. BATCH OPERATION TIMING (1000 ops)
===================================
qfplib Batch Results:
  sqrt batch: 78422 cycles, 78.4 cycles/op
  div batch: 145890 cycles, 145.9 cycles/op
  exp batch: 289445 cycles, 289.4 cycles/op
  sin batch: 234567 cycles, 234.6 cycles/op

3. ENERGY CALCULATION SIMULATION
================================
Processing 1000 samples...
Total processing: 892456 cycles (18.6ms @ 48MHz)
Average per sample: 892 cycles
Memory usage: 4.2KB

Performance testing complete!
qfplib shows significant improvements for transcendental functions!
```

Compare this with micromath output by uploading `emon32-performance-micromath.uf2`.

### File Organization

Keep your performance documentation organized:
```
rust-poc/
├── PERFORMANCE_RESULTS_TEMPLATE.md     # Template file
├── PERFORMANCE_RESULTS_2025-09-03.md   # Your actual results
├── PERFORMANCE_TESTING_GUIDE.md        # Hardware testing guide
├── QFPLIB_INTEGRATION_COMPLETE.md      # Technical integration details
└── performance_data/                    # Raw RTT output files
    ├── standard_math_output.txt
    └── qfplib_output.txt
```

## Quick Checklist

- [ ] Arduino Zero connected and in bootloader mode
- [ ] Standard firmware deployed and tested
- [ ] qfplib firmware deployed and tested  
- [ ] RTT output captured for both versions
- [ ] Performance results template filled out
- [ ] Improvement ratios calculated
- [ ] Analysis sections completed
- [ ] Project documentation updated
- [ ] Results committed to git

This documentation approach will give you a complete record of the qfplib performance benefits and help justify its use in the production firmware.
