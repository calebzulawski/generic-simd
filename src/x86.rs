//! x86/x86-64 vector types.

/// SSE instruction set extension.
pub mod sse {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

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

    impl crate::vector::Feature for Sse {
        fn new() -> Option<Self> {
            if is_x86_feature_detected!("sse") {
                Some(Self(()))
            } else {
                None
            }
        }

        unsafe fn new_unchecked() -> Self {
            Self(())
        }

        fn apply<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
            #[target_feature(enable = "sse")]
            unsafe fn apply<T, F: FnOnce(Sse) -> T>(handle: Sse, f: F) -> T {
                f(handle)
            }
            unsafe { apply(self, f) }
        }
    }

    impl crate::vector::Capability<f32> for Sse {
        type Vector = Vf32;
    }

    impl crate::vector::Capability<f64> for Sse {
        type Vector = Vf64;
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

    as_slice! { Vf32 }
    as_slice! { Vf64 }

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
}

/// AVX instruction set extension.
pub mod avx {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

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

    impl crate::vector::Feature for Avx {
        fn new() -> Option<Self> {
            if is_x86_feature_detected!("avx") {
                Some(Self(()))
            } else {
                None
            }
        }

        unsafe fn new_unchecked() -> Self {
            Self(())
        }

        fn apply<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
            #[target_feature(enable = "avx")]
            unsafe fn apply<T, F: FnOnce(Avx) -> T>(handle: Avx, f: F) -> T {
                f(handle)
            }
            unsafe { apply(self, f) }
        }
    }

    impl crate::vector::Capability<f32> for Avx {
        type Vector = Vf32;
    }

    impl crate::vector::Capability<f64> for Avx {
        type Vector = Vf64;
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

    as_slice! { Vf32 }
    as_slice! { Vf64 }

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
}
