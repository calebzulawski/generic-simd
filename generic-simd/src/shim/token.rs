use crate::arch;
use crate::vector::Vector;
use core::marker::PhantomData;

#[cfg(feature = "complex")]
use crate::vector::Complex;

/// Shim that converts the associated token.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct ShimToken<Underlying, Scalar, Token>(Underlying, PhantomData<(Scalar, Token)>);

unsafe impl<Underlying, Scalar, Token> Vector for ShimToken<Underlying, Scalar, Token>
where
    Underlying: Vector<Scalar = Scalar>,
    Scalar: Copy,
    Token: arch::Token + Into<<Underlying as Vector>::Token>,
{
    type Scalar = Scalar;
    type Token = Token;
    type Width = <Underlying as Vector>::Width;
    type Underlying = <Underlying as Vector>::Underlying;

    #[inline]
    fn zeroed(token: Self::Token) -> Self {
        Self(Underlying::zeroed(token.into()), PhantomData)
    }

    #[inline]
    fn splat(token: Self::Token, from: Self::Scalar) -> Self {
        Self(Underlying::splat(token.into(), from), PhantomData)
    }
}

impl<Underlying, Scalar, Token> AsRef<[Scalar]> for ShimToken<Underlying, Scalar, Token>
where
    Underlying: AsRef<[Scalar]>,
{
    #[inline]
    fn as_ref(&self) -> &[Scalar] {
        self.0.as_ref()
    }
}

impl<Underlying, Scalar, Token> AsMut<[Scalar]> for ShimToken<Underlying, Scalar, Token>
where
    Underlying: AsMut<[Scalar]>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [Scalar] {
        self.0.as_mut()
    }
}

impl<Underlying, Scalar, Token> core::ops::Deref for ShimToken<Underlying, Scalar, Token>
where
    Underlying: core::ops::Deref,
{
    type Target = Underlying::Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Underlying, Scalar, Token> core::ops::DerefMut for ShimToken<Underlying, Scalar, Token>
where
    Underlying: core::ops::DerefMut,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
        &mut self.0
    }
}

macro_rules! implement {
    {
        @op $trait:ident :: $func:ident
    } => {
        impl<Underlying, Scalar, Token> core::ops::$trait<Self> for ShimToken<Underlying, Scalar, Token>
        where
            Underlying: Copy + core::ops::$trait<Underlying, Output=Underlying>,
        {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: Self) -> Self {
                Self((self.0).$func(rhs.0), PhantomData)
            }
        }

        impl<Underlying, Scalar, Token> core::ops::$trait<Scalar> for ShimToken<Underlying, Scalar, Token>
        where
            Underlying: Copy + core::ops::$trait<Scalar, Output=Underlying>,
            Scalar: Copy,
        {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: Scalar) -> Self {
                Self((self.0).$func(rhs), PhantomData)
            }
        }
    };

    {
        @op_assign $trait:ident :: $func:ident
    } => {
        impl<Underlying, Scalar, Token> core::ops::$trait<Self> for ShimToken<Underlying, Scalar, Token>
        where
            Underlying: Copy + core::ops::$trait<Underlying>,
            Scalar: Copy,
        {
            #[inline]
            fn $func(&mut self, rhs: Self) {
                (self.0).$func(rhs.0);
            }
        }

        impl<Underlying, Scalar, Token> core::ops::$trait<Scalar> for ShimToken<Underlying, Scalar, Token>
        where
            Underlying: Copy + core::ops::$trait<Scalar>,
            Scalar: Copy,
        {
            #[inline]
            fn $func(&mut self, rhs: Scalar) {
                (self.0).$func(rhs);
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

impl<Underlying, Scalar, Token> core::ops::Neg for ShimToken<Underlying, Scalar, Token>
where
    Underlying: Copy + core::ops::Neg<Output = Underlying>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, PhantomData)
    }
}

impl<Underlying, Scalar, Token> core::iter::Sum<ShimToken<Underlying, Scalar, Token>>
    for Option<ShimToken<Underlying, Scalar, Token>>
where
    ShimToken<Underlying, Scalar, Token>: core::ops::AddAssign,
    Underlying: Copy,
{
    #[inline]
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = ShimToken<Underlying, Scalar, Token>>,
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

impl<Underlying, Scalar, Token> core::iter::Sum<ShimToken<Underlying, Scalar, Token>>
    for <ShimToken<Underlying, Scalar, Token> as Vector>::Scalar
where
    Option<ShimToken<Underlying, Scalar, Token>>:
        core::iter::Sum<ShimToken<Underlying, Scalar, Token>>,
    Underlying: Vector<Scalar = Scalar>,
    Scalar: Copy + core::ops::Add<Self, Output = Self> + Default,
    Token: arch::Token,
    Underlying::Token: From<Token>,
{
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = ShimToken<Underlying, Scalar, Token>>,
    {
        let mut value = Self::default();
        if let Some(sums) = iter.sum::<Option<ShimToken<Underlying, Scalar, Token>>>() {
            for sum in sums.as_slice() {
                value = value + *sum;
            }
        }
        value
    }
}

impl<Underlying, Scalar, Token> core::iter::Product<ShimToken<Underlying, Scalar, Token>>
    for Option<ShimToken<Underlying, Scalar, Token>>
where
    ShimToken<Underlying, Scalar, Token>: core::ops::MulAssign,
    Underlying: Copy,
{
    #[inline]
    fn product<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = ShimToken<Underlying, Scalar, Token>>,
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

impl<Underlying, Scalar, Token> core::iter::Product<ShimToken<Underlying, Scalar, Token>>
    for <ShimToken<Underlying, Scalar, Token> as Vector>::Scalar
where
    Option<ShimToken<Underlying, Scalar, Token>>:
        core::iter::Product<ShimToken<Underlying, Scalar, Token>>,
    Underlying: Vector<Scalar = Scalar>,
    Scalar: Copy + core::ops::Mul<Self, Output = Self> + Default,
    Token: arch::Token,
    Underlying::Token: From<Token>,
{
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = ShimToken<Underlying, Scalar, Token>>,
    {
        let mut value = Self::default();
        if let Some(products) = iter.product::<Option<ShimToken<Underlying, Scalar, Token>>>() {
            for product in products.as_slice() {
                value = value * *product;
            }
        }
        value
    }
}

#[cfg(feature = "complex")]
impl<Underlying, Real, Token> Complex for ShimToken<Underlying, num_complex::Complex<Real>, Token>
where
    Underlying: Vector<Scalar = num_complex::Complex<Real>> + Complex<RealScalar = Real>,
    Real: Copy,
    Token: arch::Token,
    Underlying::Token: From<Token>,
{
    type RealScalar = Real;

    #[inline]
    fn conj(self) -> Self {
        Self(self.0.conj(), PhantomData)
    }

    #[inline]
    fn mul_i(self) -> Self {
        Self(self.0.mul_i(), PhantomData)
    }

    #[inline]
    fn mul_neg_i(self) -> Self {
        Self(self.0.mul_neg_i(), PhantomData)
    }
}
