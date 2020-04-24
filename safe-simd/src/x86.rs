//! x86/x86-64 vector types.

/// SSE instruction set extension.
pub mod sse {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    use num_complex::Complex;

    /// SSE instruction set handle.
    #[derive(Clone, Copy, Debug)]
    pub struct Sse(());

    /// An SSE vector of `f32`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vf32(__m128);

    /// An SSE vector of `f64`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vf64(__m128d);

    /// An SSE vector of `Complex<f32>`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vcf32(__m128);

    /// An SSE vector of `Complex<f64>`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vcf64(__m128d);

    impl crate::vector::Feature for Sse {
        #[inline]
        fn new() -> Option<Self> {
            if is_x86_feature_detected!("sse3") {
                Some(Self(()))
            } else {
                None
            }
        }

        #[inline]
        unsafe fn new_unchecked() -> Self {
            Self(())
        }
    }

    impl crate::vector::Loader<f32> for Sse {
        type Vector = Vf32;
    }

    impl crate::vector::Loader<f64> for Sse {
        type Vector = Vf64;
    }

    impl crate::vector::Loader<Complex<f32>> for Sse {
        type Vector = Vcf32;
    }

    impl crate::vector::Loader<Complex<f64>> for Sse {
        type Vector = Vcf64;
    }

    arithmetic_ops! {
        for Vf32:
            add -> _mm_add_ps,
            sub -> _mm_sub_ps,
            mul -> _mm_mul_ps,
            div -> _mm_div_ps
    }

    arithmetic_ops! {
        for Vf64:
            add -> _mm_add_pd,
            sub -> _mm_sub_pd,
            mul -> _mm_mul_pd,
            div -> _mm_div_pd
    }

    arithmetic_ops! {
        for Vcf32:
            add -> _mm_add_ps,
            sub -> _mm_sub_ps,
            mul -> cmul_f32,
            div -> cdiv_f32
    }

    arithmetic_ops! {
        for Vcf64:
            add -> _mm_add_pd,
            sub -> _mm_sub_pd,
            mul -> cmul_f64,
            div -> cdiv_f64
    }

    #[target_feature(enable = "sse3")]
    unsafe fn cmul_f32(a: __m128, b: __m128) -> __m128 {
        let re = _mm_moveldup_ps(a);
        let im = _mm_movehdup_ps(a);
        let sh = _mm_shuffle_ps(b, b, 0xb1);
        _mm_addsub_ps(_mm_mul_ps(re, b), _mm_mul_ps(im, sh))
    }

    #[target_feature(enable = "sse3")]
    unsafe fn cmul_f64(a: __m128d, b: __m128d) -> __m128d {
        let re = _mm_shuffle_pd(a, a, 0x00);
        let im = _mm_shuffle_pd(a, a, 0x03);
        let sh = _mm_shuffle_pd(b, b, 0x01);
        _mm_addsub_pd(_mm_mul_pd(re, b), _mm_mul_pd(im, sh))
    }

    // [(a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im)] + i [(a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)]
    #[target_feature(enable = "sse3")]
    unsafe fn cdiv_f32(a: __m128, b: __m128) -> __m128 {
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
    unsafe fn cdiv_f64(a: __m128d, b: __m128d) -> __m128d {
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

    impl core::ops::Neg for Vf32 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.)) })
        }
    }

    impl core::ops::Neg for Vf64 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm_xor_pd(self.0, _mm_set1_pd(-0.)) })
        }
    }

    impl core::ops::Neg for Vcf32 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.)) })
        }
    }

    impl core::ops::Neg for Vcf64 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm_xor_pd(self.0, _mm_set1_pd(-0.)) })
        }
    }

    as_slice! { Vf32 }
    as_slice! { Vf64 }
    as_slice! { Vcf32 }
    as_slice! { Vcf64 }

    unsafe impl crate::vector::VectorCore for Vf32 {
        type Scalar = f32;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm_set1_ps(from))
        }
    }

    unsafe impl crate::vector::VectorCore for Vf64 {
        type Scalar = f64;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm_set1_pd(from))
        }
    }

    unsafe impl crate::vector::VectorCore for Vcf32 {
        type Scalar = Complex<f32>;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm_set_ps(from.im, from.re, from.im, from.re))
        }
    }

    unsafe impl crate::vector::VectorCore for Vcf64 {
        type Scalar = Complex<f64>;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm_set_pd(from.im, from.re))
        }
    }

    impl crate::vector::Complex<f32> for Vcf32 {
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

    impl crate::vector::Complex<f64> for Vcf64 {
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
}

/// AVX instruction set extension.
pub mod avx {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    use num_complex::Complex;

    /// AVX instruction set handle.
    #[derive(Clone, Copy, Debug)]
    pub struct Avx(());

    /// An AVX vector of `f32`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vf32(__m256);

    /// An AVX vector of `f64`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vf64(__m256d);

    /// An AVX vector of `Complex<f32>`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vcf32(__m256);

    /// An AVX vector of `Complex<f64>`s.
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vcf64(__m256d);

    impl crate::vector::Feature for Avx {
        #[inline]
        fn new() -> Option<Self> {
            if is_x86_feature_detected!("avx") {
                Some(Self(()))
            } else {
                None
            }
        }

        #[inline]
        unsafe fn new_unchecked() -> Self {
            Self(())
        }
    }

    impl crate::vector::Loader<f32> for Avx {
        type Vector = Vf32;
    }

    impl crate::vector::Loader<f64> for Avx {
        type Vector = Vf64;
    }

    impl crate::vector::Loader<Complex<f32>> for Avx {
        type Vector = Vcf32;
    }

    impl crate::vector::Loader<Complex<f64>> for Avx {
        type Vector = Vcf64;
    }

    arithmetic_ops! {
        for Vf32:
            add -> _mm256_add_ps,
            sub -> _mm256_sub_ps,
            mul -> _mm256_mul_ps,
            div -> _mm256_div_ps
    }

    arithmetic_ops! {
        for Vf64:
            add -> _mm256_add_pd,
            sub -> _mm256_sub_pd,
            mul -> _mm256_mul_pd,
            div -> _mm256_div_pd
    }

    arithmetic_ops! {
        for Vcf32:
            add -> _mm256_add_ps,
            sub -> _mm256_sub_ps,
            mul -> cmul_f32,
            div -> cdiv_f32
    }

    arithmetic_ops! {
        for Vcf64:
            add -> _mm256_add_pd,
            sub -> _mm256_sub_pd,
            mul -> cmul_f64,
            div -> cdiv_f64
    }

    #[target_feature(enable = "avx")]
    unsafe fn cmul_f32(a: __m256, b: __m256) -> __m256 {
        let re = _mm256_moveldup_ps(a);
        let im = _mm256_movehdup_ps(a);
        let sh = _mm256_shuffle_ps(b, b, 0xb1);
        _mm256_addsub_ps(_mm256_mul_ps(re, b), _mm256_mul_ps(im, sh))
    }

    #[target_feature(enable = "avx")]
    unsafe fn cmul_f64(a: __m256d, b: __m256d) -> __m256d {
        let re = _mm256_shuffle_pd(a, a, 0x00);
        let im = _mm256_shuffle_pd(a, a, 0x03);
        let sh = _mm256_shuffle_pd(b, b, 0x01);
        _mm256_addsub_pd(_mm256_mul_pd(re, b), _mm256_mul_pd(im, sh))
    }

    // [(a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im)] + i [(a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)]
    #[target_feature(enable = "avx")]
    unsafe fn cdiv_f32(a: __m256, b: __m256) -> __m256 {
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
    unsafe fn cdiv_f64(a: __m256d, b: __m256d) -> __m256d {
        let b_re = _mm256_shuffle_pd(b, b, 0x00);
        let b_im = _mm256_shuffle_pd(b, b, 0x03);
        let a_flip = _mm256_shuffle_pd(a, a, 0x01);
        let norm_sqr = _mm256_add_pd(_mm256_mul_pd(b_re, b_re), _mm256_mul_pd(b_im, b_im));
        _mm256_div_pd(
            _mm256_addsub_pd(
                _mm256_mul_pd(a, b_re),
                _mm256_xor_pd(_mm256_mul_pd(a_flip, b_im), _mm256_set1_pd(-0.)),
            ),
            norm_sqr,
        )
    }

    impl core::ops::Neg for Vf32 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm256_xor_ps(self.0, _mm256_set1_ps(-0.)) })
        }
    }

    impl core::ops::Neg for Vf64 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.)) })
        }
    }

    impl core::ops::Neg for Vcf32 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm256_xor_ps(self.0, _mm256_set1_ps(-0.)) })
        }
    }

    impl core::ops::Neg for Vcf64 {
        type Output = Self;
        fn neg(self) -> Self {
            Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.)) })
        }
    }

    as_slice! { Vf32 }
    as_slice! { Vf64 }
    as_slice! { Vcf32 }
    as_slice! { Vcf64 }

    unsafe impl crate::vector::VectorCore for Vf32 {
        type Scalar = f32;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm256_set1_ps(from))
        }
    }

    unsafe impl crate::vector::VectorCore for Vf64 {
        type Scalar = f64;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm256_set1_pd(from))
        }
    }

    unsafe impl crate::vector::VectorCore for Vcf32 {
        type Scalar = Complex<f32>;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm256_setr_ps(
                from.re, from.im, from.re, from.im, from.re, from.im, from.re, from.im,
            ))
        }
    }

    unsafe impl crate::vector::VectorCore for Vcf64 {
        type Scalar = Complex<f64>;

        #[inline]
        unsafe fn splat(from: Self::Scalar) -> Self {
            Self(_mm256_setr_pd(from.re, from.im, from.re, from.im))
        }
    }

    impl crate::vector::Complex<f32> for Vcf32 {
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

    impl crate::vector::Complex<f64> for Vcf64 {
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
}
