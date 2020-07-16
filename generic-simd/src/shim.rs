//! Shims for unsupported vector widths.

use crate::vector::{width, Vector};
use core::marker::PhantomData;

#[cfg(feature = "complex")]
use crate::vector::Complex;

/// Determines the doubled width of this vector.
pub trait Double {
    type Doubled: width::Width;
}

impl Double for width::W1 {
    type Doubled = width::W2;
}

impl Double for width::W2 {
    type Doubled = width::W4;
}

impl Double for width::W4 {
    type Doubled = width::W8;
}

/// Shim that doubles the width of a vector.
#[derive(Copy, Clone, Debug)]
pub struct Shim2<Underlying, Scalar>([Underlying; 2], PhantomData<Scalar>);

/// Shim that quadruples the width of a vector.
pub type Shim4<Underlying, Scalar> = Shim2<Shim2<Underlying, Scalar>, Scalar>;

/// Shim that octuples the width of a vector.
pub type Shim8<Underlying, Scalar> = Shim4<Shim2<Underlying, Scalar>, Scalar>;

unsafe impl<Underlying, Scalar> Vector for Shim2<Underlying, Scalar>
where
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy,
{
    type Scalar = Scalar;
    type Token = <Underlying as Vector>::Token;
    type Width = <Underlying::Width as Double>::Doubled;

    #[inline]
    fn splat(feature: Self::Token, from: Self::Scalar) -> Self {
        Self([Underlying::splat(feature, from); 2], PhantomData)
    }
}

impl<Underlying, Scalar> AsRef<[Scalar]> for Shim2<Underlying, Scalar>
where
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy,
{
    #[inline]
    fn as_ref(&self) -> &[Scalar] {
        self.as_slice()
    }
}

impl<Underlying, Scalar> AsMut<[Scalar]> for Shim2<Underlying, Scalar>
where
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [Scalar] {
        self.as_slice_mut()
    }
}

impl<Underlying, Scalar> core::ops::Deref for Shim2<Underlying, Scalar>
where
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy,
{
    type Target = [Scalar];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<Underlying, Scalar> core::ops::DerefMut for Shim2<Underlying, Scalar>
where
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
        self.as_slice_mut()
    }
}

macro_rules! implement {
    {
        @op $trait:ident :: $func:ident
    } => {
        impl<Underlying, Scalar> core::ops::$trait<Self> for Shim2<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Underlying, Output=Underlying>,
        {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self {
                Self([self.0[0].$func(rhs.0[0]), self.0[1].$func(rhs.0[1])], PhantomData)
            }
        }

        impl<Underlying, Scalar> core::ops::$trait<Scalar> for Shim2<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Scalar, Output=Underlying>,
            Scalar: Copy,
        {
            type Output = Self;
            fn $func(self, rhs: Scalar) -> Self {
                Self([self.0[0].$func(rhs), self.0[1].$func(rhs)], PhantomData)
            }
        }
    };

    {
        @op_assign $trait:ident :: $func:ident
    } => {
        impl<Underlying, Scalar> core::ops::$trait<Self> for Shim2<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Underlying>,
            Scalar: Copy,
        {
            fn $func(&mut self, rhs: Self) {
                self.0[0].$func(rhs.0[0]);
                self.0[1].$func(rhs.0[1]);
            }
        }

        impl<Underlying, Scalar> core::ops::$trait<Scalar> for Shim2<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Scalar>,
            Scalar: Copy,
        {
            fn $func(&mut self, rhs: Scalar) {
                self.0[0].$func(rhs);
                self.0[1].$func(rhs);
            }
        }
    };
}

implement! { @op Add::add }
implement! { @op Sub::sub }
implement! { @op Mul::mul }
implement! { @op Div::div }
implement! { @op_assign AddAssign::add_assign }
implement! { @op_assign SubAssign::sub_assign }
implement! { @op_assign MulAssign::mul_assign }
implement! { @op_assign DivAssign::div_assign }

impl<Underlying, Scalar> core::ops::Neg for Shim2<Underlying, Scalar>
where
    Underlying: Copy + core::ops::Neg<Output = Underlying>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1]], PhantomData)
    }
}

impl<Underlying, Scalar> core::iter::Sum<Shim2<Underlying, Scalar>>
    for Option<Shim2<Underlying, Scalar>>
where
    Shim2<Underlying, Scalar>: core::ops::AddAssign,
    Underlying: Copy,
{
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Shim2<Underlying, Scalar>>,
    {
        if let Some(mut sum) = iter.next() {
            for v in iter {
                sum += v;
            }
            Some(sum)
        } else {
            None
        }
    }
}

impl<Underlying, Scalar> core::iter::Sum<Shim2<Underlying, Scalar>>
    for <Shim2<Underlying, Scalar> as Vector>::Scalar
where
    Option<Shim2<Underlying, Scalar>>: core::iter::Sum<Shim2<Underlying, Scalar>>,
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy + num_traits::Num,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Shim2<Underlying, Scalar>>,
    {
        let mut value = Self::zero();
        if let Some(sums) = iter.sum::<Option<Shim2<Underlying, Scalar>>>() {
            for sum in sums.as_slice() {
                value = value + *sum;
            }
        }
        value
    }
}

impl<Underlying, Scalar> core::iter::Product<Shim2<Underlying, Scalar>>
    for Option<Shim2<Underlying, Scalar>>
where
    Shim2<Underlying, Scalar>: core::ops::MulAssign,
    Underlying: Copy,
{
    fn product<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Shim2<Underlying, Scalar>>,
    {
        if let Some(mut sum) = iter.next() {
            for v in iter {
                sum *= v;
            }
            Some(sum)
        } else {
            None
        }
    }
}

impl<Underlying, Scalar> core::iter::Product<Shim2<Underlying, Scalar>>
    for <Shim2<Underlying, Scalar> as Vector>::Scalar
where
    Option<Shim2<Underlying, Scalar>>: core::iter::Product<Shim2<Underlying, Scalar>>,
    Underlying: Vector<Scalar = Scalar>,
    Underlying::Width: Double,
    Scalar: Copy + num_traits::Num,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Shim2<Underlying, Scalar>>,
    {
        let mut value = Self::zero();
        if let Some(products) = iter.product::<Option<Shim2<Underlying, Scalar>>>() {
            for product in products.as_slice() {
                value = value * *product;
            }
        }
        value
    }
}

#[cfg(feature = "complex")]
impl<Underlying, Real> Complex for Shim2<Underlying, num_complex::Complex<Real>>
where
    Option<Underlying::Scalar>: core::iter::Sum<Shim2<Underlying, Underlying::Scalar>>,
    Underlying::Scalar:
        core::ops::AddAssign + core::iter::Sum<Underlying> + core::iter::Product<Underlying>,
    Underlying: Vector<Scalar = num_complex::Complex<Real>>
        + core::iter::Sum<Underlying>
        + core::iter::Product<Underlying>
        + Complex<RealScalar = Real>,
    Underlying::Width: Double,
    Real: Copy + num_traits::Num,
{
    type RealScalar = Real;

    #[inline]
    fn conj(self) -> Self {
        Self([self.0[0].conj(), self.0[1].conj()], PhantomData)
    }

    #[inline]
    fn mul_i(self) -> Self {
        Self([self.0[0].mul_i(), self.0[1].mul_i()], PhantomData)
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        Self([self.0[0].mul_neg_i(), self.0[1].mul_neg_i()], PhantomData)
    }
}
