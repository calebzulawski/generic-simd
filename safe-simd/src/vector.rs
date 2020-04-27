//! Vector type interfaces.

use crate::slice::{Overlapping, OverlappingMut};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A handle for a CPU feature.
pub trait Feature: Sized + Copy + Clone {
    /// Create a new CPU feature handle, returning None if the feature is not supported by the
    /// processor.
    ///
    /// Requires the `runtime_dispatch` feature.
    fn new() -> Option<Self>;

    /// Create a new CPU feature handle without checking if the feature is supported.
    unsafe fn new_unchecked() -> Self;
}

/// A handle for loading a specific vector type.
///
/// This interface provides the only safe methods of creating vectors.
pub trait Loader<Scalar>: Feature {
    /// The supported vector type.
    type Vector: Vector<Scalar = Scalar>;

    /// Read a vector from a pointer.
    ///
    /// # Safety
    /// `from` must point to an array of length at least `Vector::width()`.
    #[inline]
    unsafe fn read_ptr(&self, from: *const Scalar) -> Self::Vector {
        Self::Vector::read_ptr(from)
    }

    /// Read a vector from a slice without checking the length.
    ///
    /// # Safety
    /// `from` must be length at least `Vector::width()`.
    #[inline]
    unsafe fn read_unchecked<T: AsRef<[Scalar]>>(&self, from: &T) -> Self::Vector {
        Self::Vector::read_unchecked(from)
    }

    /// Read a vector from a slice.
    ///
    /// # Panics
    /// Panics if the length of `from` is less than `Vector::width()`.
    #[inline]
    fn read<T: AsRef<[Scalar]>>(&self, from: &T) -> Self::Vector {
        unsafe { Self::Vector::read(from) }
    }

    /// Create a new vector with each lane containing the provided value.
    #[inline]
    fn splat(&self, from: Scalar) -> Self::Vector {
        unsafe { Self::Vector::splat(from) }
    }

    /// Create a new vector with each lane zeroed.
    #[inline]
    fn zeroed(&self) -> Self::Vector {
        unsafe { Self::Vector::zeroed() }
    }

    /// Extract a slice of aligned vectors, as if by [`align_to`].
    ///
    /// [`align_to`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to
    #[inline]
    fn align<'a>(&self, slice: &'a [Scalar]) -> (&'a [Scalar], &'a [Self::Vector], &'a [Scalar]) {
        unsafe { slice.align_to() }
    }

    /// Extract a slice of aligned mutable vectors, as if by [`align_to_mut`].
    ///
    /// [`align_to_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut
    #[inline]
    fn align_mut<'a>(
        &self,
        slice: &'a mut [Scalar],
    ) -> (&'a mut [Scalar], &'a mut [Self::Vector], &'a mut [Scalar]) {
        unsafe { slice.align_to_mut() }
    }

    /// Create a slice of overlapping vectors from a slice of scalars.
    #[inline]
    fn overlapping<'a>(&'a self, slice: &'a [Scalar]) -> Overlapping<'a, Self::Vector> {
        unsafe { Overlapping::new(slice) }
    }

    /// Create a mutable slice of overlapping vectors from a slice of scalars.
    #[inline]
    fn overlapping_mut<'a>(&'a self, slice: &'a mut [Scalar]) -> OverlappingMut<'a, Self::Vector> {
        unsafe { OverlappingMut::new(slice) }
    }
}

/// The fundamental vector type.
///
/// # Safety
/// This trait may only be implemented for types that have the memory layout of an array of
/// `Scalar` with length `width()`.
pub unsafe trait VectorCore: Sized + Copy + Clone {
    /// The type of elements in the vector.
    type Scalar;

    /// Returns the number of elements in the vector.
    #[inline]
    fn width() -> usize {
        core::mem::size_of::<Self>() / core::mem::size_of::<Self::Scalar>()
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
    /// * The CPU feature must be supported.
    /// * `from` must point to an array of length at least `width()`.
    #[inline]
    unsafe fn read_ptr(from: *const Self::Scalar) -> Self {
        core::mem::transmute::<*const Self::Scalar, *const Self>(from).read_unaligned()
    }

    /// Read from a slice without checking the length.
    ///
    /// # Safety
    /// * The CPU feature must be supported.
    /// * `from` be length at least `width()`.
    #[inline]
    unsafe fn read_unchecked<T: AsRef<[Self::Scalar]>>(from: T) -> Self {
        Self::read_ptr(from.as_ref().as_ptr())
    }

    /// Read from a slice.
    ///
    /// # Panic
    /// Panics if the length of `from` is less than `width()`.
    ///
    /// # Safety
    /// The CPU feature must be supported.
    #[inline]
    unsafe fn read<T: AsRef<[Self::Scalar]>>(from: T) -> Self {
        assert!(
            from.as_ref().len() >= Self::width(),
            "source not larget enough to load vector"
        );
        Self::read_unchecked(from)
    }

    /// Write to a pointer.
    ///
    /// # Safety
    /// `from` must point to an array of length at least `width()`
    #[inline]
    unsafe fn write_ptr(self, to: *mut Self::Scalar) {
        core::mem::transmute::<*mut Self::Scalar, *mut Self>(to).write_unaligned(self);
    }

    /// Write to a slice without checking the length.
    ///
    /// # Safety
    /// `from` must be length at least `width()`.
    #[inline]
    unsafe fn write_unchecked<T: AsMut<[Self::Scalar]>>(self, to: &mut T) {
        self.write_ptr(to.as_mut().as_mut_ptr());
    }

    /// Write to a slice.
    ///
    /// # Panics
    /// Panics if the length of `from` is less than `width()`.
    #[inline]
    fn write<T: AsMut<[Self::Scalar]>>(self, to: &mut T) {
        assert!(
            to.as_mut().len() >= Self::width(),
            "destination not large enough to store vector"
        );
        unsafe { self.write_unchecked(to) };
    }

    /// Create a new vector with each lane containing zeroes.
    ///
    /// # Safety
    /// The CPU feature must be supported.
    #[inline]
    unsafe fn zeroed() -> Self {
        core::mem::zeroed()
    }

    /// Create a new vector with each lane containing the provided value.
    ///
    /// # Safety
    /// The CPU feature must be supported.
    unsafe fn splat(from: Self::Scalar) -> Self;
}

/// A supertrait for vectors that allow arithmetic operations.
pub trait Vector:
    VectorCore
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Neg<Output = Self>
{
}
impl<S, V> Vector for V where
    V: VectorCore<Scalar = S>
        + Add<Self, Output = Self>
        + AddAssign<Self>
        + Sub<Self, Output = Self>
        + SubAssign<Self>
        + Mul<Self, Output = Self>
        + MulAssign<Self>
        + Div<Self, Output = Self>
        + DivAssign<Self>
        + Neg<Output = Self>
{
}

/// Complex valued vectors.
pub trait Complex<Real>: Vector<Scalar = num_complex::Complex<Real>> {
    /// Multiply by i.
    fn mul_i(self) -> Self;

    /// Multiply by -i.
    fn mul_neg_i(self) -> Self;
}
