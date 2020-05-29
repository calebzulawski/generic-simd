//! Shims for unsupported vector widths.

use crate::vector::Vector;
use arch_types::marker::Superset;
use core::marker::PhantomData;

#[cfg(feature = "complex")]
use crate::vector::Complex;

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
    Scalar: Copy,
{
    type Scalar = Scalar;
    type Feature = <Underlying as Vector>::Feature;

    #[inline]
    fn splat(feature: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
        Self([Underlying::splat(feature, from); 2], PhantomData)
    }
}

impl<Underlying, Scalar> AsRef<[Scalar]> for Shim2<Underlying, Scalar>
where
    Underlying: Vector<Scalar = Scalar>,
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

#[cfg(feature = "complex")]
impl<Underlying, Real> Complex for Shim2<Underlying, num_complex::Complex<Real>>
where
    Underlying: Vector<Scalar = num_complex::Complex<Real>> + Complex<RealScalar = Real>,
    Real: Copy,
{
    type RealScalar = Real;

    #[inline]
    fn mul_i(self) -> Self {
        Self([self.0[0].mul_i(), self.0[1].mul_i()], PhantomData)
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        Self([self.0[0].mul_neg_i(), self.0[1].mul_neg_i()], PhantomData)
    }
}
