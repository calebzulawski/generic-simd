//! arm/aarch64 vector types.

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

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use core::arch::arm::*;

/// NEON instruction set token.
#[derive(Copy, Clone, Debug)]
pub struct Neon(());

impl_token! { Neon => "neon" }

impl Native<Neon> for f32 {
    type Width = width::W4;
}

impl Native<Neon> for f64 {
    type Width = width::W2;
}

/// A NEON vector of 2 `f32`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f32x2(float32x2_t);

/// A NEON vector of 4 `f32`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f32x4(float32x4_t);

/// A NEON vector of 2 `f64`s.
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f64x2(float64x2_t);

impl Scalar<Neon, width::W1> for f32 {
    type Vector = ShimToken<generic::f32x1, Self, Neon>;
}

impl Scalar<Neon, width::W2> for f32 {
    type Vector = f32x2;
}

impl Scalar<Neon, width::W4> for f32 {
    type Vector = f32x4;
}

impl Scalar<Neon, width::W8> for f32 {
    type Vector = Shim2<f32x4, Self>;
}

impl Scalar<Neon, width::W1> for f64 {
    type Vector = ShimToken<generic::f64x1, Self, Neon>;
}

#[cfg(target_arch = "arm")]
impl Scalar<Neon, width::W2> for f64 {
    type Vector = ShimToken<generic::f64x2, Self, Neon>;
}

#[cfg(target_arch = "aarch64")]
impl Scalar<Neon, width::W2> for f64 {
    type Vector = f64x2;
}

impl Scalar<Neon, width::W4> for f64 {
    type Vector = Shim2<<Self as Scalar<Neon, width::W2>>::Vector, Self>;
}

impl Scalar<Neon, width::W8> for f64 {
    type Vector = Shim4<<Self as Scalar<Neon, width::W2>>::Vector, Self>;
}

arithmetic_ops! {
    feature: Neon::new_unchecked(),
    for f32x2:
        add -> vadd_f32,
        sub -> vsub_f32,
        mul -> vmul_f32,
        div -> default
}

arithmetic_ops! {
    feature: Neon::new_unchecked(),
    for f32x4:
        add -> vaddq_f32,
        sub -> vsubq_f32,
        mul -> vmulq_f32,
        div -> default
}

#[cfg(target_arch = "aarch64")]
arithmetic_ops! {
    feature: Neon::new_unchecked(),
    for f64x2:
        add -> vaddq_f64,
        sub -> vsubq_f64,
        mul -> vmulq_f64,
        div -> default
}

impl core::ops::Neg for f32x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        for v in self.as_slice_mut() {
            *v = -*v;
        }
        self
    }
}

impl core::ops::Neg for f32x4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        for v in self.as_slice_mut() {
            *v = -*v;
        }
        self
    }
}

#[cfg(target_arch = "aarch64")]
impl core::ops::Neg for f64x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        for v in self.as_slice_mut() {
            *v = -*v;
        }
        self
    }
}

as_slice! { f32x2 }
as_slice! { f32x4 }

#[cfg(target_arch = "aarch64")]
as_slice! { f64x2 }

unsafe impl Vector for f32x2 {
    type Scalar = f32;

    type Token = Neon;

    type Width = crate::vector::width::W2;

    type Underlying = float32x2_t;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        // TODO use vdup
        Self(unsafe { core::mem::zeroed() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        // TODO use vdup
        let v: Self = unsafe { core::mem::zeroed() };
        v[0] = from;
        v[1] = from;
        v
    }
}

unsafe impl Vector for f32x4 {
    type Scalar = f32;

    type Token = Neon;

    type Width = crate::vector::width::W4;

    type Underlying = float32x4_t;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        // TODO use vdup
        Self(unsafe { core::mem::zeroed() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        // TODO use vdup
        let v: Self = unsafe { core::mem::zeroed() };
        v[0] = from;
        v[1] = from;
        v[2] = from;
        v[3] = from;
        v
    }
}

#[cfg(target_arch = "aarch64")]
unsafe impl Vector for f64x2 {
    type Scalar = f64;

    type Token = Neon;

    type Width = crate::vector::width::W2;

    type Underlying = float64x2_t;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        // TODO use vdup
        Self(unsafe { core::mem::zeroed() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        // TODO use vdup
        let v: Self = unsafe { core::mem::zeroed() };
        v[0] = from;
        v[1] = from;
        v
    }
}
