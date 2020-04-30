//! Generic vector types for any platform.

use crate::vector::{FeatureDetect, Vector, Widest};
use num_complex::Complex;

/// A generic instruction set handle supported by all CPUs.
#[derive(Clone, Copy, Debug, Default)]
#[allow(non_camel_case_types)]
pub struct Generic(());

/// A generic vector of one `f32`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f32x1(f32);

/// A generic vector of one `f64`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f64x1(f64);

/// A generic vector of one `Complex<f32>`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x1(Complex<f32>);

/// A generic vector of one `Complex<f64>`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x1(Complex<f64>);

impl FeatureDetect for Generic {
    #[inline]
    fn detect() -> Option<Self> {
        Some(Self(()))
    }

    #[inline]
    unsafe fn new() -> Self {
        Self(())
    }
}

impl Widest<f32> for Generic {
    type Widest = f32x1;
}
impl Widest<f64> for Generic {
    type Widest = f64x1;
}
impl Widest<Complex<f32>> for Generic {
    type Widest = cf32x1;
}
impl Widest<Complex<f64>> for Generic {
    type Widest = cf64x1;
}

macro_rules! implement {
    {
        $vector:ty, $scalar:ty
    } => {
        arithmetic_ops! {
            feature: crate::generic::Generic::new(),
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

        unsafe impl Vector for $vector {
            type Scalar = $scalar;

            type Feature = Generic;

            #[inline]
            fn splat(_: Self::Feature, from: Self::Scalar) -> Self
            {
                Self(from)
            }
        }
    }
}

implement! { f32x1, f32 }
implement! { f64x1, f64 }
implement! { cf32x1, Complex<f32> }
implement! { cf64x1, Complex<f64> }

macro_rules! implement_complex {
    {
        $vector:ty, $real:ty
    } => {
        impl crate::vector::Complex<$real> for $vector {
            #[inline]
            fn mul_i(self) -> Self {
                Self(Complex::new(-self.0.im, self.0.re))
            }

            #[inline]
            fn mul_neg_i(self) -> Self {
                Self(Complex::new(self.0.im, -self.0.re))
            }
        }
    }
}

implement_complex! { cf32x1, f32 }
implement_complex! { cf64x1, f64 }
