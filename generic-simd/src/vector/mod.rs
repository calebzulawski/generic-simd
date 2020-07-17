//! Vector type interfaces.

pub mod pointer;
pub mod scalar;
pub mod slice;
pub mod width;

use crate::arch::Token;
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use scalar::ScalarWidth;

/// Indicates the widest native vector.
pub trait Native<Token> {
    type Width: width::Width;
}

/// Convenience type for the widest native vector size.
pub type NativeWidth<Scalar, Token> = <Scalar as Native<Token>>::Width;

/// Convenience type for the widest native vector.
pub type NativeVector<Scalar, Token> = SizedVector<Scalar, NativeWidth<Scalar, Token>, Token>;

/// Convenience type for the vector with a particular width.
pub type SizedVector<Scalar, Width, Token> = <Scalar as ScalarWidth<Token, Width>>::Vector;

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
        #[allow(unused_variables)] token: Self::Token,
        from: *const Self::Scalar,
    ) -> Self {
        (from as *const Self).read_unaligned()
    }

    /// Read from a slice without checking the length.
    ///
    /// # Safety
    /// * `from` be length at least `width()`.
    #[inline]
    unsafe fn read_unchecked(token: Self::Token, from: &[Self::Scalar]) -> Self {
        Self::read_ptr(token, from.as_ptr())
    }

    /// Read from a slice.
    ///
    /// # Panic
    /// Panics if the length of `from` is less than `width()`.
    #[inline]
    fn read(token: Self::Token, from: &[Self::Scalar]) -> Self {
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
    fn zeroed(#[allow(unused_variables)] token: Self::Token) -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Create a new vector with each lane containing the provided value.
    fn splat(token: Self::Token, from: Self::Scalar) -> Self;
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
