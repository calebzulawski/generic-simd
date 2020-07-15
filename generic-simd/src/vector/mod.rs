//! Vector type interfaces.

pub mod width;

use crate::arch::Token;
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub trait ScalarSized<Token, Width>: Copy
where
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Token: crate::arch::Token + From<Token>;
    type Vector: Vector<Scalar = Self, Token = Self::Token, Width = Width>;

    /// Read a vector from a pointer.
    ///
    /// # Safety
    /// See [`read_ptr`](trait.Vector.html#method.read_ptr).
    #[inline]
    unsafe fn read_ptr(token: Token, from: *const Self) -> Self::Vector {
        Self::Vector::read_ptr(token, from)
    }

    /// Read a vector from a slice without checking the length.
    ///
    /// # Safety
    /// See [`read_unchecked`](trait.Vector.html#method.read_ptr).
    #[inline]
    unsafe fn read_unchecked(token: Token, from: &[Self]) -> Self::Vector {
        Self::Vector::read_unchecked(token, from)
    }

    /// Read a vector from a slice.
    ///
    /// See [`read`](trait.Vector.html#method.read).
    #[inline]
    fn read(token: Token, from: &[Self]) -> Self::Vector {
        Self::Vector::read(token, from)
    }

    /// Create a vector set to zero.
    ///
    /// See [`zeroed`](trait.Vector.html#method.zeroed).
    #[inline]
    fn zeroed(token: Token) -> Self::Vector {
        Self::Vector::zeroed(token)
    }

    /// Splat a scalar to a vector.
    ///
    /// See [`splat`](trait.Vector.html#tymethod.splat).
    #[inline]
    fn splat(self, token: Token) -> Self::Vector {
        Self::Vector::splat(token, self)
    }

    /// Align a slice of scalars to vectors.
    ///
    /// See [`align`](../slice/fn.align.html).
    #[inline]
    fn align(token: Token, slice: &[Self]) -> (&[Self], &[Self::Vector], &[Self]) {
        crate::slice::align(Self::Token::from(token), slice)
    }

    /// Align a slice of scalars to vectors.
    ///
    /// See [`align`](../slice/fn.align.html).
    #[inline]
    fn align_mut(
        token: Token,
        slice: &mut [Self],
    ) -> (&mut [Self], &mut [Self::Vector], &mut [Self]) {
        crate::slice::align_mut(Self::Token::from(token), slice)
    }

    /// Create a slice of overlapping vectors from a slice of scalars.
    ///
    /// See [`overlapping`](../slice/fn.overlapping.html).
    #[inline]
    fn overlapping(token: Token, slice: &[Self]) -> crate::slice::Overlapping<'_, Self::Vector> {
        crate::slice::Overlapping::new(Self::Token::from(token), slice)
    }

    /// Create a mutable slice of overlapping vectors from a slice of scalars.
    ///
    /// See [`overlapping_mut`](../slice/fn.overlapping_mut.html).
    #[inline]
    fn overlapping_mut(
        token: Token,
        slice: &mut [Self],
    ) -> crate::slice::OverlappingMut<'_, Self::Vector> {
        crate::slice::OverlappingMut::new(Self::Token::from(token), slice)
    }
}

macro_rules! token_impl {
    {
        $width:literal,
        $width_type:ty,
        $read_ptr:ident,
        $read_unchecked:ident,
        $read:ident,
        $zeroed:ident,
        $splat:ident,
        $align:ident,
        $align_mut:ident,
        $overlapping:ident,
        $overlapping_mut:ident
    } => {
        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a pointer.\n\nSee [`read_ptr`](trait.Vector.html#method.read_ptr)."]
        #[inline]
        unsafe fn $read_ptr(token: Token, from: *const Self) -> <Self as ScalarSized<Token, $width_type>>::Vector {
            <Self as ScalarSized<Token, $width_type>>::read_ptr(token.into(), from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice without checking the length.\n\nSee [`read_unchecked`](trait.Vector.html#method.read_ptr)."]
        #[inline]
        unsafe fn $read_unchecked(token: Token, from: &[Self]) -> <Self as ScalarSized<Token, $width_type>>::Vector {
            <Self as ScalarSized<Token, $width_type>>::read_unchecked(token.into(), from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice.\n\nSee [`read`](trait.Vector.html#method.read)."]
        #[inline]
        fn $read(token: Token, from: &[Self]) -> <Self as ScalarSized<Token, $width_type>>::Vector {
            <Self as ScalarSized<Token, $width_type>>::read(token.into(), from)
        }

        #[doc = "Create a vector with "]
        #[doc = $width]
        #[doc = " set to zero.\n\nSee [`zeroed`](trait.Vector.html#method.zeroed)."]
        #[inline]
        fn $zeroed(token: Token) -> <Self as ScalarSized<Token, $width_type>>::Vector {
           <Self as ScalarSized<Token, $width_type>>::zeroed(token.into())
        }

        #[doc = "Splat a scalar to "]
        #[doc = $width]
        #[doc = ".\n\nSee [`splat`](trait.Vector.html#tymethod.splat)."]
        #[inline]
        fn $splat(self, token: Token) -> <Self as ScalarSized<Token, $width_type>>::Vector {
            <Self as ScalarSized<Token, $width_type>>::splat(self, token.into())
        }

        #[doc = "Align a slice of scalars to vectors with "]
        #[doc = $width]
        #[doc = ".\n\nSee [`align`](../slice/fn.align.html)."]
        #[inline]
        fn $align(token: Token, slice: &[Self]) -> (&[Self], &[<Self as ScalarSized<Token, $width_type>>::Vector], &[Self]) {
            <Self as ScalarSized<Token, $width_type>>::align(token.into(), slice)
        }

        #[doc = "Align a slice of scalars to vectors with "]
        #[doc = $width]
        #[doc = ".\n\nSee [`align_mut`](../slice/fn.align_mut.html)."]
        #[inline]
        fn $align_mut(token: Token, slice: &mut [Self]) -> (&mut [Self], &mut [<Self as ScalarSized<Token, $width_type>>::Vector], &mut [Self]) {
            <Self as ScalarSized<Token, $width_type>>::align_mut(token.into(), slice)
        }

        #[doc = "Create a slice of overlapping vectors of "]
        #[doc = $width]
        #[doc = "from a slice of scalars.\n\nSee [`overlapping`](../slice/fn.overlapping.html)."]
        #[inline]
        fn $overlapping(token: Token, slice: &[Self]) -> crate::slice::Overlapping<'_, <Self as ScalarSized<Token, $width_type>>::Vector> {
            <Self as ScalarSized<Token, $width_type>>::overlapping(token.into(), slice)
        }

        #[doc = "Create a mutable slice of overlapping vectors of "]
        #[doc = $width]
        #[doc = "from a slice of scalars.\n\nSee [`overlapping_mut`](../slice/fn.overlapping_mut.html)."]
        #[inline]
        fn $overlapping_mut(
            token: Token,
            slice: &mut [Self],
        ) -> crate::slice::OverlappingMut<'_, <Self as ScalarSized<Token, $width_type>>::Vector> {
            <Self as ScalarSized<Token, $width_type>>::overlapping_mut(token.into(), slice)
        }
    }
}

/// Indicates the widest native vector.
pub trait Native<Token> {
    type Width: width::Width;
}

/// Convenience type for the widest native vector size.
pub type NativeWidth<Scalar, Token> = <Scalar as Native<Token>>::Width;

/// Convenience type for the widest native vector.
pub type NativeVector<Scalar, Token> = SizedVector<Scalar, NativeWidth<Scalar, Token>, Token>;

/// Convenience type for the vector with a particular width.
pub type SizedVector<Scalar, Width, Token> = <Scalar as ScalarSized<Token, Width>>::Vector;

/// Handle for working with all vector sizes.
pub trait Scalar<Token>:
    Native<Token>
    + ScalarSized<Token, width::W1>
    + ScalarSized<Token, width::W2>
    + ScalarSized<Token, width::W4>
    + ScalarSized<Token, width::W8>
    + ScalarSized<Token, NativeWidth<Self, Token>>
where
    Token: crate::arch::Token + From<Token> + Into<Token>,
{
    token_impl! { "the native number of lanes", <Self as Native<Token>>::Width, read_native_ptr, read_native_unchecked, read_native, zeroed_native, splat_native, align_native, align_native_mut, overlapping_native, overlapping_native_mut }
    token_impl! { "1 lane",  width::W1, read1_ptr, read1_unchecked, read1, zeroed1, splat1, align1, align1_mut, overlapping1, overlapping1_mut }
    token_impl! { "2 lanes", width::W2, read2_ptr, read2_unchecked, read2, zeroed2, splat2, align2, align2_mut, overlapping2, overlapping2_mut }
    token_impl! { "4 lanes", width::W4, read4_ptr, read4_unchecked, read4, zeroed4, splat4, align4, align4_mut, overlapping4, overlapping4_mut }
    token_impl! { "8 lanes", width::W8, read8_ptr, read8_unchecked, read8, zeroed8, splat8, align8, align8_mut, overlapping8, overlapping8_mut }
}

impl<Token, Scalar> self::Scalar<Token> for Scalar
where
    Token: crate::arch::Token,
    Scalar: Native<Token>
        + ScalarSized<Token, width::W1>
        + ScalarSized<Token, width::W2>
        + ScalarSized<Token, width::W4>
        + ScalarSized<Token, width::W8>
        + ScalarSized<Token, NativeWidth<Self, Token>>,
{
}

/// The fundamental vector type.
///
/// # Safety
/// This trait may only be implemented for types that have the memory layout of an array of
/// `Scalar` with length `width()`.
pub unsafe trait Vector: Copy {
    /// The type of elements in the vector.
    type Scalar: Copy;

    /// The token that proves support for this vector on the CPU.
    type Token: Token + From<Self::Token> + Into<Self::Token>;

    /// The number of elements in the vector.
    type Width: width::Width;

    /// Returns the number of lanes.
    fn width() -> usize {
        <Self::Width as width::Width>::VALUE
    }

    /// Returns a slice containing the vector.
    #[inline]
    fn as_slice(&self) -> &[Self::Scalar] {
        unsafe { core::slice::from_raw_parts(self as *const _ as *const _, Self::width()) }
    }

    /// Returns a mutable slice containing the vector.
    #[inline]
    fn as_slice_mut(&mut self) -> &mut [Self::Scalar] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as *mut _, Self::width()) }
    }

    /// Read from a pointer.
    ///
    /// # Safety
    /// * `from` must point to an array of length at least `width()`.
    #[inline]
    unsafe fn read_ptr(
        #[allow(unused_variables)] token: impl Into<Self::Token>,
        from: *const Self::Scalar,
    ) -> Self {
        (from as *const Self).read_unaligned()
    }

    /// Read from a slice without checking the length.
    ///
    /// # Safety
    /// * `from` be length at least `width()`.
    #[inline]
    unsafe fn read_unchecked(token: impl Into<Self::Token>, from: &[Self::Scalar]) -> Self {
        Self::read_ptr(token, from.as_ptr())
    }

    /// Read from a slice.
    ///
    /// # Panic
    /// Panics if the length of `from` is less than `width()`.
    #[inline]
    fn read(token: impl Into<Self::Token>, from: &[Self::Scalar]) -> Self {
        assert!(
            from.len() >= Self::width(),
            "source not larget enough to load vector"
        );
        unsafe { Self::read_unchecked(token, from) }
    }

    /// Write to a pointer.
    ///
    /// # Safety
    /// `from` must point to an array of length at least `width()`
    #[inline]
    unsafe fn write_ptr(self, to: *mut Self::Scalar) {
        (to as *mut Self).write_unaligned(self);
    }

    /// Write to a slice without checking the length.
    ///
    /// # Safety
    /// `from` must be length at least `width()`.
    #[inline]
    unsafe fn write_unchecked(self, to: &mut [Self::Scalar]) {
        self.write_ptr(to.as_mut_ptr());
    }

    /// Write to a slice.
    ///
    /// # Panics
    /// Panics if the length of `from` is less than `width()`.
    #[inline]
    fn write(self, to: &mut [Self::Scalar]) {
        assert!(
            to.len() >= Self::width(),
            "destination not large enough to store vector"
        );
        unsafe { self.write_unchecked(to) };
    }

    /// Create a new vector with each lane containing zeroes.
    #[inline]
    fn zeroed(#[allow(unused_variables)] token: impl Into<Self::Token>) -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Create a new vector with each lane containing the provided value.
    fn splat(token: impl Into<Self::Token>, from: Self::Scalar) -> Self;
}

/// A supertrait for vectors supporting typical arithmetic operations.
pub trait Ops:
    Vector
    + AsRef<[<Self as Vector>::Scalar]>
    + AsMut<[<Self as Vector>::Scalar]>
    + Deref<Target = [<Self as Vector>::Scalar]>
    + DerefMut
    + Add<Self, Output = Self>
    + Add<<Self as Vector>::Scalar, Output = Self>
    + AddAssign<Self>
    + AddAssign<<Self as Vector>::Scalar>
    + Sub<Self, Output = Self>
    + Sub<<Self as Vector>::Scalar, Output = Self>
    + SubAssign<Self>
    + SubAssign<<Self as Vector>::Scalar>
    + Mul<Self, Output = Self>
    + Mul<<Self as Vector>::Scalar, Output = Self>
    + MulAssign<Self>
    + MulAssign<<Self as Vector>::Scalar>
    + Div<Self, Output = Self>
    + Div<<Self as Vector>::Scalar, Output = Self>
    + DivAssign<Self>
    + DivAssign<<Self as Vector>::Scalar>
{
}
impl<V> Ops for V where
    V: Vector
        + AsRef<[<V as Vector>::Scalar]>
        + AsMut<[<V as Vector>::Scalar]>
        + Deref<Target = [<V as Vector>::Scalar]>
        + DerefMut
        + Add<V, Output = V>
        + Add<<V as Vector>::Scalar, Output = V>
        + AddAssign<V>
        + AddAssign<<V as Vector>::Scalar>
        + Sub<V, Output = V>
        + Sub<<V as Vector>::Scalar, Output = V>
        + SubAssign<V>
        + SubAssign<<V as Vector>::Scalar>
        + Mul<V, Output = V>
        + Mul<<V as Vector>::Scalar, Output = V>
        + MulAssign<V>
        + MulAssign<<V as Vector>::Scalar>
        + Div<V, Output = V>
        + Div<<V as Vector>::Scalar, Output = V>
        + DivAssign<V>
        + DivAssign<<V as Vector>::Scalar>
{
}

/// A supertrait for vectors that allow arithmetic operations over signed types.
pub trait Signed: Ops + Neg<Output = Self> {}
impl<V> Signed for V where V: Ops + Neg<Output = V> {}

/// Complex valued vectors.
pub trait Complex: Signed {
    /// The real scalar type.
    type RealScalar: Copy;

    /// Conjugate.
    fn conj(self) -> Self;

    /// Multiply by i.
    fn mul_i(self) -> Self;

    /// Multiply by -i.
    fn mul_neg_i(self) -> Self;
}
