//! Shims for unsupported vector widths.

use crate::vector::{Complex, Vector};
use arch_types::marker::Superset;
use core::marker::PhantomData;

macro_rules! implement {
    {
        $type:ident, $size_lit:literal, $size_string:literal
    } => {
        #[doc = "Shim for a vector of "]
        #[doc = $size_string]
        #[doc = " native vectors"]
        #[derive(Copy, Clone, Debug)]
        pub struct $type<Underlying, Scalar>([Underlying; $size_lit], PhantomData<Scalar>);

        unsafe impl<Underlying, Scalar> Vector for $type<Underlying, Scalar>
        where
            Underlying: Vector<Scalar = Scalar>,
            Scalar: Copy,
        {
            type Scalar = Scalar;
            type Feature = <Underlying as Vector>::Feature;

            #[inline]
            fn splat(feature: impl Superset<Self::Feature>, from: Self::Scalar) -> Self {
                Self([Underlying::splat(feature, from); $size_lit], PhantomData)
            }
        }

        impl<Underlying, Scalar> AsRef<[Scalar]> for $type<Underlying, Scalar>
        where
            Underlying: Vector<Scalar = Scalar>,
            Scalar: Copy,
        {
            fn as_ref(&self) -> &[Scalar] {
                self.as_slice()
            }
        }

        impl<Underlying, Scalar> AsMut<[Scalar]> for $type<Underlying, Scalar>
        where
            Underlying: Vector<Scalar = Scalar>,
            Scalar: Copy,
        {
            fn as_mut(&mut self) -> &mut [Scalar] {
                self.as_slice_mut()
            }
        }

        impl<Underlying, Scalar> core::ops::Deref for $type<Underlying, Scalar>
        where
            Underlying: Vector<Scalar = Scalar>,
            Scalar: Copy,
        {
            type Target = [Scalar];
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl<Underlying, Scalar> core::ops::DerefMut for $type<Underlying, Scalar>
        where
            Underlying: Vector<Scalar = Scalar>,
            Scalar: Copy,
        {
            fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
                self.as_slice_mut()
            }
        }

        implement! { @op $type => Add::add }
        implement! { @op $type => Sub::sub }
        implement! { @op $type => Mul::mul }
        implement! { @op $type => Div::div }
        implement! { @op_assign $type => AddAssign::add_assign }
        implement! { @op_assign $type => SubAssign::sub_assign }
        implement! { @op_assign $type => MulAssign::mul_assign }
        implement! { @op_assign $type => DivAssign::div_assign }

        impl<Underlying, Scalar> core::ops::Neg for $type<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::Neg<Output=Underlying>,
        {
            type Output = Self;
            fn neg(mut self) -> Self {
                for x in self.0.iter_mut() {
                    *x = -*x;
                }
                self
            }
        }

        impl<Underlying, Real> Complex<Real> for $type<Underlying, num_complex::Complex<Real>>
        where
            Underlying: Vector<Scalar = num_complex::Complex<Real>> + Complex<Real>,
            Real: Copy,
        {
            fn mul_i(mut self) -> Self {
                for x in self.0.iter_mut() {
                    *x = x.mul_i()
                }
                self
            }

            fn mul_neg_i(mut self) -> Self {
                for x in self.0.iter_mut() {
                    *x = x.mul_neg_i()
                }
                self
            }
        }
    };

    {
        @op $type:ident => $trait:ident :: $func:ident
    } => {
        impl<Underlying, Scalar> core::ops::$trait<Self> for $type<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Underlying, Output=Underlying>,
        {
            type Output = Self;
            fn $func(mut self, rhs: Self) -> Self {
                for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
                    *a = a.$func(*b);
                }
                self
            }
        }

        impl<Underlying, Scalar> core::ops::$trait<Scalar> for $type<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Scalar, Output=Underlying>,
            Scalar: Copy,
        {
            type Output = Self;
            fn $func(mut self, rhs: Scalar) -> Self {
                for x in self.0.iter_mut() {
                    *x = x.$func(rhs);
                }
                self
            }
        }
    };

    {
        @op_assign $type:ident => $trait:ident :: $func:ident
    } => {
        impl<Underlying, Scalar> core::ops::$trait<Self> for $type<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Underlying>,
            Scalar: Copy,
        {
            fn $func(&mut self, rhs: Self) {
                for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
                    a.$func(*b);
                }
            }
        }

        impl<Underlying, Scalar> core::ops::$trait<Scalar> for $type<Underlying, Scalar>
        where
            Underlying: Copy + core::ops::$trait<Scalar>,
            Scalar: Copy,
        {
            fn $func(&mut self, rhs: Scalar) {
                for x in self.0.iter_mut() {
                    x.$func(rhs);
                }
            }
        }
    };
}

implement! { Shim2, 2, "2" }
implement! { Shim4, 4, "4" }
implement! { Shim8, 8, "8" }
