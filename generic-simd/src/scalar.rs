//! Extensions for scalars.

use crate::vector::{width, Native, NativeWidth, Vector};

/// A scalar value.
pub trait Scalar<Token, Width>: Copy
where
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Vector: Vector<Scalar = Self, Token = Token, Width = Width>;

    /// Create a vector set to zero.
    ///
    /// See [`zeroed`](../vector/trait.Vector.html#method.zeroed).
    #[inline]
    fn zeroed(token: Token) -> Self::Vector {
        Self::Vector::zeroed(token)
    }

    /// Splat a scalar to a vector.
    ///
    /// See [`splat`](../vector/trait.Vector.html#tymethod.splat).
    #[inline]
    fn splat(self, token: Token) -> Self::Vector {
        Self::Vector::splat(token, self)
    }
}

macro_rules! scalar_impl {
    {
        $width:literal,
        $width_type:ty,
        $zeroed:ident,
        $splat:ident
    } => {
        #[doc = "Create a vector with "]
        #[doc = $width]
        #[doc = " set to zero.\n\nSee [`zeroed`](../vector/trait.Vector.html#method.zeroed)."]
        #[inline]
        fn $zeroed(token: Token) -> <Self as Scalar<Token, $width_type>>::Vector {
           <Self as Scalar<Token, $width_type>>::zeroed(token.into())
        }

        #[doc = "Splat a scalar to "]
        #[doc = $width]
        #[doc = ".\n\nSee [`splat`](../vector/trait.Vector.html#tymethod.splat)."]
        #[inline]
        fn $splat(self, token: Token) -> <Self as Scalar<Token, $width_type>>::Vector {
            <Self as Scalar<Token, $width_type>>::splat(self, token.into())
        }
    }
}

/// A scalar value, supporting all vector widths.
pub trait ScalarExt<Token>:
    Native<Token>
    + self::Scalar<Token, width::W1>
    + self::Scalar<Token, width::W2>
    + self::Scalar<Token, width::W4>
    + self::Scalar<Token, width::W8>
    + self::Scalar<Token, NativeWidth<Self, Token>>
where
    Token: crate::arch::Token + From<Token> + Into<Token>,
{
    scalar_impl! { "the native number of lanes", <Self as Native<Token>>::Width, zeroed_native, splat_native }
    scalar_impl! { "1 lane",  width::W1, zeroed1, splat1 }
    scalar_impl! { "2 lanes", width::W2, zeroed2, splat2 }
    scalar_impl! { "4 lanes", width::W4, zeroed4, splat4 }
    scalar_impl! { "8 lanes", width::W8, zeroed8, splat8 }
}

impl<Token, Scalar> ScalarExt<Token> for Scalar
where
    Token: crate::arch::Token,
    Scalar: Native<Token>
        + self::Scalar<Token, width::W1>
        + self::Scalar<Token, width::W2>
        + self::Scalar<Token, width::W4>
        + self::Scalar<Token, width::W8>
        + self::Scalar<Token, NativeWidth<Self, Token>>,
{
}
