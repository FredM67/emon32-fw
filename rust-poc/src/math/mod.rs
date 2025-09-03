// qfplib Rust bindings for fast floating-point math on ARM Cortex-M0+
// These are safe Rust wrappers around the highly optimized qfplib assembly library

#[cfg(all(target_arch = "arm", feature = "qfplib"))]
mod qfplib_bindings {
    // External C functions from qfplib assembly
    extern "C" {
        pub fn qfp_fadd(x: f32, y: f32) -> f32;
        pub fn qfp_fsub(x: f32, y: f32) -> f32;
        pub fn qfp_fmul(x: f32, y: f32) -> f32;
        pub fn qfp_fdiv(x: f32, y: f32) -> f32;
        pub fn qfp_fcmp(x: f32, y: f32) -> i32;
        pub fn qfp_fsqrt(x: f32) -> f32;
        pub fn qfp_fsin(x: f32) -> f32;
        pub fn qfp_fcos(x: f32) -> f32;
        pub fn qfp_ftan(x: f32) -> f32;
        pub fn qfp_fatan2(y: f32, x: f32) -> f32;
        pub fn qfp_fexp(x: f32) -> f32;
        pub fn qfp_fln(x: f32) -> f32;
        pub fn qfp_int2float(x: i32) -> f32;
        pub fn qfp_float2int(x: f32) -> i32;
        pub fn qfp_uint2float(x: u32) -> f32;
        pub fn qfp_float2uint(x: f32) -> u32;
        pub fn qfp_fix2float(x: i32, f: i32) -> f32;
        pub fn qfp_float2fix(x: f32, f: i32) -> i32;
    }
}

/// High-performance floating-point math trait
/// Uses qfplib on ARM Cortex-M, falls back to micromath/std on other platforms
pub trait FastMath {
    fn fast_add(self, other: Self) -> Self;
    fn fast_sub(self, other: Self) -> Self;
    fn fast_mul(self, other: Self) -> Self;
    fn fast_div(self, other: Self) -> Self;
    fn fast_sqrt(self) -> Self;
    fn fast_sin(self) -> Self;
    fn fast_cos(self) -> Self;
    fn fast_tan(self) -> Self;
    fn fast_atan2(self, x: Self) -> Self;
    fn fast_exp(self) -> Self;
    fn fast_ln(self) -> Self;
    fn fast_abs(self) -> Self;
    fn fast_min(self, other: Self) -> Self;
    fn fast_max(self, other: Self) -> Self;
}

impl FastMath for f32 {
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_add(self, other: Self) -> Self {
        unsafe { qfplib_bindings::qfp_fadd(self, other) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_add(self, other: Self) -> Self {
        self + other // Standard Rust addition
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_sub(self, other: Self) -> Self {
        unsafe { qfplib_bindings::qfp_fsub(self, other) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_sub(self, other: Self) -> Self {
        self - other
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_mul(self, other: Self) -> Self {
        unsafe { qfplib_bindings::qfp_fmul(self, other) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_mul(self, other: Self) -> Self {
        self * other
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_div(self, other: Self) -> Self {
        unsafe { qfplib_bindings::qfp_fdiv(self, other) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_div(self, other: Self) -> Self {
        self / other
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_sqrt(self) -> Self {
        unsafe { qfplib_bindings::qfp_fsqrt(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_sqrt(self) -> Self {
        use micromath::F32Ext;
        self.sqrt()
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_sin(self) -> Self {
        unsafe { qfplib_bindings::qfp_fsin(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_sin(self) -> Self {
        use micromath::F32Ext;
        self.sin()
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_cos(self) -> Self {
        unsafe { qfplib_bindings::qfp_fcos(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_cos(self) -> Self {
        use micromath::F32Ext;
        self.cos()
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_tan(self) -> Self {
        unsafe { qfplib_bindings::qfp_ftan(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_tan(self) -> Self {
        use micromath::F32Ext;
        self.tan()
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_atan2(self, x: Self) -> Self {
        unsafe { qfplib_bindings::qfp_fatan2(self, x) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_atan2(self, x: Self) -> Self {
        use micromath::F32Ext;
        self.atan2(x)
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_exp(self) -> Self {
        unsafe { qfplib_bindings::qfp_fexp(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_exp(self) -> Self {
        use micromath::F32Ext;
        self.exp()
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_ln(self) -> Self {
        unsafe { qfplib_bindings::qfp_fln(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_ln(self) -> Self {
        use micromath::F32Ext;
        self.ln()
    }

    // Additional utility functions that use fast comparison
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_abs(self) -> Self {
        // Use qfp comparison for sign check
        if unsafe { qfplib_bindings::qfp_fcmp(self, 0.0) } < 0 {
            unsafe { qfplib_bindings::qfp_fsub(0.0, self) }
        } else {
            self
        }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_abs(self) -> Self {
        self.abs()
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_min(self, other: Self) -> Self {
        if unsafe { qfplib_bindings::qfp_fcmp(self, other) } <= 0 {
            self
        } else {
            other
        }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_min(self, other: Self) -> Self {
        self.min(other)
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    #[inline]
    fn fast_max(self, other: Self) -> Self {
        if unsafe { qfplib_bindings::qfp_fcmp(self, other) } >= 0 {
            self
        } else {
            other
        }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    #[inline]
    fn fast_max(self, other: Self) -> Self {
        self.max(other)
    }
}

/// Type conversion helpers for qfplib
pub trait FastConvert<T> {
    fn to_fast_float(self) -> f32;
    fn from_fast_float(val: f32) -> T;
}

impl FastConvert<i32> for i32 {
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    fn to_fast_float(self) -> f32 {
        unsafe { qfplib_bindings::qfp_int2float(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    fn to_fast_float(self) -> f32 {
        self as f32
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    fn from_fast_float(val: f32) -> i32 {
        unsafe { qfplib_bindings::qfp_float2int(val) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    fn from_fast_float(val: f32) -> i32 {
        val as i32
    }
}

impl FastConvert<u32> for u32 {
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    fn to_fast_float(self) -> f32 {
        unsafe { qfplib_bindings::qfp_uint2float(self) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    fn to_fast_float(self) -> f32 {
        self as f32
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    fn from_fast_float(val: f32) -> u32 {
        unsafe { qfplib_bindings::qfp_float2uint(val) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    fn from_fast_float(val: f32) -> u32 {
        val as u32
    }
}

/// Fixed-point conversion helpers
pub trait FastFixedPoint {
    fn to_fixed_float(self, fraction_bits: i32) -> f32;
    fn from_fixed_float(val: f32, fraction_bits: i32) -> Self;
}

impl FastFixedPoint for i32 {
    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    fn to_fixed_float(self, fraction_bits: i32) -> f32 {
        unsafe { qfplib_bindings::qfp_fix2float(self, fraction_bits) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    fn to_fixed_float(self, fraction_bits: i32) -> f32 {
        (self as f32) / ((1 << fraction_bits) as f32)
    }

    #[cfg(all(target_arch = "arm", feature = "qfplib"))]
    fn from_fixed_float(val: f32, fraction_bits: i32) -> Self {
        unsafe { qfplib_bindings::qfp_float2fix(val, fraction_bits) }
    }

    #[cfg(not(all(target_arch = "arm", feature = "qfplib")))]
    fn from_fixed_float(val: f32, fraction_bits: i32) -> Self {
        (val * ((1 << fraction_bits) as f32)) as i32
    }
}

#[cfg(all(test, not(target_arch = "arm")))]
mod tests {
    use super::*;

    #[test]
    fn test_fast_math_operations() {
        let a = 3.0f32;
        let b = 4.0f32;

        // Test basic operations
        assert!((a.fast_add(b) - 7.0).abs() < 1e-6);
        assert!((a.fast_mul(b) - 12.0).abs() < 1e-6);
        assert!((b.fast_div(a) - 4.0 / 3.0).abs() < 1e-6);

        // Test sqrt
        let c = 9.0f32;
        assert!((c.fast_sqrt() - 3.0).abs() < 1e-6);

        // Test trig functions
        let pi_2 = core::f32::consts::FRAC_PI_2;
        assert!((pi_2.fast_sin() - 1.0).abs() < 1e-4);
        assert!((0.0f32.fast_cos() - 1.0).abs() < 1e-6);

        // Test utility functions
        assert_eq!((-5.0f32).fast_abs(), 5.0);
        assert_eq!(a.fast_min(b), a);
        assert_eq!(a.fast_max(b), b);
    }

    #[test]
    fn test_conversions() {
        let i = 42i32;
        let f = i.to_fast_float();
        let back = i32::from_fast_float(f);
        assert_eq!(back, i);

        let u = 100u32;
        let f = u.to_fast_float();
        let back = u32::from_fast_float(f);
        assert_eq!(back, u);
    }

    #[test]
    fn test_fixed_point() {
        let fixed_val = 1024i32; // 1.0 in Q10 format (10 fraction bits)
        let float_val = fixed_val.to_fixed_float(10);
        assert!((float_val - 1.0).abs() < 1e-6);

        let back = i32::from_fixed_float(float_val, 10);
        assert_eq!(back, fixed_val);
    }
}
