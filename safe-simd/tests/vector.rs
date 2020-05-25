use arch_types::Features;
use num_complex::{Complex, ComplexDistribution};
use num_traits::Num;
use rand::distributions::Standard;
use rand::prelude::*;
use safe_simd::vector::{Native, Signed, Vector};

#[inline]
fn unary_op_impl<T, D, F, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    feature: F,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    F: Features + Native<T>,
    F::Vector: Signed<T>,
    VFunc: Fn(F::Vector) -> F::Vector,
    SFunc: Fn(T) -> T,
{
    let mut input = F::Vector::zeroed(feature);
    let mut rng = rand::thread_rng();
    for x in input.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(input);
    for i in 0..F::Vector::WIDTH {
        assert_eq!(output[i], sfunc(input[i]))
    }
}

#[inline]
fn binary_op_impl<T, D, F, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    feature: F,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    F: Features + Native<T>,
    F::Vector: Signed<T>,
    VFunc: Fn(F::Vector, F::Vector) -> F::Vector,
    SFunc: Fn(T, T) -> T,
{
    let mut a = F::Vector::zeroed(feature);
    let mut b = F::Vector::zeroed(feature);

    let mut rng = rand::thread_rng();
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(a, b);
    for i in 0..F::Vector::WIDTH {
        assert_eq!(output[i], sfunc(a[i], b[i]))
    }
}

#[inline]
fn binary_scalar_op_impl<T, D, F, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    feature: F,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    F: Features + Native<T>,
    F::Vector: Signed<T>,
    VFunc: Fn(F::Vector, T) -> F::Vector,
    SFunc: Fn(T, T) -> T,
{
    let mut rng = rand::thread_rng();
    let mut a = F::Vector::zeroed(feature);
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(a, b);
    for i in 0..F::Vector::WIDTH {
        assert_eq!(output[i], sfunc(a[i], b))
    }
}

#[inline]
fn assign_op_impl<T, D, F, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    feature: F,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    F: Features + Native<T>,
    F::Vector: Signed<T>,
    VFunc: Fn(&mut F::Vector, F::Vector),
    SFunc: Fn(&mut T, T),
{
    let mut a = F::Vector::zeroed(feature);
    let mut b = F::Vector::zeroed(feature);

    let mut rng = rand::thread_rng();
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output = a.clone();
    vfunc(&mut output, b);
    for i in 0..F::Vector::WIDTH {
        sfunc(&mut a[i], b[i]);
        assert_eq!(output[i], a[i])
    }
}

#[inline]
fn assign_scalar_op_impl<T, D, F, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    feature: F,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    F: Features + Native<T>,
    F::Vector: Signed<T>,
    VFunc: Fn(&mut F::Vector, T),
    SFunc: Fn(&mut T, T),
{
    let mut rng = rand::thread_rng();
    let mut a = F::Vector::zeroed(feature);
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output = a.clone();
    vfunc(&mut output, b);
    for i in 0..F::Vector::WIDTH {
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
        $test(0f32, Standard, $handle, $func, $func);
        $test(0f64, Standard, $handle, $func, $func);
        $test(Complex::<f32>::default(), ComplexDistribution::new(Standard, Standard), $handle, $func, $func);
        $test(Complex::<f64>::default(), ComplexDistribution::new(Standard, Standard), $handle, $func, $func);
    };
}

ops_test! { ops_generic, safe_simd::generic::Generic, safe_simd::generic::Generic::new() }
ops_test! { ops_sse, safe_simd::x86::Sse, safe_simd::x86::Sse::new() }
ops_test! { ops_avx, safe_simd::x86::Avx, safe_simd::x86::Avx::new() }
