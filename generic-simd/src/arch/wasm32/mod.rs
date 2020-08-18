//! WebAssembly vector types.

#[cfg(feature = "complex")]
mod complex;
#[cfg(feature = "complex")]
pub use complex::*;

use crate::{
    arch::{generic, Token},
    scalar::Scalar,
    shim::{Shim2, Shim4, ShimToken},
    vector::{width, Native, Vector},
};
use core::arch::wasm32::*;

/// WASM instruction set token.
#[derive(Copy, Clone, Debug)]
pub struct Wasm32(());

impl_token! { Wasm32 => "simd128" }

impl Native<Wasm32> for f32 {
    type Width = width::W4;
}

impl Native<Wasm32> for f64 {
    type Width = width::W2;
}

/// A WASM vector of `f32`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f32x4(v128);

/// A WASM vector of `f64`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f64x2(v128);

impl Scalar<Wasm32, width::W1> for f32 {
    type Vector = ShimToken<generic::f32x1, Self, Wasm32>;
}

impl Scalar<Wasm32, width::W2> for f32 {
    type Vector = ShimToken<Shim2<generic::f32x1, Self>, Self, Wasm32>;
}

impl Scalar<Wasm32, width::W4> for f32 {
    type Vector = f32x4;
}

impl Scalar<Wasm32, width::W8> for f32 {
    type Vector = Shim2<f32x4, f32>;
}

impl Scalar<Wasm32, width::W1> for f64 {
    type Vector = ShimToken<generic::f64x1, Self, Wasm32>;
}

impl Scalar<Wasm32, width::W2> for f64 {
    type Vector = f64x2;
}

impl Scalar<Wasm32, width::W4> for f64 {
    type Vector = Shim2<f64x2, f64>;
}

impl Scalar<Wasm32, width::W8> for f64 {
    type Vector = Shim4<f64x2, f64>;
}

as_slice! { f32x4 }
as_slice! { f64x2 }

unsafe impl Vector for f32x4 {
    type Scalar = f32;
    type Token = Wasm32;
    type Width = width::W4;
    type Underlying = v128;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { f32x4_splat(0.) })
    }

    #[inline]
    fn splat(_: Self::Token, value: Self::Scalar) -> Self {
        Self(unsafe { f32x4_splat(value) })
    }
}

unsafe impl Vector for f64x2 {
    type Scalar = f64;
    type Token = Wasm32;
    type Width = width::W2;
    type Underlying = v128;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { f64x2_splat(0.) })
    }

    #[inline]
    fn splat(_: Self::Token, value: Self::Scalar) -> Self {
        Self(unsafe { f64x2_splat(value) })
    }
}

arithmetic_ops! {
    feature: Wasm32::new_unchecked(),
    for f32x4:
        add -> f32x4_add,
        sub -> f32x4_sub,
        mul -> f32x4_mul,
        div -> f32x4_div
}

arithmetic_ops! {
    feature: Wasm32::new_unchecked(),
    for f64x2:
        add -> f64x2_add,
        sub -> f64x2_sub,
        mul -> f64x2_mul,
        div -> f64x2_div
}

impl core::ops::Neg for f32x4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { f32x4_neg(self.0) })
    }
}

impl core::ops::Neg for f64x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { f64x2_neg(self.0) })
    }
}
