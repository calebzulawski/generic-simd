/// Generic instruction set handle.
#[derive(Clone, Copy, Debug, Default)]
pub struct Generic(());

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vf32(f32);

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

    #[inline]
    unsafe fn load_ptr(&self, from: *const f32) -> Vf32 {
        Vf32(from.read())
    }

    #[inline]
    fn splat(&self, from: f32) -> Vf32 {
        Vf32(from)
    }

    #[inline]
    fn zero(&self) -> Vf32 {
        Vf32(0.)
    }
}

impl crate::vector::Capability<f64> for Generic {
    type Vector = Vf64;

    #[inline]
    unsafe fn load_ptr(&self, from: *const f64) -> Vf64 {
        Vf64(from.read())
    }

    #[inline]
    fn splat(&self, from: f64) -> Vf64 {
        Vf64(from)
    }

    #[inline]
    fn zero(&self) -> Vf64 {
        Vf64(0.)
    }
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
    unsafe fn store_ptr(self, to: *mut f32) {
        to.write(self.0)
    }
}
unsafe impl crate::vector::VectorCore for Vf64 {
    type Scalar = f64;

    #[inline]
    unsafe fn store_ptr(self, to: *mut f64) {
        to.write(self.0)
    }
}
