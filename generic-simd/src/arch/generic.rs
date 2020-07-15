//! Generic vector types for any platform.

use crate::arch::Token;
use crate::shim::{Shim2, Shim4, Shim8};
use crate::vector::{width, Native, ScalarSized, Vector};

#[cfg(feature = "complex")]
use num_complex::Complex;

/// Generic instruction set handle.
#[derive(Copy, Clone, Debug)]
pub struct Generic;

unsafe impl Token for Generic {
    fn new() -> Option<Self> {
        Some(Self)
    }

    unsafe fn new_unchecked() -> Self {
        Self
    }
}

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
///
/// Requires feature `"complex"`.
#[cfg(feature = "complex")]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x1(Complex<f32>);

/// A generic vector of one `Complex<f64>`.
///
/// Requires feature `"complex"`.
#[cfg(feature = "complex")]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x1(Complex<f64>);

macro_rules! implement {
    {
        $vector:ty, $scalar:ty
    } => {
        impl ScalarSized<Generic, width::W1> for $scalar {
            type Token = Generic;
            type Vector = $vector;
        }

        impl ScalarSized<Generic, width::W2> for $scalar {
            type Token = Generic;
            type Vector = Shim2<$vector, $scalar>;
        }

        impl ScalarSized<Generic, width::W4> for $scalar {
            type Token = Generic;
            type Vector = Shim4<$vector, $scalar>;
        }

        impl ScalarSized<Generic, width::W8> for $scalar {
            type Token = Generic;
            type Vector = Shim8<$vector, $scalar>;
        }

        impl Native<Generic> for $scalar {
            type Width = width::W1;
        }
    }
}

implement! { f32x1, f32 }
implement! { f64x1, f64 }

#[cfg(feature = "complex")]
implement! { cf32x1, Complex<f32> }
#[cfg(feature = "complex")]
implement! { cf64x1, Complex<f64> }

macro_rules! implement {
    {
        $vector:ty, $scalar:ty
    } => {
        arithmetic_ops! {
            feature: Generic::new_unchecked(),
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

            type Token = Generic;

            type Width = crate::vector::width::W1;

            #[inline]
            fn splat(_: impl Into<Self::Token>, from: Self::Scalar) -> Self
            {
                Self(from)
            }
        }
    }
}

implement! { f32x1, f32 }
implement! { f64x1, f64 }

#[cfg(feature = "complex")]
implement! { cf32x1, Complex<f32> }
#[cfg(feature = "complex")]
implement! { cf64x1, Complex<f64> }

#[cfg(feature = "complex")]
macro_rules! implement_complex {
    {
        $vector:ty, $real:ty
    } => {
        impl crate::vector::Complex for $vector {
            type RealScalar = $real;

            #[inline]
            fn conj(self) -> Self {
                Self(Complex::new(self.0.re, -self.0.im))
            }

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

#[cfg(feature = "complex")]
implement_complex! { cf32x1, f32 }
#[cfg(feature = "complex")]
implement_complex! { cf64x1, f64 }
