//! qfplib-sys: Low-level FFI bindings for qfplib ARM Cortex-M0+ floating-point library
//! 
//! This crate provides safe, zero-cost abstractions over the highly optimized qfplib
//! assembly library for ARM Cortex-M0+ processors. All functions are marked with
//! `#[inline(always)]` to enable maximum LTO optimization and eliminate FFI overhead.
//!
//! # Features
//! 
//! - **Zero-cost abstractions**: FFI calls are inlined away with LTO
//! - **ARM-optimized**: Hand-tuned assembly for Cortex-M0+ processors  
//! - **High precision**: IEEE 754 compliant floating-point operations
//! - **Comprehensive**: Full set of math functions (trig, exp, log, etc.)
//!
//! # Usage
//!
//! ```rust,no_run
//! use qfplib_sys::*;
//! 
//! let x = 1.5f32;
//! let y = 2.5f32;
//! let result = unsafe { qfp_fadd(x, y) };
//! ```
//!
//! # Safety
//!
//! All functions are marked `unsafe` because they are FFI calls to assembly code.
//! However, they are safe to use with valid f32 inputs within normal floating-point ranges.

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

// Only provide bindings for ARM Cortex-M0+ targets
#[cfg(all(target_arch = "arm", feature = "arm-cortex-m0plus"))]
mod bindings {
    //! Raw FFI bindings to qfplib assembly functions
    
    extern "C" {
        /// Add two single-precision floating-point numbers
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_fadd(x: f32, y: f32) -> f32;
        
        /// Subtract two single-precision floating-point numbers
        /// 
        /// # Safety  
        /// Safe to call with any valid f32 values
        pub fn qfp_fsub(x: f32, y: f32) -> f32;
        
        /// Multiply two single-precision floating-point numbers
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_fmul(x: f32, y: f32) -> f32;
        
        /// Divide two single-precision floating-point numbers
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values. Division by zero returns infinity.
        pub fn qfp_fdiv(x: f32, y: f32) -> f32;
        
        /// Compare two single-precision floating-point numbers
        /// Returns: -1 if x < y, 0 if x == y, 1 if x > y
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_fcmp(x: f32, y: f32) -> i32;
        
        /// Compute square root of single-precision floating-point number
        /// 
        /// # Safety
        /// Safe to call with non-negative f32 values. Negative inputs return NaN.
        pub fn qfp_fsqrt(x: f32) -> f32;
        
        /// Compute sine of single-precision floating-point number (radians)
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_fsin(x: f32) -> f32;
        
        /// Compute cosine of single-precision floating-point number (radians)
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_fcos(x: f32) -> f32;
        
        /// Compute tangent of single-precision floating-point number (radians)
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_ftan(x: f32) -> f32;
        
        /// Compute arctangent of y/x in correct quadrant
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values
        pub fn qfp_fatan2(y: f32, x: f32) -> f32;
        
        /// Compute exponential function (e^x)
        /// 
        /// # Safety
        /// Safe to call with any valid f32 values. Large inputs may overflow to infinity.
        pub fn qfp_fexp(x: f32) -> f32;
        
        /// Compute natural logarithm
        /// 
        /// # Safety
        /// Safe to call with positive f32 values. Zero/negative inputs return NaN/infinity.
        pub fn qfp_fln(x: f32) -> f32;
        
        /// Convert signed integer to floating-point
        /// 
        /// # Safety
        /// Safe to call with any i32 value
        pub fn qfp_int2float(x: i32) -> f32;
        
        /// Convert floating-point to signed integer (truncation)
        /// 
        /// # Safety
        /// Safe to call with f32 values within i32 range
        pub fn qfp_float2int(x: f32) -> i32;
        
        /// Convert unsigned integer to floating-point
        /// 
        /// # Safety
        /// Safe to call with any u32 value
        pub fn qfp_uint2float(x: u32) -> f32;
        
        /// Convert floating-point to unsigned integer (truncation)
        /// 
        /// # Safety
        /// Safe to call with non-negative f32 values within u32 range
        pub fn qfp_float2uint(x: f32) -> u32;
        
        /// Convert fixed-point to floating-point
        /// 
        /// # Safety
        /// Safe to call with any i32 value and reasonable fractional bits
        pub fn qfp_fix2float(x: i32, f: i32) -> f32;
        
        /// Convert floating-point to fixed-point
        /// 
        /// # Safety
        /// Safe to call with f32 values within range for given fractional bits
        pub fn qfp_float2fix(x: f32, f: i32) -> i32;
    }
}

// Re-export bindings with LTO-optimized wrappers
#[cfg(all(target_arch = "arm", feature = "arm-cortex-m0plus"))]
pub use self::bindings::*;

// LTO-optimized wrapper functions with maximum inlining
#[cfg(all(target_arch = "arm", feature = "arm-cortex-m0plus"))]
impl LtoOptimized {
    /// LTO-optimized addition with guaranteed inlining
    #[inline(always)]
    pub fn add(x: f32, y: f32) -> f32 {
        unsafe { bindings::qfp_fadd(x, y) }
    }
    
    /// LTO-optimized subtraction with guaranteed inlining
    #[inline(always)]
    pub fn sub(x: f32, y: f32) -> f32 {
        unsafe { bindings::qfp_fsub(x, y) }
    }
    
    /// LTO-optimized multiplication with guaranteed inlining
    #[inline(always)]
    pub fn mul(x: f32, y: f32) -> f32 {
        unsafe { bindings::qfp_fmul(x, y) }
    }
    
    /// LTO-optimized division with guaranteed inlining
    #[inline(always)]
    pub fn div(x: f32, y: f32) -> f32 {
        unsafe { bindings::qfp_fdiv(x, y) }
    }
    
    /// LTO-optimized square root with guaranteed inlining
    #[inline(always)]
    pub fn sqrt(x: f32) -> f32 {
        unsafe { bindings::qfp_fsqrt(x) }
    }
    
    /// LTO-optimized sine with guaranteed inlining
    #[inline(always)]
    pub fn sin(x: f32) -> f32 {
        unsafe { bindings::qfp_fsin(x) }
    }
    
    /// LTO-optimized cosine with guaranteed inlining
    #[inline(always)]
    pub fn cos(x: f32) -> f32 {
        unsafe { bindings::qfp_fcos(x) }
    }
    
    /// LTO-optimized tangent with guaranteed inlining
    #[inline(always)]
    pub fn tan(x: f32) -> f32 {
        unsafe { bindings::qfp_ftan(x) }
    }
    
    /// LTO-optimized arctangent2 with guaranteed inlining
    #[inline(always)]
    pub fn atan2(y: f32, x: f32) -> f32 {
        unsafe { bindings::qfp_fatan2(y, x) }
    }
    
    /// LTO-optimized exponential with guaranteed inlining
    #[inline(always)]
    pub fn exp(x: f32) -> f32 {
        unsafe { bindings::qfp_fexp(x) }
    }
    
    /// LTO-optimized natural logarithm with guaranteed inlining
    #[inline(always)]
    pub fn ln(x: f32) -> f32 {
        unsafe { bindings::qfp_fln(x) }
    }
    
    /// LTO-optimized floating-point comparison with guaranteed inlining
    #[inline(always)]
    pub fn fcmp(x: f32, y: f32) -> i32 {
        unsafe { bindings::qfp_fcmp(x, y) }
    }
    
    /// LTO-optimized integer to float conversion with guaranteed inlining
    #[inline(always)]
    pub fn int2float(x: i32) -> f32 {
        unsafe { bindings::qfp_int2float(x) }
    }
    
    /// LTO-optimized float to integer conversion with guaranteed inlining
    #[inline(always)]
    pub fn float2int(x: f32) -> i32 {
        unsafe { bindings::qfp_float2int(x) }
    }
    
    /// LTO-optimized unsigned integer to float conversion with guaranteed inlining
    #[inline(always)]
    pub fn uint2float(x: u32) -> f32 {
        unsafe { bindings::qfp_uint2float(x) }
    }
    
    /// LTO-optimized float to unsigned integer conversion with guaranteed inlining
    #[inline(always)]
    pub fn float2uint(x: f32) -> u32 {
        unsafe { bindings::qfp_float2uint(x) }
    }
    
    /// LTO-optimized fixed-point to float conversion with guaranteed inlining
    #[inline(always)]
    pub fn fix2float(x: i32, fraction_bits: i32) -> f32 {
        unsafe { bindings::qfp_fix2float(x, fraction_bits) }
    }
    
    /// LTO-optimized float to fixed-point conversion with guaranteed inlining
    #[inline(always)]
    pub fn float2fix(x: f32, fraction_bits: i32) -> i32 {
        unsafe { bindings::qfp_float2fix(x, fraction_bits) }
    }
}

/// Zero-cost LTO-optimized wrapper for qfplib functions
/// 
/// This struct provides a namespace for LTO-optimized function calls
/// that are guaranteed to be inlined when Link Time Optimization is enabled.
#[cfg(all(target_arch = "arm", feature = "arm-cortex-m0plus"))]
pub struct LtoOptimized;

// Provide stub implementations for non-ARM targets
#[cfg(not(all(target_arch = "arm", feature = "arm-cortex-m0plus")))]
pub struct LtoOptimized;

#[cfg(not(all(target_arch = "arm", feature = "arm-cortex-m0plus")))]
impl LtoOptimized {
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn add(x: f32, y: f32) -> f32 { x + y }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn sub(x: f32, y: f32) -> f32 { x - y }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn mul(x: f32, y: f32) -> f32 { x * y }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn div(x: f32, y: f32) -> f32 { x / y }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn sqrt(x: f32) -> f32 { x.sqrt() }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn sin(x: f32) -> f32 { x.sin() }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn cos(x: f32) -> f32 { x.cos() }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn tan(x: f32) -> f32 { x.tan() }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn atan2(y: f32, x: f32) -> f32 { y.atan2(x) }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn exp(x: f32) -> f32 { x.exp() }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn ln(x: f32) -> f32 { x.ln() }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn fcmp(x: f32, y: f32) -> i32 {
        if x < y { -1 } else if x > y { 1 } else { 0 }
    }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn int2float(x: i32) -> f32 { x as f32 }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn float2int(x: f32) -> i32 { x as i32 }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn uint2float(x: u32) -> f32 { x as f32 }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn float2uint(x: f32) -> u32 { x as u32 }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn fix2float(x: i32, fraction_bits: i32) -> f32 {
        (x as f32) / ((1 << fraction_bits) as f32)
    }
    
    /// Stub implementation for non-ARM targets
    #[inline(always)]
    pub fn float2fix(x: f32, fraction_bits: i32) -> i32 {
        (x * ((1 << fraction_bits) as f32)) as i32
    }
}