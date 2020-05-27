//! Generic vector types for any platform.

use crate::shim::{Shim2, Shim4, Shim8};
use crate::vector::{Handle, Vector};
use arch_types::{marker::Superset, Features};
use num_complex::Complex;

arch_types::new_features_type! { #[doc = "A generic instruction set handle supported by all CPUs."] pub Generic => }

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

impl Handle<f32> for Generic {
    type VectorNative = f32x1;
    type Feature1 = Generic;
    type Feature2 = Generic;
    type Feature4 = Generic;
    type Feature8 = Generic;
    type Vector1 = f32x1;
    type Vector2 = Shim2<f32x1, f32>;
    type Vector4 = Shim4<f32x1, f32>;
    type Vector8 = Shim8<f32x1, f32>;
}

impl Handle<f64> for Generic {
    type VectorNative = f64x1;
    type Feature1 = Generic;
    type Feature2 = Generic;
    type Feature4 = Generic;
    type Feature8 = Generic;
    type Vector1 = f64x1;
    type Vector2 = Shim2<f64x1, f64>;
    type Vector4 = Shim4<f64x1, f64>;
    type Vector8 = Shim8<f64x1, f64>;
}

impl Handle<Complex<f32>> for Generic {
    type VectorNative = cf32x1;
    type Feature1 = Generic;
    type Feature2 = Generic;
    type Feature4 = Generic;
    type Feature8 = Generic;
    type Vector1 = cf32x1;
    type Vector2 = Shim2<cf32x1, Complex<f32>>;
    type Vector4 = Shim4<cf32x1, Complex<f32>>;
    type Vector8 = Shim8<cf32x1, Complex<f32>>;
}

impl Handle<Complex<f64>> for Generic {
    type VectorNative = cf64x1;
    type Feature1 = Generic;
    type Feature2 = Generic;
    type Feature4 = Generic;
    type Feature8 = Generic;
    type Vector1 = cf64x1;
    type Vector2 = Shim2<cf64x1, Complex<f64>>;
    type Vector4 = Shim4<cf64x1, Complex<f64>>;
    type Vector8 = Shim8<cf64x1, Complex<f64>>;
}

macro_rules! implement {
    {
        $vector:ty, $scalar:ty
    } => {
        arithmetic_ops! {
            feature: crate::generic::Generic::new_unchecked(),
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
            fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self
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
        impl crate::vector::Complex for $vector {
            type RealScalar = $real;

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
