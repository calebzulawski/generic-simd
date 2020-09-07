#![cfg_attr(
    all(feature = "nightly", target_arch = "wasm32"),
    feature(wasm_simd, wasm_target_feature)
)]
#![cfg_attr(
    all(feature = "nightly", target_arch = "aarch64"),
    feature(stdsimd, aarch64_target_feature)
)]
#![cfg_attr(
    all(feature = "nightly", target_arch = "arm"),
    feature(stdsimd, arm_target_feature)
)]

use generic_simd::{dispatch, scalar::ScalarExt, vector::Signed};
use num_traits::Num;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::SeedableRng;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(feature = "complex")]
use num_complex::{Complex, ComplexDistribution};

#[inline]
fn unary_op_impl<D, V, VFunc, SFunc>(distribution: D, mut vector: V, vfunc: VFunc, sfunc: SFunc)
where
    V::Scalar: Num + core::ops::Neg<Output = V::Scalar> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<V::Scalar> + Copy,
    V: Signed,
    VFunc: Fn(V) -> V,
    SFunc: Fn(V::Scalar) -> V::Scalar,
{
    let mut rng = rand_pcg::Pcg32::seed_from_u64(999);
    for x in vector.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(vector);
    for i in 0..V::width() {
        assert_eq!(output[i], sfunc(vector[i]))
    }
}

#[inline]
fn binary_op_impl<D, V, VFunc, SFunc>(
    distribution: D,
    (mut a, mut b): (V, V),
    vfunc: VFunc,
    sfunc: SFunc,
) where
    V::Scalar: Num + core::ops::Neg<Output = V::Scalar> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<V::Scalar> + Copy,
    V: Signed,
    VFunc: Fn(V, V) -> V,
    SFunc: Fn(V::Scalar, V::Scalar) -> V::Scalar,
{
    let mut rng = rand_pcg::Pcg32::seed_from_u64(999);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(a, b);
    for i in 0..V::width() {
        assert_eq!(output[i], sfunc(a[i], b[i]))
    }
}

#[inline]
fn binary_scalar_op_impl<D, V, VFunc, SFunc>(distribution: D, mut a: V, vfunc: VFunc, sfunc: SFunc)
where
    V::Scalar: Num + core::ops::Neg<Output = V::Scalar> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<V::Scalar> + Copy,
    V: Signed,
    VFunc: Fn(V, V::Scalar) -> V,
    SFunc: Fn(V::Scalar, V::Scalar) -> V::Scalar,
{
    let mut rng = rand_pcg::Pcg32::seed_from_u64(999);
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(a, b);
    for i in 0..V::width() {
        assert_eq!(output[i], sfunc(a[i], b))
    }
}

#[inline]
fn assign_op_impl<D, V, VFunc, SFunc>(
    distribution: D,
    (mut a, mut b): (V, V),
    vfunc: VFunc,
    sfunc: SFunc,
) where
    V::Scalar: Num + core::ops::Neg<Output = V::Scalar> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<V::Scalar> + Copy,
    V: Signed,
    VFunc: Fn(&mut V, V),
    SFunc: Fn(&mut V::Scalar, V::Scalar),
{
    let mut rng = rand_pcg::Pcg32::seed_from_u64(999);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output: V = a;
    vfunc(&mut output, b);
    for i in 0..V::width() {
        sfunc(&mut a[i], b[i]);
        assert_eq!(output[i], a[i])
    }
}

#[inline]
fn assign_scalar_op_impl<D, V, VFunc, SFunc>(distribution: D, mut a: V, vfunc: VFunc, sfunc: SFunc)
where
    V::Scalar: Num + core::ops::Neg<Output = V::Scalar> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<V::Scalar> + Copy,
    V: Signed,
    VFunc: Fn(&mut V, V::Scalar),
    SFunc: Fn(&mut V::Scalar, V::Scalar),
{
    let mut rng = rand_pcg::Pcg32::seed_from_u64(999);
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output: V = a;
    vfunc(&mut output, b);
    for i in 0..V::width() {
        sfunc(&mut a[i], b);
        assert_eq!(output[i], a[i])
    }
}

macro_rules! ops_test {
    {
        $token:ident, $type:ty
    } => {
        pub mod width_native {
            use super::*;
            ops_test! { @wrapper $token, $type, zeroed_native }
        }
        pub mod width_1 {
            use super::*;
            ops_test! { @wrapper $token, $type, zeroed1 }
        }
        pub mod width_2 {
            use super::*;
            ops_test! { @wrapper $token, $type, zeroed2 }
        }
        pub mod width_4 {
            use super::*;
            ops_test! { @wrapper $token, $type, zeroed4 }
        }
        pub mod width_8 {
            use super::*;
            ops_test! { @wrapper $token, $type, zeroed8 }
        }
    };
    {
        @wrapper $token:ident, $type:ty, $init:ident
    } => {
        ops_test! { @impl $type, $init, add,               binary_op_impl,        $token, core::ops::Add::add }
        ops_test! { @impl $type, $init, sub,               binary_op_impl,        $token, core::ops::Sub::sub }
        ops_test! { @impl $type, $init, mul,               binary_op_impl,        $token, core::ops::Mul::mul }
        ops_test! { @impl $type, $init, div,               binary_op_impl,        $token, core::ops::Div::div }
        ops_test! { @impl $type, $init, add_scalar,        binary_scalar_op_impl, $token, core::ops::Add::add }
        ops_test! { @impl $type, $init, sub_scalar,        binary_scalar_op_impl, $token, core::ops::Sub::sub }
        ops_test! { @impl $type, $init, mul_scalar,        binary_scalar_op_impl, $token, core::ops::Mul::mul }
        ops_test! { @impl $type, $init, div_scalar,        binary_scalar_op_impl, $token, core::ops::Div::div }
        ops_test! { @impl $type, $init, add_assign,        assign_op_impl,        $token, core::ops::AddAssign::add_assign }
        ops_test! { @impl $type, $init, sub_assign,        assign_op_impl,        $token, core::ops::SubAssign::sub_assign }
        ops_test! { @impl $type, $init, mul_assign,        assign_op_impl,        $token, core::ops::MulAssign::mul_assign }
        ops_test! { @impl $type, $init, div_assign,        assign_op_impl,        $token, core::ops::DivAssign::div_assign }
        ops_test! { @impl $type, $init, add_assign_scalar, assign_scalar_op_impl, $token, core::ops::AddAssign::add_assign }
        ops_test! { @impl $type, $init, sub_assign_scalar, assign_scalar_op_impl, $token, core::ops::SubAssign::sub_assign }
        ops_test! { @impl $type, $init, mul_assign_scalar, assign_scalar_op_impl, $token, core::ops::MulAssign::mul_assign }
        ops_test! { @impl $type, $init, div_assign_scalar, assign_scalar_op_impl, $token, core::ops::DivAssign::div_assign }
        ops_test! { @impl $type, $init, neg,               unary_op_impl,         $token, core::ops::Neg::neg }
    };
    { @distribution f32 } => { Standard };
    { @distribution f64 } => { Standard };
    { @distribution Complex<f32> } => { ComplexDistribution::new(Standard, Standard) };
    { @distribution Complex<f64> } => { ComplexDistribution::new(Standard, Standard) };
    {
        @init unary_op_impl, $type:ty, $token:ident, $init:ident
    } => {
        <$type>::$init($token)
    };
    {
        @init binary_op_impl, $type:ty, $token:ident, $init:ident
    } => {
        (<$type>::$init($token), <$type>::$init($token))
    };
    {
        @init binary_scalar_op_impl, $type:ty, $token:ident, $init:ident
    } => {
        <$type>::$init($token)
    };
    {
        @init assign_op_impl, $type:ty, $token:ident, $init:ident
    } => {
        (<$type>::$init($token), <$type>::$init($token))
    };
    {
        @init assign_scalar_op_impl, $type:ty, $token:ident, $init:ident
    } => {
        <$type>::$init($token)
    };
    {
        @impl $type:ty, $init:ident, $name:ident, $test:ident, $token:ident, $func:path
    } => {
        paste::paste! {
            #[dispatch($token)]
            pub fn [<$name _dispatch>]() {
                $test(ops_test!(@distribution $type), ops_test!(@init $test, $type, $token, $init), $func, $func);
            }

            #[test]
            #[wasm_bindgen_test::wasm_bindgen_test]
            pub fn [<$name _generic>]() {
                [<$name _dispatch_default_version>]()
            }

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[test]
            pub fn [<$name _sse>]() {
                use generic_simd::arch::Token as _;
                if generic_simd::arch::x86::Sse::new().is_some() {
                    unsafe { [<$name _dispatch_sse41_version>]() }
                }
            }

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[test]
            pub fn [<$name _avx>]() {
                use generic_simd::arch::Token as _;
                if generic_simd::arch::x86::Avx::new().is_some() {
                    unsafe { [<$name _dispatch_avx_version>]() }
                }
            }

            #[cfg(all(feature = "nightly", target_arch = "aarch64"))]
            #[test]
            pub fn [<$name _neon>]() {
                use generic_simd::arch::Token as _;
                if generic_simd::arch::arm::Neon::new().is_some() {
                    unsafe { [<$name _dispatch_neon_version>]() }
                }
            }

            #[cfg(all(feature = "nightly", target_arch = "wasm32", target_feature = "simd128"))]
            #[wasm_bindgen_test::wasm_bindgen_test]
            pub fn [<$name _simd128>]() {
                use generic_simd::arch::Token as _;
                assert!(generic_simd::arch::wasm::Simd128::new().is_some());
                unsafe { [<$name _dispatch_simd128_version>]() }
            }
        }
    };
}

pub mod r#f32 {
    use super::*;
    ops_test! { token, f32 }
}

pub mod r#f64 {
    use super::*;
    ops_test! { token, f64 }
}

#[cfg(feature = "complex")]
pub mod complex_f32 {
    use super::*;
    ops_test! { token, Complex<f32> }
}

#[cfg(feature = "complex")]
pub mod complex_f64 {
    use super::*;
    ops_test! { token, Complex<f64> }
}
