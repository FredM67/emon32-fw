# Quick Performance Testing Documentation Guide

## When You're Ready to Document Your Results

After you upload the firmware to your Arduino Zero and collect the performance data, here's how to document it:

### Step 1: Collect Raw Data

1. **Deploy Standard Firmware:**
   ```bash
   # Option A: UF2 Bootloader Upload (Easier)
   # 1. Double-press Arduino Zero reset button
   # 2. Copy bin/emon32-performance-standard.uf2 to EMONBOOT drive
   # 3. Use serial monitor (115200 baud) to capture output
   
   # Option B: RTT with probe-rs (Advanced)
   probe-rs run --chip ATSAMD21J17A target/thumbv6m-none-eabi/release/emon32-performance
   # Copy all RTT output to a text file
   ```

2. **Deploy qfplib Firmware:**
   ```bash
   # Option A: UF2 Bootloader Upload (Easier)
   # 1. Double-press Arduino Zero reset button  
   # 2. Copy bin/emon32-qfplib-performance.uf2 to EMONBOOT drive
   # 3. Use serial monitor (115200 baud) to capture output
   
   # Option B: RTT with probe-rs (Advanced)
   probe-rs run --chip ATSAMD21J17A target/thumbv6m-none-eabi/release/emon32-qfplib-performance
   # Copy all RTT output to a text file
   ```\n\n### Step 2: Fill Out the Results Template\n\n1. **Copy the template:**\n   ```bash\n   cp PERFORMANCE_RESULTS_TEMPLATE.md PERFORMANCE_RESULTS_[DATE].md\n   ```\n\n2. **Replace placeholders** with your actual measurements:\n   - `[CYCLES]` → actual cycle counts from RTT output\n   - `[MICROSECONDS]` → actual timing measurements  \n   - `[BASELINE]` → standard math results\n   - `[OPTIMIZED]` → qfplib results\n   - `[X.Xx]` → improvement ratios (optimized/baseline)\n   - `[TIME_DIFF]` → time savings in microseconds\n\n### Step 3: Calculate Performance Improvements\n\n**Example calculation:**\n```\nStandard Math Square Root: 12000 cycles\nqfplib Square Root: 4000 cycles\nImprovement = 12000 / 4000 = 3.0x faster\nTime Saved = (12000 - 4000) / 48MHz = 167 μs per 1000 operations\n```\n\n### Step 4: Add Analysis\n\nFill in the analysis sections with:\n- **Performance Impact:** What the improvements mean for energy monitoring\n- **Real-world Benefits:** How this affects sampling rates, power consumption\n- **Recommendations:** Whether to use qfplib in production\n\n### Step 5: Update Project Documentation\n\nOnce you have results:\n\n1. **Update QFPLIB_INTEGRATION_COMPLETE.md:**\n   - Change status from \"Awaiting Hardware Validation\" to \"Complete\"\n   - Add actual performance numbers\n\n2. **Update README.md:**\n   - Add link to your results file\n   - Include performance summary\n\n3. **Update PROJECT_STATUS.md:**\n   - Mark qfplib performance testing as complete\n\n### Example RTT Output Format\n\nYour RTT output will look like this:\n```\nqfplib Performance Test Starting...\nARM Cortex-M0+ SysTick-based timing measurement\nSysTick frequency: 48000000 Hz\nRunning performance tests with 1000 operations per test\n\nTesting standard floating-point (micromath fallback):\n\n=== Standard Math (micromath) Performance Test ===\nResults for 5000 operations:\n  Square root: 12000 cycles (250 μs)\n  Division:    8000 cycles (167 μs)\n  Multiply:    4000 cycles (83 μs)\n  Combined:    16000 cycles (333 μs)\n  Total:       40000 cycles (833 μs)\n\nTesting qfplib optimized floating-point:\n\n=== qfplib Fast Math Performance Test ===\nResults for 5000 operations:\n  Square root: 4000 cycles (83 μs)\n  Division:    2000 cycles (42 μs)  \n  Multiply:    2000 cycles (42 μs)\n  Combined:    6000 cycles (125 μs)\n  Total:       14000 cycles (292 μs)\n\nPerformance testing complete!\nqfplib has been successfully integrated and tested!\n```\n\n### File Organization\n\nKeep your performance documentation organized:\n```\nrust-poc/\n├── PERFORMANCE_RESULTS_TEMPLATE.md     # Template file\n├── PERFORMANCE_RESULTS_2025-09-03.md   # Your actual results\n├── PERFORMANCE_TESTING_GUIDE.md        # Hardware testing guide\n├── QFPLIB_INTEGRATION_COMPLETE.md      # Technical integration details\n└── performance_data/                    # Raw RTT output files\n    ├── standard_math_output.txt\n    └── qfplib_output.txt\n```\n\n## Quick Checklist\n\n- [ ] Arduino Zero connected and in bootloader mode\n- [ ] Standard firmware deployed and tested\n- [ ] qfplib firmware deployed and tested  \n- [ ] RTT output captured for both versions\n- [ ] Performance results template filled out\n- [ ] Improvement ratios calculated\n- [ ] Analysis sections completed\n- [ ] Project documentation updated\n- [ ] Results committed to git\n\nThis documentation approach will give you a complete record of the qfplib performance benefits and help justify its use in the production firmware.