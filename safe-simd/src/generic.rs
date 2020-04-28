//! Generic vector types for any platform.

use crate::vector::{Feature, ProvidedBy, Vector, Widest};
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

impl Feature for Generic {
    #[inline]
    fn new() -> Option<Self> {
        Some(Self(()))
    }

    #[inline]
    unsafe fn new_unchecked() -> Self {
        Self(())
    }
}

unsafe impl<F> ProvidedBy<F> for f32x1 where F: Feature {}
unsafe impl<F> ProvidedBy<F> for f64x1 where F: Feature {}
unsafe impl<F> ProvidedBy<F> for cf32x1 where F: Feature {}
unsafe impl<F> ProvidedBy<F> for cf64x1 where F: Feature {}

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

            #[inline]
            fn splat<F>(_: F, from: Self::Scalar) -> Self
            where
                F: Feature,
                Self: ProvidedBy<F>,
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
