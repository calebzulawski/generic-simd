#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    arch::{generic, x86::*, Token},
    scalar::Scalar,
    shim::{Shim2, Shim4, Shim8, ShimToken},
    vector::{width, Native, Vector},
};
use num_complex::Complex;

impl Native<Sse> for Complex<f32> {
    type Width = width::W2;
}

impl Native<Sse> for Complex<f64> {
    type Width = width::W1;
}

impl Native<Avx> for Complex<f32> {
    type Width = width::W4;
}

impl Native<Avx> for Complex<f64> {
    type Width = width::W2;
}

/// An SSE vector of `Complex<f32>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x2(__m128);

/// An SSE vector of `Complex<f64>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x1(__m128d);

/// An AVX vector of `Complex<f32>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x4(__m256);

/// An AVX vector of `Complex<f64>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x2(__m256d);

impl Scalar<Sse, width::W1> for Complex<f32> {
    type Vector = ShimToken<generic::cf32x1, Self, Sse>;
}

impl Scalar<Sse, width::W2> for Complex<f32> {
    type Vector = cf32x2;
}

impl Scalar<Sse, width::W4> for Complex<f32> {
    type Vector = Shim2<cf32x2, Complex<f32>>;
}

impl Scalar<Sse, width::W8> for Complex<f32> {
    type Vector = Shim4<cf32x2, Complex<f32>>;
}

impl Scalar<Sse, width::W1> for Complex<f64> {
    type Vector = cf64x1;
}

impl Scalar<Sse, width::W2> for Complex<f64> {
    type Vector = Shim2<cf64x1, Complex<f64>>;
}

impl Scalar<Sse, width::W4> for Complex<f64> {
    type Vector = Shim4<cf64x1, Complex<f64>>;
}

impl Scalar<Sse, width::W8> for Complex<f64> {
    type Vector = Shim8<cf64x1, Self>;
}

impl Scalar<Avx, width::W1> for Complex<f32> {
    type Vector = ShimToken<generic::cf32x1, Self, Avx>;
}

impl Scalar<Avx, width::W2> for Complex<f32> {
    type Vector = ShimToken<cf32x2, Self, Avx>;
}

impl Scalar<Avx, width::W4> for Complex<f32> {
    type Vector = cf32x4;
}

impl Scalar<Avx, width::W8> for Complex<f32> {
    type Vector = Shim2<cf32x4, Complex<f32>>;
}

impl Scalar<Avx, width::W1> for Complex<f64> {
    type Vector = ShimToken<cf64x1, Self, Avx>;
}

impl Scalar<Avx, width::W2> for Complex<f64> {
    type Vector = cf64x2;
}

impl Scalar<Avx, width::W4> for Complex<f64> {
    type Vector = Shim2<cf64x2, Complex<f64>>;
}

impl Scalar<Avx, width::W8> for Complex<f64> {
    type Vector = Shim4<cf64x2, Complex<f64>>;
}

arithmetic_ops! {
    feature: Sse::new_unchecked(),
    for cf32x2:
        add -> _mm_add_ps,
        sub -> _mm_sub_ps,
        mul -> mul_cf32x2,
        div -> div_cf32x2
}

arithmetic_ops! {
    feature: Sse::new_unchecked(),
    for cf64x1:
        add -> _mm_add_pd,
        sub -> _mm_sub_pd,
        mul -> mul_cf64x1,
        div -> div_cf64x1
}

arithmetic_ops! {
    feature: Avx::new_unchecked(),
    for cf32x4:
        add -> _mm256_add_ps,
        sub -> _mm256_sub_ps,
        mul -> mul_cf32x4,
        div -> div_cf32x4
}

arithmetic_ops! {
    feature: Avx::new_unchecked(),
    for cf64x2:
        add -> _mm256_add_pd,
        sub -> _mm256_sub_pd,
        mul -> mul_cf64x2,
        div -> div_cf64x2
}

#[target_feature(enable = "sse3")]
#[inline]
unsafe fn mul_cf32x2(a: __m128, b: __m128) -> __m128 {
    let re = _mm_moveldup_ps(a);
    let im = _mm_movehdup_ps(a);
    let sh = _mm_shuffle_ps(b, b, 0xb1);
    _mm_addsub_ps(_mm_mul_ps(re, b), _mm_mul_ps(im, sh))
}

#[target_feature(enable = "sse3")]
#[inline]
unsafe fn mul_cf64x1(a: __m128d, b: __m128d) -> __m128d {
    let re = _mm_shuffle_pd(a, a, 0x00);
    let im = _mm_shuffle_pd(a, a, 0x03);
    let sh = _mm_shuffle_pd(b, b, 0x01);
    _mm_addsub_pd(_mm_mul_pd(re, b), _mm_mul_pd(im, sh))
}

// [(a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im)] + i [(a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)]
#[target_feature(enable = "sse3")]
#[inline]
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
#[inline]
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
#[inline]
unsafe fn mul_cf32x4(a: __m256, b: __m256) -> __m256 {
    let re = _mm256_moveldup_ps(a);
    let im = _mm256_movehdup_ps(a);
    let sh = _mm256_shuffle_ps(b, b, 0xb1);
    _mm256_addsub_ps(_mm256_mul_ps(re, b), _mm256_mul_ps(im, sh))
}

#[target_feature(enable = "avx")]
#[inline]
unsafe fn mul_cf64x2(a: __m256d, b: __m256d) -> __m256d {
    let re = _mm256_unpacklo_pd(a, a);
    let im = _mm256_unpackhi_pd(a, a);
    let sh = _mm256_shuffle_pd(b, b, 0x5);
    _mm256_addsub_pd(_mm256_mul_pd(re, b), _mm256_mul_pd(im, sh))
}

// [(a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im)] + i [(a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)]
#[target_feature(enable = "avx")]
#[inline]
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
#[inline]
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

impl core::ops::Neg for cf32x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for cf64x1 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_pd(self.0, _mm_set1_pd(-0.)) })
    }
}

impl core::ops::Neg for cf32x4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_ps(self.0, _mm256_set1_ps(-0.)) })
    }
}

impl core::ops::Neg for cf64x2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.)) })
    }
}

as_slice! { cf32x2 }
as_slice! { cf32x4 }
as_slice! { cf64x1 }
as_slice! { cf64x2 }

unsafe impl Vector for cf32x2 {
    type Scalar = Complex<f32>;

    type Token = Sse;

    type Width = crate::vector::width::W2;

    type Underlying = __m128;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm_setzero_ps() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set_ps(from.im, from.re, from.im, from.re) })
    }
}

unsafe impl Vector for cf64x1 {
    type Scalar = Complex<f64>;

    type Token = Sse;

    type Width = crate::vector::width::W1;

    type Underlying = __m128d;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm_setzero_pd() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm_set_pd(from.im, from.re) })
    }
}

unsafe impl Vector for cf32x4 {
    type Scalar = Complex<f32>;

    type Token = Avx;

    type Width = crate::vector::width::W4;

    type Underlying = __m256;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm256_setzero_ps() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        unsafe {
            Self(_mm256_setr_ps(
                from.re, from.im, from.re, from.im, from.re, from.im, from.re, from.im,
            ))
        }
    }
}

unsafe impl Vector for cf64x2 {
    type Scalar = Complex<f64>;

    type Token = Avx;

    type Width = crate::vector::width::W2;

    type Underlying = __m256d;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        Self(unsafe { _mm256_setzero_pd() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        Self(unsafe { _mm256_setr_pd(from.re, from.im, from.re, from.im) })
    }
}

impl crate::vector::Complex for cf32x2 {
    type RealScalar = f32;

    #[inline]
    fn conj(self) -> Self {
        Self(unsafe { _mm_xor_ps(self.0, _mm_set_ps(-0., 0., -0., 0.)) })
    }

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
    fn conj(self) -> Self {
        Self(unsafe { _mm_xor_pd(self.0, _mm_set_pd(-0., 0.)) })
    }

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
    fn conj(self) -> Self {
        Self(unsafe { _mm256_xor_ps(self.0, _mm256_set_ps(-0., 0., -0., 0., -0., 0., -0., 0.)) })
    }

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
    fn conj(self) -> Self {
        Self(unsafe { _mm256_xor_pd(self.0, _mm256_set_pd(-0., 0., -0., 0.)) })
    }

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
