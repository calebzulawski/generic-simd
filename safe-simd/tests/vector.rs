use num_complex::Complex;
use num_traits::Num;
use rand::distributions::Standard;
use rand::prelude::*;
use safe_simd::vector::{FeatureDetect, Signed, Vector, Widest};

#[inline]
fn ops_impl<T, D, F>(distribution: D, feature: F)
where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    F: FeatureDetect + Widest<T>,
    F::Widest: Signed<T>,
{
    let mut a = F::Widest::zeroed(feature);
    let mut b = F::Widest::zeroed(feature);

    let mut rng = rand::thread_rng();
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    // Add
    {
        let c = a + b;
        for i in 0..F::Widest::WIDTH {
            assert_eq!(c[i], a[i] + b[i])
        }
    }

    // Sub
    {
        let c = a - b;
        for i in 0..F::Widest::WIDTH {
            assert_eq!(c[i], a[i] - b[i])
        }
    }

    // Mul
    {
        let c = a * b;
        for i in 0..F::Widest::WIDTH {
            assert_eq!(c[i], a[i] * b[i])
        }
    }

    // Div
    {
        let c = a / b;
        for i in 0..F::Widest::WIDTH {
            assert_eq!(c[i], a[i] / b[i])
        }
    }

    // Neg
    {
        let c = -a;
        for i in 0..F::Widest::WIDTH {
            assert_eq!(c[i], -a[i])
        }
    }
}

macro_rules! ops_test {
    {
        $name:ident, $handle:path, $handleinit:expr, $type:ty
    } => {
        #[test]
        fn $name() {
            if let Some(handle) = $handleinit {
                ops_impl::<$type, ops_test!(@distty $type), $handle>(ops_test!(@distinit $type), handle);
            }
        }
    };
    {
        @distty Complex<$type:ty>
    } => {
        ComplexDistribution<$type>
    };
    {
        @distty $type:ty
    } => {
        Standard
    };
    {
        @distinit Complex<$type:ty>
    } => {
        ComplexDistribution::new(Standard, Standard)
    };
    {
        @distinit $type:ty
    } => {
        Standard
    }
}

ops_test! { ops_generic_f32, safe_simd::generic::Generic, safe_simd::generic::Generic::detect(), f32 }
ops_test! { ops_generic_f64, safe_simd::generic::Generic, safe_simd::generic::Generic::detect(), f64 }
ops_test! { ops_generic_cf32, safe_simd::generic::Generic, safe_simd::generic::Generic::detect(), Complex<f32> }
ops_test! { ops_generic_cf64, safe_simd::generic::Generic, safe_simd::generic::Generic::detect(), Complex<f32> }
ops_test! { ops_sse_f32, safe_simd::x86::Sse, safe_simd::x86::Sse::detect(), f32 }
ops_test! { ops_sse_f64, safe_simd::x86::Sse, safe_simd::x86::Sse::detect(), f64 }
ops_test! { ops_sse_cf32, safe_simd::x86::Sse, safe_simd::x86::Sse::detect(), Complex<f32> }
ops_test! { ops_sse_cf64, safe_simd::x86::Sse, safe_simd::x86::Sse::detect(), Complex<f32> }
ops_test! { ops_avx_f32, safe_simd::x86::Avx, safe_simd::x86::Avx::detect(), f32 }
ops_test! { ops_avx_f64, safe_simd::x86::Avx, safe_simd::x86::Avx::detect(), f64 }
ops_test! { ops_avx_cf32, safe_simd::x86::Avx, safe_simd::x86::Avx::detect(), Complex<f32> }
ops_test! { ops_avx_cf64, safe_simd::x86::Avx, safe_simd::x86::Avx::detect(), Complex<f32> }
