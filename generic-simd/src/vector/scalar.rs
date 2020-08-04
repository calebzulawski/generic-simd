//! Extensions for scalars.

use crate::vector::{width, Native, NativeWidth, Vector};

/// A scalar value, parameterized by vector width.
pub trait ScalarWidth<Token, Width>: Copy
where
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Vector: Vector<Scalar = Self, Token = Token, Width = Width>;

    /// Read a vector from a slice without checking the length.
    ///
    /// # Safety
    /// See [`read_unchecked`](../trait.Vector.html#method.read_ptr).
    #[inline]
    unsafe fn read_unchecked(token: Token, from: &[Self]) -> Self::Vector {
        Self::Vector::read_unchecked(token, from)
    }

    /// Read a vector from a slice.
    ///
    /// See [`read`](../trait.Vector.html#method.read).
    #[inline]
    fn read(token: Token, from: &[Self]) -> Self::Vector {
        Self::Vector::read(token, from)
    }

    /// Create a vector set to zero.
    ///
    /// See [`zeroed`](../trait.Vector.html#method.zeroed).
    #[inline]
    fn zeroed(token: Token) -> Self::Vector {
        Self::Vector::zeroed(token)
    }

    /// Splat a scalar to a vector.
    ///
    /// See [`splat`](../trait.Vector.html#tymethod.splat).
    #[inline]
    fn splat(self, token: Token) -> Self::Vector {
        Self::Vector::splat(token, self)
    }
}

macro_rules! scalar_impl {
    {
        $width:literal,
        $width_type:ty,
        $read_ptr:ident,
        $read_unchecked:ident,
        $read:ident,
        $zeroed:ident,
        $splat:ident
    } => {
        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice without checking the length.\n\nSee [`read_unchecked`](../trait.Vector.html#method.read_ptr)."]
        #[inline]
        unsafe fn $read_unchecked(token: Token, from: &[Self]) -> <Self as ScalarWidth<Token, $width_type>>::Vector {
            <Self as ScalarWidth<Token, $width_type>>::read_unchecked(token.into(), from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice.\n\nSee [`read`](../trait.Vector.html#method.read)."]
        #[inline]
        fn $read(token: Token, from: &[Self]) -> <Self as ScalarWidth<Token, $width_type>>::Vector {
            <Self as ScalarWidth<Token, $width_type>>::read(token.into(), from)
        }

        #[doc = "Create a vector with "]
        #[doc = $width]
        #[doc = " set to zero.\n\nSee [`zeroed`](../trait.Vector.html#method.zeroed)."]
        #[inline]
        fn $zeroed(token: Token) -> <Self as ScalarWidth<Token, $width_type>>::Vector {
           <Self as ScalarWidth<Token, $width_type>>::zeroed(token.into())
        }

        #[doc = "Splat a scalar to "]
        #[doc = $width]
        #[doc = ".\n\nSee [`splat`](../trait.Vector.html#tymethod.splat)."]
        #[inline]
        fn $splat(self, token: Token) -> <Self as ScalarWidth<Token, $width_type>>::Vector {
            <Self as ScalarWidth<Token, $width_type>>::splat(self, token.into())
        }
    }
}

/// A scalar value.
pub trait Scalar<Token>:
    Native<Token>
    + ScalarWidth<Token, width::W1>
    + ScalarWidth<Token, width::W2>
    + ScalarWidth<Token, width::W4>
    + ScalarWidth<Token, width::W8>
    + ScalarWidth<Token, NativeWidth<Self, Token>>
where
    Token: crate::arch::Token + From<Token> + Into<Token>,
{
    scalar_impl! { "the native number of lanes", <Self as Native<Token>>::Width, read_native_ptr, read_native_unchecked, read_native, zeroed_native, splat_native }
    scalar_impl! { "1 lane",  width::W1, read1_ptr, read1_unchecked, read1, zeroed1, splat1 }
    scalar_impl! { "2 lanes", width::W2, read2_ptr, read2_unchecked, read2, zeroed2, splat2 }
    scalar_impl! { "4 lanes", width::W4, read4_ptr, read4_unchecked, read4, zeroed4, splat4 }
    scalar_impl! { "8 lanes", width::W8, read8_ptr, read8_unchecked, read8, zeroed8, splat8 }
}

impl<Token, Scalar> self::Scalar<Token> for Scalar
where
    Token: crate::arch::Token,
    Scalar: Native<Token>
        + ScalarWidth<Token, width::W1>
        + ScalarWidth<Token, width::W2>
        + ScalarWidth<Token, width::W4>
        + ScalarWidth<Token, width::W8>
        + ScalarWidth<Token, NativeWidth<Self, Token>>,
{
}
