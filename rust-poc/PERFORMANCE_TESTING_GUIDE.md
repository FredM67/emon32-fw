# Solving the Pending qfplib Performance Testing

## Current Status

✅ **Completed:**
- qfplib integration into Rust project
- FastMath trait abstraction for conditional compilation
- Build system generates qfplib-enabled ARM binaries
- SysTick-based cycle counting for accurate ARM timing
- Two test binaries for comparison (standard vs qfplib)

⏳ **Pending Work:**
- Hardware validation on Arduino Zero
- Real performance measurements and data collection
- Performance comparison analysis
- Documentation of results

## How to Complete the Performance Testing

### Step 1: Hardware Setup

1. **Connect Arduino Zero:**
   ```bash
   # Connect Arduino Zero via USB
   # The board should appear as a USB device
   lsusb | grep Arduino
   ```

2. **Install RTT Viewer:**
   ```bash
   # Install probe-rs for RTT (Real-Time Transfer) debugging
   cargo install probe-rs --features=cli
   ```

### Step 2: Deploy and Test Standard Math Baseline

1. **Enter Bootloader Mode:**
   - Double-press the reset button on Arduino Zero
   - The board should mount as "ARDUINO" drive

2. **Deploy Standard Math Firmware:**
   ```bash
   cd /home/fredm67/git/emon32-fw/rust-poc
   cp emon32-performance-standard.uf2 /media/*/ARDUINO/
   ```

3. **Connect RTT Viewer:**
   ```bash
   # Wait for device to restart, then connect RTT
   probe-rs rtt attach
   ```

4. **Record Results:**
   - Copy the cycle counts and timing data
   - Save as baseline measurements

### Step 3: Deploy and Test qfplib Optimized Version

1. **Enter Bootloader Mode Again:**
   - Double-press reset button
   - Wait for ARDUINO drive to mount

2. **Deploy qfplib Firmware:**
   ```bash
   cp emon32-qfplib-performance.uf2 /media/*/ARDUINO/
   ```

3. **Connect RTT and Record Results:**
   ```bash
   probe-rs rtt attach
   ```

4. **Compare Performance:**
   - Record cycle counts for each operation type
   - Calculate performance improvement percentages

### Step 4: Expected Results Analysis

The qfplib version should show significant improvements:

**Square Root Operations (RMS calculations):**
- Expected: 2-3x faster cycle counts
- Critical for energy monitoring accuracy

**Division Operations (power calculations):**
- Expected: 2-4x faster cycle counts
- Important for efficiency calculations

**Multiplication Operations:**
- Expected: 1.5-2x faster cycle counts
- Used extensively in energy calculations

**Combined Energy Calculations:**
- Expected: Overall 2-3x performance improvement
- Real-world energy monitoring simulation

### Step 5: Build Script for Easy Testing

The `build_qfplib_performance.sh` script automates the entire build process:

```bash
cd /home/fredm67/git/emon32-fw/rust-poc
./build_qfplib_performance.sh
```

This generates both firmware files automatically.

### Step 6: Troubleshooting

**If RTT connection fails:**
```bash
# Check if device is detected
probe-rs list
# Try with specific target
probe-rs rtt attach --chip ATSAMD21J17A
```

**If bootloader doesn't appear:**
- Try double-pressing reset more quickly
- Check USB cable connection
- Try different USB port

**If performance differences are minimal:**
- Ensure qfplib feature is enabled in build
- Verify ARM target compilation
- Check that assembly files are being linked

### Step 7: Document Results

Create a performance comparison table:

| Operation Type | Standard Math (cycles) | qfplib (cycles) | Improvement |
|---------------|------------------------|-----------------|-------------|
| Square Root   | [Record baseline]      | [Record qfplib] | [Calculate] |
| Division      | [Record baseline]      | [Record qfplib] | [Calculate] |
| Multiplication| [Record baseline]      | [Record qfplib] | [Calculate] |
| Combined      | [Record baseline]      | [Record qfplib] | [Calculate] |

## Technical Implementation Details

### SysTick Timer Configuration

The performance test uses ARM SysTick for accurate cycle counting:

```rust
// 48MHz system clock, 24-bit counter
systick.set_clock_source(SystClkSource::Core);
systick.set_reload(0x00FF_FFFF); // Maximum 24-bit value
```

### Test Operations

The benchmark performs realistic energy monitoring operations:

1. **RMS Calculations:** `sqrt(v²)` for voltage/current measurements
2. **Power Calculations:** `V / I` and `V * I` operations
3. **Energy Integration:** Combined operations simulating real workload

### Build Features

- **Standard build:** Uses micromath library (`--features rtt`)
- **qfplib build:** Uses optimized assembly (`--features rtt,qfplib`)

## Next Steps

1. **Immediate:** Run hardware tests on Arduino Zero
2. **Short-term:** Document performance improvements
3. **Medium-term:** Integrate qfplib into main energy calculator
4. **Long-term:** Optimize energy monitoring algorithms with qfplib

The performance testing framework is complete and ready for hardware validation. The pending work is primarily data collection and analysis on real ARM hardware.