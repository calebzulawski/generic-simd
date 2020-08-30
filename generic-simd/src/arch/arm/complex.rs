use crate::{
    arch::{arm::Neon, Token},
    scalar::Scalar,
    shim::{Shim2, Shim4, Shim8},
    vector::{width, Native, Vector},
};
use num_complex::Complex;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use core::arch::arm::*;

impl Native<Neon> for Complex<f32> {
    type Width = width::W2;
}

impl Native<Neon> for Complex<f64> {
    type Width = width::W1;
}

/// A NEON vector of `Complex<f32>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x1(float32x2_t);

/// A NEON vector of `Complex<f32>`s.
///
/// Requires feature `"complex"`.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf32x2(float32x4_t);

/// A NEON vector of `Complex<f64>`s.
///
/// Requires feature `"complex"`.
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cf64x1(float64x2_t);

impl Scalar<Neon, width::W1> for Complex<f32> {
    type Vector = cf32x1;
}

impl Scalar<Neon, width::W2> for Complex<f32> {
    type Vector = cf32x2;
}

impl Scalar<Neon, width::W4> for Complex<f32> {
    type Vector = Shim2<cf32x2, Complex<f32>>;
}

impl Scalar<Neon, width::W8> for Complex<f32> {
    type Vector = Shim4<cf32x2, Complex<f32>>;
}

#[cfg(target_arch = "arm")]
impl Scalar<Neon, width::W1> for Complex<f64> {
    type Vector = crate::arch::generic::cf64x1;
}

#[cfg(target_arch = "aarch64")]
impl Scalar<Neon, width::W1> for Complex<f64> {
    type Vector = cf64x1;
}

impl Scalar<Neon, width::W2> for Complex<f64> {
    type Vector = Shim2<<Self as Scalar<Neon, width::W1>>::Vector, Complex<f64>>;
}

impl Scalar<Neon, width::W4> for Complex<f64> {
    type Vector = Shim4<<Self as Scalar<Neon, width::W1>>::Vector, Complex<f64>>;
}

impl Scalar<Neon, width::W8> for Complex<f64> {
    type Vector = Shim8<<Self as Scalar<Neon, width::W1>>::Vector, Complex<f64>>;
}

arithmetic_ops! {
    feature: Neon::new_unchecked(),
    for cf32x1:
        add -> (vadd_f32),
        sub -> (vsub_f32),
        mul -> (),
        div -> ()
}

arithmetic_ops! {
    feature: Neon::new_unchecked(),
    for cf32x2:
        add -> (vaddq_f32),
        sub -> (vsubq_f32),
        mul -> (),
        div -> ()
}

#[cfg(target_arch = "aarch64")]
arithmetic_ops! {
    feature: Neon::new_unchecked(),
    for cf64x1:
        add -> (vaddq_f64),
        sub -> (vsubq_f64),
        mul -> (),
        div -> ()
}

impl core::ops::Neg for cf32x1 {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = -*v;
        }
        self
    }
}

impl core::ops::Neg for cf32x2 {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = -*v;
        }
        self
    }
}

#[cfg(target_arch = "aarch64")]
impl core::ops::Neg for cf64x1 {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = -*v;
        }
        self
    }
}

as_slice! { cf32x1 }
as_slice! { cf32x2 }
#[cfg(target_arch = "aarch64")]
as_slice! { cf64x1 }

unsafe impl Vector for cf32x1 {
    type Scalar = Complex<f32>;

    type Token = Neon;

    type Width = crate::vector::width::W1;

    type Underlying = float32x2_t;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        // TODO use vdup
        Self(unsafe { core::mem::zeroed() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        // TODO use vdup
        let mut v: Self = unsafe { core::mem::zeroed() };
        v[0] = from;
        v
    }
}

unsafe impl Vector for cf32x2 {
    type Scalar = Complex<f32>;

    type Token = Neon;

    type Width = crate::vector::width::W2;

    type Underlying = float32x4_t;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        // TODO use vdup
        Self(unsafe { core::mem::zeroed() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        // TODO use vdup
        let mut v: Self = unsafe { core::mem::zeroed() };
        v[0] = from;
        v[1] = from;
        v
    }
}

#[cfg(target_arch = "aarch64")]
unsafe impl Vector for cf64x1 {
    type Scalar = Complex<f64>;

    type Token = Neon;

    type Width = crate::vector::width::W1;

    type Underlying = float64x2_t;

    #[inline]
    fn zeroed(_: Self::Token) -> Self {
        // TODO use vdup
        Self(unsafe { core::mem::zeroed() })
    }

    #[inline]
    fn splat(_: Self::Token, from: Self::Scalar) -> Self {
        // TODO use vdup
        let mut v: Self = unsafe { core::mem::zeroed() };
        v[0] = from;
        v
    }
}

impl crate::vector::Complex for cf32x1 {
    type RealScalar = f32;

    #[inline]
    fn conj(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = v.conj();
        }
        self
    }

    #[inline]
    fn mul_i(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = Complex::new(-v.im, v.re);
        }
        self
    }

    #[inline]
    fn mul_neg_i(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = Complex::new(v.im, -v.re);
        }
        self
    }
}

impl crate::vector::Complex for cf32x2 {
    type RealScalar = f32;

    #[inline]
    fn conj(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = v.conj();
        }
        self
    }

    #[inline]
    fn mul_i(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = Complex::new(-v.im, v.re);
        }
        self
    }

    #[inline]
    fn mul_neg_i(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = Complex::new(v.im, -v.re);
        }
        self
    }
}

#[cfg(target_arch = "aarch64")]
impl crate::vector::Complex for cf64x1 {
    type RealScalar = f32;

    #[inline]
    fn conj(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = v.conj();
        }
        self
    }

    #[inline]
    fn mul_i(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = Complex::new(-v.im, v.re);
        }
        self
    }

    #[inline]
    fn mul_neg_i(mut self) -> Self {
        for v in self.as_slice_mut() {
            *v = Complex::new(v.im, -v.re);
        }
        self
    }
}
