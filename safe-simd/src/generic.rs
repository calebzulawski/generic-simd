//! Generic vector types for any platform.

use num_complex::Complex;

/// A generic instruction set handle supported by all CPUs.
#[derive(Clone, Copy, Debug, Default)]
pub struct Generic(());

/// A generic vector of one `f32`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vf32(f32);

/// A generic vector of one `f64`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vf64(f64);

/// A generic vector of one `Complex<f32>`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vcf32(Complex<f32>);

/// A generic vector of one `Complex<f64>`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vcf64(Complex<f64>);

impl crate::vector::Feature for Generic {
    #[inline]
    fn new() -> Option<Self> {
        Some(Self(()))
    }

    #[inline]
    unsafe fn new_unchecked() -> Self {
        Self(())
    }
}

impl crate::vector::Capability<f32> for Generic {
    type Vector = Vf32;
}

impl crate::vector::Capability<f64> for Generic {
    type Vector = Vf64;
}

impl crate::vector::Capability<Complex<f32>> for Generic {
    type Vector = Vcf32;
}

impl crate::vector::Capability<Complex<f64>> for Generic {
    type Vector = Vcf64;
}

macro_rules! implement {
    {
        $vector:ty, $scalar:ty
    } => {
        arithmetic_ops! {
            for $vector:
                add -> core::ops::Add::add,
                sub -> core::ops::Sub::sub,
                mul -> core::ops::Mul::mul,
                div -> core::ops::Div::div
        }

        impl core::ops::Neg for $vector {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        as_slice! { $vector }

        unsafe impl crate::vector::VectorCore for $vector {
            type Scalar = $scalar;

            #[inline]
            unsafe fn splat(from: Self::Scalar) -> Self {
                Self(from)
            }
        }
    }
}

implement! { Vf32, f32 }
implement! { Vf64, f64 }
implement! { Vcf32, Complex<f32> }
implement! { Vcf64, Complex<f64> }
