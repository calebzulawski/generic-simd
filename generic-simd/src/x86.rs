//! x86/x86-64 vector types.

use crate::shim::{Shim2, Shim4, Shim8};
use crate::vector::{Handle, Vector};
use arch_types::{marker::Superset, Features};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use num_complex::Complex;

arch_types::new_features_type! { #[doc = "SSE instruction set handle."] pub Sse => "sse", "sse3" }
arch_types::new_features_type! { #[doc = "AVX instruction set handle."] pub Avx => "sse", "sse3", "avx" }

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

/// An SSE vector of `Complex<f32>`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x2(__m128);

/// An SSE vector of `Complex<f64>`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x1(__m128d);

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

/// An AVX vector of `Complex<f32>`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x4(__m256);

/// An AVX vector of `Complex<f64>`s.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x2(__m256d);

impl Handle<f32> for Sse {
    type VectorNative = f32x4;
    type Feature1 = crate::generic::Generic;
    type Feature2 = crate::generic::Generic;
    type Feature4 = Sse;
    type Feature8 = Sse;
    type Vector1 = crate::generic::f32x1;
    type Vector2 = Shim2<crate::generic::f32x1, f32>;
    type Vector4 = f32x4;
    type Vector8 = Shim2<f32x4, f32>;
}

impl Handle<f64> for Sse {
    type VectorNative = f64x2;
    type Feature1 = crate::generic::Generic;
    type Feature2 = Sse;
    type Feature4 = Sse;
    type Feature8 = Sse;
    type Vector1 = crate::generic::f64x1;
    type Vector2 = f64x2;
    type Vector4 = Shim2<f64x2, f64>;
    type Vector8 = Shim4<f64x2, f64>;
}

impl Handle<Complex<f32>> for Sse {
    type VectorNative = cf32x2;
    type Feature1 = crate::generic::Generic;
    type Feature2 = Sse;
    type Feature4 = Sse;
    type Feature8 = Sse;
    type Vector1 = crate::generic::cf32x1;
    type Vector2 = cf32x2;
    type Vector4 = Shim2<cf32x2, Complex<f32>>;
    type Vector8 = Shim4<cf32x2, Complex<f32>>;
}

impl Handle<Complex<f64>> for Sse {
    type VectorNative = cf64x1;
    type Feature1 = Sse;
    type Feature2 = Sse;
    type Feature4 = Sse;
    type Feature8 = Sse;
    type Vector1 = cf64x1;
    type Vector2 = Shim2<cf64x1, Complex<f64>>;
    type Vector4 = Shim4<cf64x1, Complex<f64>>;
    type Vector8 = Shim8<cf64x1, Complex<f64>>;
}

impl Handle<f32> for Avx {
    type VectorNative = f32x8;
    type Feature1 = crate::generic::Generic;
    type Feature2 = crate::generic::Generic;
    type Feature4 = Sse;
    type Feature8 = Avx;
    type Vector1 = crate::generic::f32x1;
    type Vector2 = Shim2<crate::generic::f32x1, f32>;
    type Vector4 = f32x4;
    type Vector8 = f32x8;
}

impl Handle<f64> for Avx {
    type VectorNative = f64x4;
    type Feature1 = crate::generic::Generic;
    type Feature2 = Sse;
    type Feature4 = Avx;
    type Feature8 = Avx;
    type Vector1 = crate::generic::f64x1;
    type Vector2 = f64x2;
    type Vector4 = f64x4;
    type Vector8 = Shim2<f64x4, f64>;
}

impl Handle<Complex<f32>> for Avx {
    type VectorNative = cf32x4;
    type Feature1 = crate::generic::Generic;
    type Feature2 = Sse;
    type Feature4 = Avx;
    type Feature8 = Avx;
    type Vector1 = crate::generic::cf32x1;
    type Vector2 = cf32x2;
    type Vector4 = cf32x4;
    type Vector8 = Shim2<cf32x4, Complex<f32>>;
}

impl Handle<Complex<f64>> for Avx {
    type VectorNative = cf64x2;
    type Feature1 = Sse;
    type Feature2 = Avx;
    type Feature4 = Avx;
    type Feature8 = Avx;
    type Vector1 = cf64x1;
    type Vector2 = cf64x2;
    type Vector4 = Shim2<cf64x2, Complex<f64>>;
    type Vector8 = Shim4<cf64x2, Complex<f64>>;
}

arithmetic_ops! {
    feature: crate::x86::Sse::new_unchecked(),
    for f32x4:
        add -> _mm_add_ps,
        sub -> _mm_sub_ps,
        mul -> _mm_mul_ps,
        div -> _mm_div_ps
}

arithmetic_ops! {
    feature: crate::x86::Sse::new_unchecked(),
    for f64x2:
        add -> _mm_add_pd,
        sub -> _mm_sub_pd,
        mul -> _mm_mul_pd,
        div -> _mm_div_pd
}

arithmetic_ops! {
    feature: crate::x86::Sse::new_unchecked(),
    for cf32x2:
        add -> _mm_add_ps,
        sub -> _mm_sub_ps,
        mul -> mul_cf32x2,
        div -> div_cf32x2
}

arithmetic_ops! {
    feature: crate::x86::Sse::new_unchecked(),
    for cf64x1:
        add -> _mm_add_pd,
        sub -> _mm_sub_pd,
        mul -> mul_cf64x1,
        div -> div_cf64x1
}

arithmetic_ops! {
    feature: crate::x86::Avx::new_unchecked(),
    for f32x8:
        add -> _mm256_add_ps,
        sub -> _mm256_sub_ps,
        mul -> _mm256_mul_ps,
        div -> _mm256_div_ps
}

arithmetic_ops! {
    feature: crate::x86::Avx::new_unchecked(),
    for f64x4:
        add -> _mm256_add_pd,
        sub -> _mm256_sub_pd,
        mul -> _mm256_mul_pd,
        div -> _mm256_div_pd
}

arithmetic_ops! {
    feature: crate::x86::Avx::new_unchecked(),
    for cf32x4:
        add -> _mm256_add_ps,
        sub -> _mm256_sub_ps,
        mul -> mul_cf32x4,
        div -> div_cf32x4
}

arithmetic_ops! {
    feature: crate::x86::Avx::new_unchecked(),
    for cf64x2:
        add -> _mm256_add_pd,
        sub -> _mm256_sub_pd,
        mul -> mul_cf64x2,
        div -> div_cf64x2
}

#[target_feature(enable = "sse3")]
unsafe fn mul_cf32x2(a: __m128, b: __m128) -> __m128 {
    let re = _mm_moveldup_ps(a);
    let im = _mm_movehdup_ps(a);
    let sh = _mm_shuffle_ps(b, b, 0xb1);
    _mm_addsub_ps(_mm_mul_ps(re, b), _mm_mul_ps(im, sh))
}

#[target_feature(enable = "sse3")]
unsafe fn mul_cf64x1(a: __m128d, b: __m128d) -> __m128d {
    let re = _mm_shuffle_pd(a, a, 0x00);
    let im = _mm_shuffle_pd(a, a, 0x03);
    let sh = _mm_shuffle_pd(b, b, 0x01);
    _mm_addsub_pd(_mm_mul_pd(re, b), _mm_mul_pd(im, sh))
}

// [(a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im)] + i [(a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)]
#[target_feature(enable = "sse3")]
unsafe fn div_cf32x2(a: __m128, b: __m128) -> __m128 {
    let b_re = _mm_moveldup_ps(b);
    let b_im = _mm_movehdup_ps(b);
    let a_flip = _mm_shuffle_ps(a, a, 0xb1);
    let norm_sqr = _mm_add_ps(_mm_mul_ps(b_re, b_re), _mm_mul_ps(b_im, b_im));
    _mm_div_ps(
        _mm_addsub_ps(
            _mm_mul_ps(a, b_re),
            _mm_xor_ps(_mm_mul_ps(a_flip, b_im), _mm_set1_ps(-0.)),
        ),
        norm_sqr,
    )
}

#[target_feature(enable = "sse3")]
unsafe fn div_cf64x1(a: __m128d, b: __m128d) -> __m128d {
    let b_re = _mm_shuffle_pd(b, b, 0x00);
    let b_im = _mm_shuffle_pd(b, b, 0x03);
    let a_flip = _mm_shuffle_pd(a, a, 0x01);
    let norm_sqr = _mm_add_pd(_mm_mul_pd(b_re, b_re), _mm_mul_pd(b_im, b_im));
    _mm_div_pd(
        _mm_addsub_pd(
            _mm_mul_pd(a, b_re),
            _mm_xor_pd(_mm_mul_pd(a_flip, b_im), _mm_set1_pd(-0.)),
        ),
        norm_sqr,
    )
}

#[target_feature(enable = "avx")]
unsafe fn mul_cf32x4(a: __m256, b: __m256) -> __m256 {
    let re = _mm256_moveldup_ps(a);
    let im = _mm256_movehdup_ps(a);
    let sh = _mm256_shuffle_ps(b, b, 0xb1);
    _mm256_addsub_ps(_mm256_mul_ps(re, b), _mm256_mul_ps(im, sh))
}

#[target_feature(enable = "avx")]
unsafe fn mul_cf64x2(a: __m256d, b: __m256d) -> __m256d {
    let re = _mm256_unpacklo_pd(a, a);
    let im = _mm256_unpackhi_pd(a, a);
    let sh = _mm256_shuffle_pd(b, b, 0x5);
    _mm256_addsub_pd(_mm256_mul_pd(re, b), _mm256_mul_pd(im, sh))
}

// [(a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im)] + i [(a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)]
#[target_feature(enable = "avx")]
unsafe fn div_cf32x4(a: __m256, b: __m256) -> __m256 {
    let b_re = _mm256_moveldup_ps(b);
    let b_im = _mm256_movehdup_ps(b);
    let a_flip = _mm256_shuffle_ps(a, a, 0xb1);
    let norm_sqr = _mm256_add_ps(_mm256_mul_ps(b_re, b_re), _mm256_mul_ps(b_im, b_im));
    _mm256_div_ps(
        _mm256_addsub_ps(
            _mm256_mul_ps(a, b_re),
            _mm256_xor_ps(_mm256_mul_ps(a_flip, b_im), _mm256_set1_ps(-0.)),
        ),
        norm_sqr,
    )
}

#[target_feature(enable = "avx")]
unsafe fn div_cf64x2(a: __m256d, b: __m256d) -> __m256d {
    let b_re = _mm256_unpacklo_pd(b, b);
    let b_im = _mm256_unpackhi_pd(b, b);
    let a_flip = _mm256_shuffle_pd(a, a, 0x5);
    let norm_sqr = _mm256_add_pd(_mm256_mul_pd(b_re, b_re), _mm256_mul_pd(b_im, b_im));
    _mm256_div_pd(
        _mm256_addsub_pd(
            _mm256_mul_pd(a, b_re),
            _mm256_xor_pd(_mm256_mul_pd(a_flip, b_im), _mm256_set1_pd(-0.)),
        ),
        norm_sqr,
    )
}

impl core::ops::Neg for f32x4 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for f64x2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_pd(self.0, _mm_set1_pd(-0.)) })
    }
}

impl core::ops::Neg for cf32x2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for cf64x1 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_pd(self.0, _mm_set1_pd(-0.)) })
    }
}

impl core::ops::Neg for f32x8 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_ps(self.0, _mm256_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for f64x4 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.)) })
    }
}

impl core::ops::Neg for cf32x4 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_ps(self.0, _mm256_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for cf64x2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.)) })
    }
}

as_slice! { f32x4 }
as_slice! { f32x8 }
as_slice! { f64x2 }
as_slice! { f64x4 }
as_slice! { cf32x2 }
as_slice! { cf32x4 }
as_slice! { cf64x1 }
as_slice! { cf64x2 }

unsafe impl Vector for f32x4 {
    type Scalar = f32;

    type Feature = Sse;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set1_ps(from) })
    }
}

unsafe impl Vector for f64x2 {
    type Scalar = f64;

    type Feature = Sse;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set1_pd(from) })
    }
}

unsafe impl Vector for cf32x2 {
    type Scalar = Complex<f32>;

    type Feature = Sse;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set_ps(from.im, from.re, from.im, from.re) })
    }
}

unsafe impl Vector for cf64x1 {
    type Scalar = Complex<f64>;

    type Feature = Sse;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set_pd(from.im, from.re) })
    }
}

unsafe impl Vector for f32x8 {
    type Scalar = f32;

    type Feature = Avx;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm256_set1_ps(from) })
    }
}

unsafe impl Vector for f64x4 {
    type Scalar = f64;

    type Feature = Avx;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm256_set1_pd(from) })
    }
}

unsafe impl Vector for cf32x4 {
    type Scalar = Complex<f32>;

    type Feature = Avx;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        unsafe {
            Self(_mm256_setr_ps(
                from.re, from.im, from.re, from.im, from.re, from.im, from.re, from.im,
            ))
        }
    }
}

unsafe impl Vector for cf64x2 {
    type Scalar = Complex<f64>;

    type Feature = Avx;

    #[inline]
    fn splat(_: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self(unsafe { _mm256_setr_pd(from.re, from.im, from.re, from.im) })
    }
}

impl crate::vector::Complex for cf32x2 {
    type RealScalar = f32;

    #[inline]
    fn mul_i(self) -> Self {
        Self(unsafe { _mm_addsub_ps(_mm_setzero_ps(), _mm_shuffle_ps(self.0, self.0, 0xb1)) })
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        unsafe {
            let neg = _mm_addsub_ps(_mm_setzero_ps(), self.0);
            Self(_mm_shuffle_ps(neg, neg, 0xb1))
        }
    }
}

impl crate::vector::Complex for cf64x1 {
    type RealScalar = f64;

    #[inline]
    fn mul_i(self) -> Self {
        Self(unsafe { _mm_addsub_pd(_mm_setzero_pd(), _mm_shuffle_pd(self.0, self.0, 0x1)) })
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        unsafe {
            let neg = _mm_addsub_pd(_mm_setzero_pd(), self.0);
            Self(_mm_shuffle_pd(neg, neg, 0x1))
        }
    }
}

impl crate::vector::Complex for cf32x4 {
    type RealScalar = f32;

    #[inline]
    fn mul_i(self) -> Self {
        Self(unsafe {
            _mm256_addsub_ps(_mm256_setzero_ps(), _mm256_shuffle_ps(self.0, self.0, 0xb1))
        })
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        unsafe {
            let neg = _mm256_addsub_ps(_mm256_setzero_ps(), self.0);
            Self(_mm256_shuffle_ps(neg, neg, 0xb1))
        }
    }
}

impl crate::vector::Complex for cf64x2 {
    type RealScalar = f64;

    #[inline]
    fn mul_i(self) -> Self {
        Self(unsafe {
            _mm256_addsub_pd(_mm256_setzero_pd(), _mm256_shuffle_pd(self.0, self.0, 0x5))
        })
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        unsafe {
            let neg = _mm256_addsub_pd(_mm256_setzero_pd(), self.0);
            Self(_mm256_shuffle_pd(neg, neg, 0x5))
        }
    }
}
