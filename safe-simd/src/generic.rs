//! Generic vector types for any platform.

/// A generic instruction set handle supported by all CPUs.
#[derive(Clone, Copy, Debug, Default)]
pub struct Generic(());

/// A generic vector of one `f32`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vf32(f32);

/// A generic vector of one `f64`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vf64(f64);

impl crate::vector::Feature for Generic {
    fn new() -> Option<Self> {
        Some(Self(()))
    }

    unsafe fn new_unchecked() -> Self {
        Self(())
    }

    fn apply<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
        f(self)
    }
}

impl crate::vector::Capability<f32> for Generic {
    type Vector = Vf32;
}

impl crate::vector::Capability<f64> for Generic {
    type Vector = Vf64;
}

arithmetic_ops! {
    for Vf32:
        add -> core::ops::Add::add,
        sub -> core::ops::Sub::sub,
        mul -> core::ops::Mul::mul,
        div -> core::ops::Div::div
}

arithmetic_ops! {
    for Vf64:
        add -> core::ops::Add::add,
        sub -> core::ops::Sub::sub,
        mul -> core::ops::Mul::mul,
        div -> core::ops::Div::div
}

impl core::ops::Neg for Vf32 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl core::ops::Neg for Vf64 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

as_slice! { Vf32 }
as_slice! { Vf64 }

unsafe impl crate::vector::VectorCore for Vf32 {
    type Scalar = f32;

    #[inline]
    unsafe fn splat(from: Self::Scalar) -> Self {
        Self(from)
    }
}
unsafe impl crate::vector::VectorCore for Vf64 {
    type Scalar = f64;

    #[inline]
    unsafe fn splat(from: Self::Scalar) -> Self {
        Self(from)
    }
}
