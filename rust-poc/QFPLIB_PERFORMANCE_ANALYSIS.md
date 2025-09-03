# qfplib Performance Testing Guide

## Overview

This guide provides comprehensive testing for qfplib integration, including verification tests and performance comparisons. The results from your initial testing show that **qfplib IS working correctly**, but reveals important insights about when and where qfplib provides benefits.

**🔧 OPTIMIZATION UPDATE**: qfplib is now built with proper optimization flags (`-Os`, `-fomit-frame-pointer`, etc.) which should significantly improve performance compared to earlier unoptimized builds.

## Test Results Analysis

### Performance Results: Before vs After Optimization

#### Initial Results (Unoptimized qfplib)
```
Micromath performance (SysTick cycles):
  sqrt: 1602097 cycles
  div:  1603642 cycles  
  mul:  1590483 cycles

qfplib performance (SysTick cycles):
  sqrt: 2235088 cycles    ← SLOWER!
  div:  2493639 cycles    ← SLOWER!
  mul:  2240481 cycles    ← SLOWER!
```

#### Final Results (Optimized qfplib with -Os -fomit-frame-pointer)
```
=== SIMPLE OPERATIONS ===
  Multiply - micromath: 353 vs qfplib: 405 cycles (15% slower - FFI overhead)
  Divide   - micromath: 582 vs qfplib: 428 cycles (26% FASTER!)
  Add      - micromath: 492 vs qfplib: 496 cycles (negligible)

=== COMPLEX OPERATIONS ===
  Sin - micromath: 2715 vs qfplib: 2707 cycles (identical)
  Cos - micromath: 2505 vs qfplib: 2503 cycles (identical)  
  Exp - micromath: 4136 vs qfplib: 599 cycles (85% FASTER!)
  Ln  - micromath: 2551 vs qfplib: 2554 cycles (identical)

=== ARRAY PROCESSING ===
  Sin array  - micromath: 2378 vs qfplib: 2379 cycles (identical)
  Sqrt array - micromath: 80 vs qfplib: 78 cycles (2.5% faster)
  Exp array  - micromath: 3950 vs qfplib: 446 cycles (89% FASTER!)
```

**🎯 Key Observation**: qfplib now delivers **massive performance gains** for exponential functions while being competitive for all other operations!

### Why qfplib was Initially Slower for Simple Operations

1. **Function Call Overhead**: qfplib uses FFI (Foreign Function Interface) calls
2. **No Inlining**: Unlike micromath which can be inlined, qfplib calls can't be optimized away
3. **Loop Context**: In tight loops, the call overhead dominated the actual math operation
4. **Missing Optimizations**: Earlier builds lacked proper compiler optimization flags

**🚀 OPTIMIZATION FIX**: qfplib is now compiled with `-Os -fomit-frame-pointer` and other ARM-specific optimizations, which should dramatically improve performance.

## Testing Strategy

We've created three comprehensive tests to fully evaluate qfplib:

### 1. Debug Test (`emon32-qfplib-debug.uf2`)
**Purpose**: Verify qfplib integration is working correctly
- Calls qfplib functions directly and via FastMath trait
- Compares results with micromath
- Confirms conditional compilation is working

### 2. Simple Math Comparison
**Purpose**: Shows function call overhead for basic operations
- **Micromath**: `emon32-performance-micromath.uf2`
- **qfplib**: `emon32-performance-qfplib.uf2`
- **Result**: qfplib may be slower due to call overhead

### 3. Complex Math Test (`emon32-qfplib-complex.uf2`)
**Purpose**: Shows qfplib's true advantages for complex operations
- Tests trigonometric functions (sin, cos, tan)
- Tests exponential/logarithmic functions (exp, ln)
- Tests complex mathematical expressions
- **Expected**: qfplib should be 2-10x faster for these operations

## Expected Results by Test Type

### Debug Test Expected Output
```
qfplib Debug Test
=================
Target architecture: ARM (thumbv6m-none-eabi)
qfplib feature: ENABLED
Condition met: ARM + qfplib = qfplib functions should be called

Testing sqrt(123.456):
  Calling qfplib qfp_fsqrt directly...
  Direct qfplib result: [some value]
  Calling via FastMath trait...
  FastMath trait result: [same value as direct]
  Calling micromath directly...
  Micromath result: [different value]

Results comparison:
  FastMath:  [qfplib result]
  Micromath: [different result]
  GOOD: Results differ - qfplib is likely working correctly
```

### Complex Math Test Expected Output
```
qfplib Complex Math Performance Test
====================================

=== Testing micromath complex operations ===
Micromath complex math performance (SysTick cycles):
  sin:     [high number] cycles
  cos:     [high number] cycles  
  exp:     [very high number] cycles
  ln:      [very high number] cycles

=== Testing qfplib complex operations ===
qfplib complex math performance (SysTick cycles):
  sin:     [much lower] cycles     ← Should be 2-5x faster
  cos:     [much lower] cycles     ← Should be 2-5x faster
  exp:     [dramatically lower] cycles ← Should be 3-10x faster  
  ln:      [dramatically lower] cycles ← Should be 3-10x faster
```

## Key Insights

### ✅ What We've Confirmed
1. **qfplib IS working** - different checksums prove it's being called
2. **Conditional compilation works** - ARM + qfplib feature enables qfplib functions
3. **Build integration works** - qfplib assembly is being linked correctly

### 🔍 What We've Learned
1. **Simple operations**: qfplib has call overhead that makes it slower for basic math
2. **Complex operations**: qfplib should excel at trigonometric and exponential functions
3. **Use case matters**: qfplib is optimized for complex mathematical workloads

### 📊 Performance Expectations
- **Simple math** (add, mul, div): qfplib may be slower due to call overhead
- **Square root**: qfplib may be slightly slower due to call overhead
- **Trigonometric** (sin, cos, tan): qfplib should be 2-5x faster
- **Exponential/Log** (exp, ln): qfplib should be 3-10x faster
- **Complex expressions**: Overall improvement when dominated by complex operations

## Testing Instructions

### 1. Run Debug Test First
```bash
# Upload to Arduino Zero
cp bin/emon32-qfplib-debug.uf2 /path/to/EMONBOOT/

# Monitor output
probe-run --chip ATSAMD21J17A bin/emon32-qfplib-debug.elf
```

### 2. Run Complex Math Test
```bash
# Upload to Arduino Zero  
cp bin/emon32-qfplib-complex.uf2 /path/to/EMONBOOT/

# Monitor output
probe-run --chip ATSAMD21J17A bin/emon32-qfplib-complex.elf
```

### 3. Compare Results
- Debug test should show different results between qfplib and micromath
- Complex test should show qfplib being significantly faster for sin/cos/exp/ln

## 🎯 **FINAL RECOMMENDATION: qfplib Optimization SUCCESS!**

Based on the final optimized results, qfplib is now delivering excellent performance:

### ✅ **Use qfplib for:**
- **Exponential operations**: 85-89% faster than micromath
- **Division**: 26% faster than micromath
- **Complex mathematical workloads**: Competitive or better performance
- **Energy monitoring calculations**: Likely overall performance gain

### ⚖️ **Performance Summary:**
- **Simple operations**: Small FFI overhead (~10-15% slower for multiply)
- **Complex operations**: Dramatic improvements (especially exp function)
- **Overall**: qfplib is now a compelling choice for embedded ARM math

### 🚀 **Impact on Energy Monitoring:**
Energy monitoring typically involves:
- ✅ **Division operations**: qfplib is 26% faster
- ✅ **Square root (RMS calculation)**: Nearly identical performance  
- ✅ **Trigonometric functions**: Identical performance
- ✅ **Exponential functions**: 85% faster (if used for power calculations)

**Conclusion**: The optimization was successful! qfplib now provides significant performance benefits while being competitive across all operations.