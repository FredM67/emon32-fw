# Quick Performance Testing Documentation Guide# Quick Performance Testing Documentation Guide



## When You're Ready to Document Your Results## When You're Ready to Document Your Results



After you upload the firmware to your Arduino Zero and collect the performance data, here's how to document it:After you upload the firmware to your Arduino Zero and collect the performance data, here's how to document it:



### Step 0: Build Performance Binaries### Step 0: Build Performance Binaries



First, ensure you have the performance test binaries built:First, ensure you have the performance test binaries built:

```bash```bash

./build_unified.sh performance  # Builds micromath and qfplib performance tests./build_unified.sh performance  # Builds micromath and qfplib performance tests

# This creates:# This creates:

# - bin/emon32-performance-micromath.uf2# - bin/emon32-performance-micromath.uf2

# - bin/emon32-performance-qfplib.uf2# - bin/emon32-performance-qfplib.uf2

``````



### Step 1: Collect Raw Data### Step 1: Collect Raw Data



1. **Deploy Standard Firmware:**1. **Deploy Standard Firmware:**

   ```bash   ```bash

   # Option A: UF2 Bootloader Upload (Easier)   # Option A: UF2 Bootloader Upload (Easier)

   # 1. Double-press Arduino Zero reset button   # 1. Double-press Arduino Zero reset button

   # 2. Copy bin/emon32-performance-micromath.uf2 to EMONBOOT drive   # 2. Copy bin/emon32-performance-micromath.uf2 to EMONBOOT drive

   # 3. Use serial monitor (115200 baud) to capture output   # 3. Use serial monitor (115200 baud) to capture output

      

   # Option B: RTT with probe-rs (Advanced)   # Option B: RTT with probe-rs (Advanced)

   probe-rs run --chip ATSAMD21J17A bin/emon32-performance-micromath.elf   probe-rs run --chip ATSAMD21J17A bin/emon32-performance-micromath.elf

   # Copy all RTT output to a text file   # Copy all RTT output to a text file

   ```   ```



2. **Deploy qfplib Firmware:**2. **Deploy qfplib Firmware:**

   ```bash   ```bash

   # Option A: UF2 Bootloader Upload (Easier)   # Option A: UF2 Bootloader Upload (Easier)

   # 1. Double-press Arduino Zero reset button     # 1. Double-press Arduino Zero reset button  

   # 2. Copy bin/emon32-performance-qfplib.uf2 to EMONBOOT drive   # 2. Copy bin/emon32-performance-qfplib.uf2 to EMONBOOT drive

   # 3. Use serial monitor (115200 baud) to capture output   # 3. Use serial monitor (115200 baud) to capture output

      

   # Option B: RTT with probe-rs (Advanced)   # Option B: RTT with probe-rs (Advanced)

   probe-rs run --chip ATSAMD21J17A bin/emon32-performance-qfplib.elf   probe-rs run --chip ATSAMD21J17A bin/emon32-performance-qfplib.elf

   # Copy all RTT output to a text file   # Copy all RTT output to a text file

   ```   ```



### Step 2: Fill Out the Results Template### Step 2: Fill Out the Results Template



1. **Copy the template:**1. **Copy the template:**

   ```bash   ```bash

   cp PERFORMANCE_RESULTS_TEMPLATE.md PERFORMANCE_RESULTS_[DATE].md   cp PERFORMANCE_RESULTS_TEMPLATE.md PERFORMANCE_RESULTS_[DATE].md

   ```   ```



2. **Replace placeholders** with your actual measurements:2. **Replace placeholders** with your actual measurements:

   - `[CYCLES]` → actual cycle counts from RTT output   - `[CYCLES]` → actual cycle counts from RTT output

   - `[MICROSECONDS]` → actual timing measurements     - `[MICROSECONDS]` → actual timing measurements  

   - `[BASELINE]` → standard math results   - `[BASELINE]` → standard math results

   - `[OPTIMIZED]` → qfplib results   - `[OPTIMIZED]` → qfplib results

   - `[X.Xx]` → improvement ratios (optimized/baseline)   - `[X.Xx]` → improvement ratios (optimized/baseline)

   - `[TIME_DIFF]` → time savings in microseconds   - `[TIME_DIFF]` → time savings in microseconds



### Step 3: Calculate Performance Improvements### Step 3: Calculate Performance Improvements



**Example calculation:****Example calculation:**

``````

Standard Math Square Root: 12000 cyclesStandard Math Square Root: 12000 cycles

qfplib Square Root: 4000 cyclesqfplib Square Root: 4000 cycles

Improvement = 12000 / 4000 = 3.0x fasterImprovement = 12000 / 4000 = 3.0x faster

Time Saved = (12000 - 4000) / 48MHz = 167 μs per 1000 operationsTime Saved = (12000 - 4000) / 48MHz = 167 μs per 1000 operations

``````\n\n### Step 4: Add Analysis\n\nFill in the analysis sections with:\n- **Performance Impact:** What the improvements mean for energy monitoring\n- **Real-world Benefits:** How this affects sampling rates, power consumption\n- **Recommendations:** Whether to use qfplib in production\n\n### Step 5: Update Project Documentation\n\nOnce you have results:\n\n1. **Update QFPLIB_INTEGRATION_COMPLETE.md:**\n   - Change status from \"Awaiting Hardware Validation\" to \"Complete\"\n   - Add actual performance numbers\n\n2. **Update README.md:**\n   - Add link to your results file\n   - Include performance summary\n\n3. **Update PROJECT_STATUS.md:**\n   - Mark qfplib performance testing as complete\n\n### Example RTT Output Format\n\nYour RTT output will look like this:\n```\nqfplib Performance Test Starting...\nARM Cortex-M0+ SysTick-based timing measurement\nSysTick frequency: 48000000 Hz\nRunning performance tests with 1000 operations per test\n\nTesting standard floating-point (micromath fallback):\n\n=== Standard Math (micromath) Performance Test ===\nResults for 5000 operations:\n  Square root: 12000 cycles (250 μs)\n  Division:    8000 cycles (167 μs)\n  Multiply:    4000 cycles (83 μs)\n  Combined:    16000 cycles (333 μs)\n  Total:       40000 cycles (833 μs)\n\nTesting qfplib optimized floating-point:\n\n=== qfplib Fast Math Performance Test ===\nResults for 5000 operations:\n  Square root: 4000 cycles (83 μs)\n  Division:    2000 cycles (42 μs)  \n  Multiply:    2000 cycles (42 μs)\n  Combined:    6000 cycles (125 μs)\n  Total:       14000 cycles (292 μs)\n\nPerformance testing complete!\nqfplib has been successfully integrated and tested!\n```\n\n### File Organization\n\nKeep your performance documentation organized:\n```\nrust-poc/\n├── PERFORMANCE_RESULTS_TEMPLATE.md     # Template file\n├── PERFORMANCE_RESULTS_2025-09-03.md   # Your actual results\n├── PERFORMANCE_TESTING_GUIDE.md        # Hardware testing guide\n├── QFPLIB_INTEGRATION_COMPLETE.md      # Technical integration details\n└── performance_data/                    # Raw RTT output files\n    ├── standard_math_output.txt\n    └── qfplib_output.txt\n```\n\n## Quick Checklist\n\n- [ ] Arduino Zero connected and in bootloader mode\n- [ ] Standard firmware deployed and tested\n- [ ] qfplib firmware deployed and tested  \n- [ ] RTT output captured for both versions\n- [ ] Performance results template filled out\n- [ ] Improvement ratios calculated\n- [ ] Analysis sections completed\n- [ ] Project documentation updated\n- [ ] Results committed to git\n\nThis documentation approach will give you a complete record of the qfplib performance benefits and help justify its use in the production firmware.

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