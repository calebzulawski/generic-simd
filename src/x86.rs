/// SSE instruction set extension.
pub mod sse {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    /// SSE instruction set handle.
    #[derive(Clone, Copy, Debug)]
    pub struct Sse(());

    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Vf32(__m128);

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

        #[inline]
        unsafe fn load_ptr(&self, from: *const f32) -> Vf32 {
            Vf32(_mm_load_ps(from))
        }

        #[inline]
        fn splat(&self, from: f32) -> Vf32 {
            Vf32(unsafe { _mm_set1_ps(from) })
        }

        #[inline]
        fn zero(&self) -> Vf32 {
            Vf32(unsafe { _mm_setzero_ps() })
        }
    }

    impl crate::vector::Capability<f64> for Sse {
        type Vector = Vf64;

        #[inline]
        unsafe fn load_ptr(&self, from: *const f64) -> Vf64 {
            Vf64(_mm_load_pd(from))
        }

        #[inline]
        fn splat(&self, from: f64) -> Vf64 {
            Vf64(unsafe { _mm_set1_pd(from) })
        }

        #[inline]
        fn zero(&self) -> Vf64 {
            Vf64(unsafe { _mm_setzero_pd() })
        }
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
        unsafe fn store_ptr(self, to: *mut f32) {
            _mm_storeu_ps(to, self.0);
        }
    }

    unsafe impl crate::vector::VectorCore for Vf64 {
        type Scalar = f64;

        #[inline]
        unsafe fn store_ptr(self, to: *mut f64) {
            _mm_storeu_pd(to, self.0);
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

    #[derive(Clone, Copy, Debug)]
    pub struct Vf32(__m256);

    #[derive(Clone, Copy, Debug)]
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

        #[inline]
        unsafe fn load_ptr(&self, from: *const f32) -> Vf32 {
            Vf32(_mm256_load_ps(from))
        }

        #[inline]
        fn splat(&self, from: f32) -> Vf32 {
            Vf32(unsafe { _mm256_set1_ps(from) })
        }

        #[inline]
        fn zero(&self) -> Vf32 {
            Vf32(unsafe { _mm256_setzero_ps() })
        }
    }

    impl crate::vector::Capability<f64> for Avx {
        type Vector = Vf64;

        #[inline]
        unsafe fn load_ptr(&self, from: *const f64) -> Vf64 {
            Vf64(_mm256_load_pd(from))
        }

        #[inline]
        fn splat(&self, from: f64) -> Vf64 {
            Vf64(unsafe { _mm256_set1_pd(from) })
        }

        #[inline]
        fn zero(&self) -> Vf64 {
            Vf64(unsafe { _mm256_setzero_pd() })
        }
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
        unsafe fn store_ptr(self, to: *mut f32) {
            _mm256_storeu_ps(to, self.0);
        }
    }

    unsafe impl crate::vector::VectorCore for Vf64 {
        type Scalar = f64;

        #[inline]
        unsafe fn store_ptr(self, to: *mut f64) {
            _mm256_storeu_pd(to, self.0);
        }
    }
}
