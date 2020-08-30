use core::arch::wasm32::*;

use crate::{
    arch::{generic, wasm::*, Token},
    scalar::Scalar,
    shim::{Shim2, Shim4, Shim8, ShimToken},
    vector::{width, Native, Vector},
};
use num_complex::Complex;

impl Native<Simd128> for Complex<f32> {
    type Width = width::W2;
}

impl Native<Simd128> for Complex<f64> {
    type Width = width::W1;
}

/// A WASM vector of `Complex<f32>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x2(v128);

/// A WASM vector of `Complex<f64>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x1(v128);

impl Scalar<Simd128, width::W1> for Complex<f32> {
    type Vector = ShimToken<generic::cf32x1, Self, Simd128>;
}

impl Scalar<Simd128, width::W2> for Complex<f32> {
    type Vector = cf32x2;
}

impl Scalar<Simd128, width::W4> for Complex<f32> {
    type Vector = Shim2<cf32x2, Complex<f32>>;
}

impl Scalar<Simd128, width::W8> for Complex<f32> {
    type Vector = Shim4<cf32x2, Complex<f32>>;
}

impl Scalar<Simd128, width::W1> for Complex<f64> {
    type Vector = cf64x1;
}

impl Scalar<Simd128, width::W2> for Complex<f64> {
    type Vector = Shim2<cf64x1, Complex<f64>>;
}

impl Scalar<Simd128, width::W4> for Complex<f64> {
    type Vector = Shim4<cf64x1, Complex<f64>>;
}

impl Scalar<Simd128, width::W8> for Complex<f64> {
    type Vector = Shim8<cf64x1, Complex<f64>>;
}

as_slice! { cf32x2 }
as_slice! { cf64x1 }

unsafe impl Vector for cf32x2 {
    type Scalar = Complex<f32>;
    type Token = Simd128;
    type Width = width::W2;
    type Underlying = v128;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { f32x4_splat(0.) })
    }

    #[inline]
    fn splat(_: Self::Token, value: Self::Scalar) -> Self {
        Self(unsafe { f32x4_const(value.re, value.im, value.re, value.im) })
    }
}

unsafe impl Vector for cf64x1 {
    type Scalar = Complex<f64>;
    type Token = Simd128;
    type Width = width::W1;
    type Underlying = v128;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { f64x2_splat(0.) })
    }

    #[inline]
    fn splat(_: Self::Token, value: Self::Scalar) -> Self {
        Self(unsafe { f64x2_const(value.re, value.im) })
    }
}

arithmetic_ops! {
    feature: Simd128::new_unchecked(),
    for cf32x2:
        add -> (f32x4_add),
        sub -> (f32x4_sub),
        mul -> (cf32x2_mul),
        div -> (cf32x2_div)
}

arithmetic_ops! {
    feature: Simd128::new_unchecked(),
    for cf64x1:
        add -> (f64x2_add),
        sub -> (f64x2_sub),
        mul -> (cf64x1_mul),
        div -> (cf64x1_div)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn f32x4_ldup(x: v128) -> v128 {
    v32x4_shuffle::<0, 0, 2, 2>(x, x)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn f32x4_hdup(x: v128) -> v128 {
    v32x4_shuffle::<1, 1, 3, 3>(x, x)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn f64x2_ldup(x: v128) -> v128 {
    v64x2_shuffle::<0, 0>(x, x)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn f64x2_hdup(x: v128) -> v128 {
    v64x2_shuffle::<1, 1>(x, x)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn f32x4_addsub(a: v128, b: v128) -> v128 {
    let add = f32x4_add(a, b);
    let sub = f32x4_sub(a, b);
    v32x4_shuffle::<0, 5, 2, 7>(sub, add)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn f64x2_addsub(a: v128, b: v128) -> v128 {
    let add = f64x2_add(a, b);
    let sub = f64x2_sub(a, b);
    v64x2_shuffle::<0, 3>(sub, add)
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn cf32x2_mul(a: v128, b: v128) -> v128 {
    let re = f32x4_ldup(a);
    let im = f32x4_hdup(a);
    let sh = v32x4_shuffle::<1, 0, 3, 2>(b, b);
    f32x4_addsub(f32x4_mul(re, b), f32x4_mul(im, sh))
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn cf64x1_mul(a: v128, b: v128) -> v128 {
    let re = f64x2_ldup(a);
    let im = f64x2_hdup(a);
    let sh = v64x2_shuffle::<1, 0>(b, b);
    f64x2_addsub(f64x2_mul(re, b), f64x2_mul(im, sh))
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn cf32x2_div(a: v128, b: v128) -> v128 {
    let b_re = f32x4_ldup(b);
    let b_im = f32x4_hdup(b);
    let a_flip = v32x4_shuffle::<1, 0, 3, 2>(a, a);
    let norm_sqr = f32x4_add(f32x4_mul(b_re, b_re), f32x4_mul(b_im, b_im));
    f32x4_div(
        f32x4_addsub(f32x4_mul(a, b_re), f32x4_neg(f32x4_mul(a_flip, b_im))),
        norm_sqr,
    )
}

#[target_feature(enable = "simd128")]
#[inline]
unsafe fn cf64x1_div(a: v128, b: v128) -> v128 {
    let b_re = f64x2_ldup(b);
    let b_im = f64x2_hdup(b);
    let a_flip = v64x2_shuffle::<1, 0>(a, a);
    let norm_sqr = f64x2_add(f64x2_mul(b_re, b_re), f64x2_mul(b_im, b_im));
    f64x2_div(
        f64x2_addsub(f64x2_mul(a, b_re), f64x2_neg(f64x2_mul(a_flip, b_im))),
        norm_sqr,
    )
}

impl core::ops::Neg for cf32x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { f32x4_neg(self.0) })
    }
}

impl core::ops::Neg for cf64x1 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { f64x2_neg(self.0) })
    }
}
