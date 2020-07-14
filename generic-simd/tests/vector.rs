use generic_simd::{
    arch::Cpu,
    vector::{Handle, Signed},
};
use num_traits::Num;
use rand::distributions::Standard;
use rand::prelude::*;

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
    let mut rng = rand::thread_rng();
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
    let mut rng = rand::thread_rng();
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
    let mut rng = rand::thread_rng();
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
    let mut rng = rand::thread_rng();
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output = a.clone();
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
    let mut rng = rand::thread_rng();
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output = a.clone();
    vfunc(&mut output, b);
    for i in 0..V::width() {
        sfunc(&mut a[i], b);
        assert_eq!(output[i], a[i])
    }
}

macro_rules! ops_test {
    {
        $name:ident, $handle:path, $handleinit:expr
    } => {
        #[test]
        fn $name() {
            if let Some(handle) = $handleinit {
                ops_test!{ @impl binary_op_impl, handle, core::ops::Add::add }
                ops_test!{ @impl binary_op_impl, handle, core::ops::Sub::sub }
                ops_test!{ @impl binary_op_impl, handle, core::ops::Mul::mul }
                ops_test!{ @impl binary_op_impl, handle, core::ops::Div::div }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Add::add }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Sub::sub }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Mul::mul }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Div::div }
                ops_test!{ @impl assign_op_impl, handle, core::ops::AddAssign::add_assign }
                ops_test!{ @impl assign_op_impl, handle, core::ops::SubAssign::sub_assign }
                ops_test!{ @impl assign_op_impl, handle, core::ops::MulAssign::mul_assign }
                ops_test!{ @impl assign_op_impl, handle, core::ops::DivAssign::div_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::AddAssign::add_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::SubAssign::sub_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::MulAssign::mul_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::DivAssign::div_assign }
                ops_test!{ @impl unary_op_impl, handle, core::ops::Neg::neg }
            }
        }
    };
    {
        @impl $test:ident, $handle:ident, $func:path
    } => {
        ops_test!{@types $test, $handle, zeroed_native, $func}
        ops_test!{@types $test, $handle, zeroed1, $func}
        ops_test!{@types $test, $handle, zeroed2, $func}
        ops_test!{@types $test, $handle, zeroed4, $func}
        ops_test!{@types $test, $handle, zeroed8, $func}
    };
    {
        @types $test:ident, $handle:ident, $init:ident, $func:path
    } => {
        $test(Standard, ops_test!(@init $test, f32, $handle, $init), $func, $func);
        $test(Standard, ops_test!(@init $test, f64, $handle, $init), $func, $func);

        #[cfg(feature = "complex")]
        $test(ComplexDistribution::new(Standard, Standard), ops_test!(@init $test, Complex<f32>, $handle, $init), $func, $func);

        #[cfg(feature = "complex")]
        $test(ComplexDistribution::new(Standard, Standard), ops_test!(@init $test, Complex<f64>, $handle, $init), $func, $func);
    };
    {
        @init unary_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        Handle::<$type>::$init($handle)
    };
    {
        @init binary_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        (Handle::<$type>::$init($handle), Handle::<$type>::$init($handle))
    };
    {
        @init binary_scalar_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        Handle::<$type>::$init($handle)
    };
    {
        @init assign_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        (Handle::<$type>::$init($handle), Handle::<$type>::$init($handle))
    };
    {
        @init assign_scalar_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        Handle::<$type>::$init($handle)
    };
}

ops_test! { ops_generic, generic_simd::arch::generic::Generic, generic_simd::arch::generic::Generic::new() }

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
ops_test! { ops_sse, generic_simd::arch::x86::Sse, generic_simd::arch::x86::Sse::new() }

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
ops_test! { ops_avx, generic_simd::arch::x86::Avx, generic_simd::arch::x86::Avx::new() }
