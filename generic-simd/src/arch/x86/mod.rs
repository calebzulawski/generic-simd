//! x86/x86-64 vector types.

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

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// SSE4.1 instruction set token.
#[derive(Copy, Clone, Debug)]
pub struct Sse(());

/// AVX instruction set token.
#[derive(Copy, Clone, Debug)]
pub struct Avx(());

impl_token! { Sse => "sse4.1" }
impl_token! { Avx => "avx" }

impl core::convert::From<Avx> for Sse {
    #[inline]
    fn from(_: Avx) -> Sse {
        unsafe { Sse::new_unchecked() }
    }
}

impl Native<Sse> for f32 {
    type Width = width::W4;
}

impl Native<Sse> for f64 {
    type Width = width::W2;
}

impl Native<Avx> for f32 {
    type Width = width::W8;
}

impl Native<Avx> for f64 {
    type Width = width::W4;
}

/// An SSE vector of `f32`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f32x4(__m128);

/// An SSE vector of `f64`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f64x2(__m128d);

/// An AVX vector of `f32`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f32x8(__m256);

/// An AVX vector of `f64`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f64x4(__m256d);

impl Scalar<Sse, width::W1> for f32 {
    type Vector = ShimToken<generic::f32x1, Self, Sse>;
}

impl Scalar<Sse, width::W2> for f32 {
    type Vector = ShimToken<Shim2<generic::f32x1, Self>, Self, Sse>;
}

impl Scalar<Sse, width::W4> for f32 {
    type Vector = f32x4;
}

impl Scalar<Sse, width::W8> for f32 {
    type Vector = Shim2<f32x4, f32>;
}

impl Scalar<Sse, width::W1> for f64 {
    type Vector = ShimToken<generic::f64x1, Self, Sse>;
}

impl Scalar<Sse, width::W2> for f64 {
    type Vector = f64x2;
}

impl Scalar<Sse, width::W4> for f64 {
    type Vector = Shim2<f64x2, f64>;
}

impl Scalar<Sse, width::W8> for f64 {
    type Vector = Shim4<f64x2, f64>;
}

impl Scalar<Avx, width::W1> for f32 {
    type Vector = ShimToken<generic::f32x1, Self, Avx>;
}

impl Scalar<Avx, width::W2> for f32 {
    type Vector = ShimToken<Shim2<generic::f32x1, Self>, Self, Avx>;
}

impl Scalar<Avx, width::W4> for f32 {
    type Vector = ShimToken<f32x4, Self, Avx>;
}

impl Scalar<Avx, width::W8> for f32 {
    type Vector = f32x8;
}

impl Scalar<Avx, width::W1> for f64 {
    type Vector = ShimToken<generic::f64x1, Self, Avx>;
}

impl Scalar<Avx, width::W2> for f64 {
    type Vector = ShimToken<f64x2, Self, Avx>;
}

impl Scalar<Avx, width::W4> for f64 {
    type Vector = f64x4;
}

impl Scalar<Avx, width::W8> for f64 {
    type Vector = Shim2<f64x4, f64>;
}

arithmetic_ops! {
    feature: Sse::new_unchecked(),
    for f32x4:
        add -> (_mm_add_ps),
        sub -> (_mm_sub_ps),
        mul -> (_mm_mul_ps),
        div -> (_mm_div_ps)
}

arithmetic_ops! {
    feature: Sse::new_unchecked(),
    for f64x2:
        add -> (_mm_add_pd),
        sub -> (_mm_sub_pd),
        mul -> (_mm_mul_pd),
        div -> (_mm_div_pd)
}

arithmetic_ops! {
    feature: Avx::new_unchecked(),
    for f32x8:
        add -> (_mm256_add_ps),
        sub -> (_mm256_sub_ps),
        mul -> (_mm256_mul_ps),
        div -> (_mm256_div_ps)
}

arithmetic_ops! {
    feature: Avx::new_unchecked(),
    for f64x4:
        add -> (_mm256_add_pd),
        sub -> (_mm256_sub_pd),
        mul -> (_mm256_mul_pd),
        div -> (_mm256_div_pd)
}

impl core::ops::Neg for f32x4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for f64x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_pd(self.0, _mm_set1_pd(-0.)) })
    }
}

impl core::ops::Neg for f32x8 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_ps(self.0, _mm256_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for f64x4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.)) })
    }
}

as_slice! { f32x4 }
as_slice! { f32x8 }
as_slice! { f64x2 }
as_slice! { f64x4 }

unsafe impl Vector for f32x4 {
    type Scalar = f32;

    type Token = Sse;

    type Width = crate::vector::width::W4;

    type Underlying = __m128;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm_setzero_ps() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set1_ps(from) })
    }
}

unsafe impl Vector for f64x2 {
    type Scalar = f64;

    type Token = Sse;

    type Width = crate::vector::width::W2;

    type Underlying = __m128d;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm_setzero_pd() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set1_pd(from) })
    }
}

unsafe impl Vector for f32x8 {
    type Scalar = f32;

    type Token = Avx;

    type Width = crate::vector::width::W8;

    type Underlying = __m256;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm256_setzero_ps() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm256_set1_ps(from) })
    }
}

unsafe impl Vector for f64x4 {
    type Scalar = f64;

    type Token = Avx;

    type Width = crate::vector::width::W4;

    type Underlying = __m256d;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm256_setzero_pd() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm256_set1_pd(from) })
    }
}
