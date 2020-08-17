//! Extensions for pointers to vectors.

use crate::{
    scalar::ScalarWidth,
    vector::{width, Native, NativeWidth, Vector},
};

/// A pointer to a vector, parameterized by vector width.
pub trait PointerWidth<Token, Width>: Copy
where
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Vector: Vector<Token = Token, Width = Width>;

    /// Read a vector from a pointer.
    ///
    /// # Safety
    /// See [`read_ptr`](../trait.Vector.html#method.read_ptr).
    unsafe fn vector_read(self, token: Token) -> Self::Vector;

    /// Read a vector from a vector-aligned pointer.
    ///
    /// # Safety
    /// See [`read_aligned_ptr`](../trait.Vector.html#method.read_aligned_ptr).
    unsafe fn vector_read_aligned(self, token: Token) -> Self::Vector;
}

impl<T, Token, Width> PointerWidth<Token, Width> for *const T
where
    T: ScalarWidth<Token, Width>,
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Vector = T::Vector;

    #[inline]
    unsafe fn vector_read(self, token: Token) -> Self::Vector {
        Self::Vector::read_ptr(token, self)
    }

    #[inline]
    unsafe fn vector_read_aligned(self, token: Token) -> Self::Vector {
        Self::Vector::read_aligned_ptr(token, self)
    }
}

impl<T, Token, Width> PointerWidth<Token, Width> for *mut T
where
    T: ScalarWidth<Token, Width>,
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Vector = T::Vector;

    #[inline]
    unsafe fn vector_read(self, token: Token) -> Self::Vector {
        Self::Vector::read_ptr(token, self)
    }

    #[inline]
    unsafe fn vector_read_aligned(self, token: Token) -> Self::Vector {
        Self::Vector::read_aligned_ptr(token, self)
    }
}

macro_rules! pointer_impl {
    {
        $width:literal,
        $width_type:ty,
        $read_unaligned:ident,
        $read_aligned:ident
    } => {
        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a pointer.\n\n# Safety\nSee [`read_ptr`](../trait.Vector.html#method.read_ptr)."]
        #[inline]
        unsafe fn $read_unaligned(self, token: Token) -> <Self as PointerWidth<Token, $width_type>>::Vector {
            <Self as PointerWidth<Token, $width_type>>::vector_read(self, token)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a vector-aligned pointer.\n\n# Safety\nSee [`read_aligned_ptr`](../trait.Vector.html#method.read_aligned_ptr)."]
        #[inline]
        unsafe fn $read_aligned(self, token: Token) -> <Self as PointerWidth<Token, $width_type>>::Vector {
            <Self as PointerWidth<Token, $width_type>>::vector_read_aligned(self, token)
        }
    }
}

/// A pointer to a vector.
pub trait Pointer<Token>:
    Native<Token>
    + PointerWidth<Token, width::W1>
    + PointerWidth<Token, width::W2>
    + PointerWidth<Token, width::W4>
    + PointerWidth<Token, width::W8>
    + PointerWidth<Token, NativeWidth<Self, Token>>
where
    Token: crate::arch::Token,
{
    pointer_impl! { "the native number of lanes", <Self as Native<Token>>::Width, vector_read_native, vector_read_aligned_native }
    pointer_impl! { "1 lane",  width::W1, vector_read1, vector_read1_aligned }
    pointer_impl! { "2 lanes", width::W2, vector_read2, vector_read2_aligned }
    pointer_impl! { "4 lanes", width::W4, vector_read4, vector_read4_aligned }
    pointer_impl! { "8 lanes", width::W8, vector_read8, vector_read8_aligned }
}

impl<T, Token> Pointer<Token> for T
where
    T: Native<Token>
        + PointerWidth<Token, width::W1>
        + PointerWidth<Token, width::W2>
        + PointerWidth<Token, width::W4>
        + PointerWidth<Token, width::W8>
        + PointerWidth<Token, NativeWidth<Self, Token>>,
    Token: crate::arch::Token,
{
}
