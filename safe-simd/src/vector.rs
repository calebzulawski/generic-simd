//! Vector type interfaces.

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

/// Marker indicating features that provide a vector.
pub unsafe trait ProvidedBy<F>: Vector
where
    F: Feature,
{
}

/// The widest vector available.
pub trait Widest<T>: Feature {
    type Widest: Vector<Scalar = T> + ProvidedBy<Self>;
}

/// The fundamental vector type.
///
/// # Safety
/// This trait may only be implemented for types that have the memory layout of an array of
/// `Scalar` with length `WIDTH`.
pub unsafe trait Vector: Sized + Copy + Clone {
    /// The type of elements in the vector.
    type Scalar;

    /// The number of elements in the vector.
    const WIDTH: usize = core::mem::size_of::<Self>() / core::mem::size_of::<Self::Scalar>();

    /// Returns a slice containing the vector.
    #[inline]
    fn as_slice(&self) -> &[Self::Scalar] {
        unsafe { core::slice::from_raw_parts(self as *const _ as *const _, Self::WIDTH) }
    }

    /// Returns a mutable slice containing the vector.
    #[inline]
    fn as_slice_mut(&mut self) -> &mut [Self::Scalar] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as *mut _, Self::WIDTH) }
    }

    /// Read from a pointer.
    ///
    /// # Safety
    /// * `from` must point to an array of length at least `WIDTH`.
    #[inline]
    unsafe fn read_ptr<F>(_: F, from: *const Self::Scalar) -> Self
    where
        F: Feature,
        Self: ProvidedBy<F>,
    {
        core::mem::transmute::<*const Self::Scalar, *const Self>(from).read_unaligned()
    }

    /// Read from a slice without checking the length.
    ///
    /// # Safety
    /// * `from` be length at least `WIDTH`.
    #[inline]
    unsafe fn read_unchecked<T, F>(feature: F, from: T) -> Self
    where
        T: AsRef<[Self::Scalar]>,
        F: Feature,
        Self: ProvidedBy<F>,
    {
        Self::read_ptr(feature, from.as_ref().as_ptr())
    }

    /// Read from a slice.
    ///
    /// # Panic
    /// Panics if the length of `from` is less than `WIDTH`.
    #[inline]
    fn read<T, F>(feature: F, from: T) -> Self
    where
        T: AsRef<[Self::Scalar]>,
        F: Feature,
        Self: ProvidedBy<F>,
    {
        assert!(
            from.as_ref().len() >= Self::WIDTH,
            "source not larget enough to load vector"
        );
        unsafe { Self::read_unchecked(feature, from) }
    }

    /// Write to a pointer.
    ///
    /// # Safety
    /// `from` must point to an array of length at least `WIDTH`
    #[inline]
    unsafe fn write_ptr(self, to: *mut Self::Scalar) {
        core::mem::transmute::<*mut Self::Scalar, *mut Self>(to).write_unaligned(self);
    }

    /// Write to a slice without checking the length.
    ///
    /// # Safety
    /// `from` must be length at least `WIDTH`.
    #[inline]
    unsafe fn write_unchecked<T: AsMut<[Self::Scalar]>>(self, to: &mut T) {
        self.write_ptr(to.as_mut().as_mut_ptr());
    }

    /// Write to a slice.
    ///
    /// # Panics
    /// Panics if the length of `from` is less than `WIDTH`.
    #[inline]
    fn write<T: AsMut<[Self::Scalar]>>(self, to: &mut T) {
        assert!(
            to.as_mut().len() >= Self::WIDTH,
            "destination not large enough to store vector"
        );
        unsafe { self.write_unchecked(to) };
    }

    /// Create a new vector with each lane containing zeroes.
    #[inline]
    fn zeroed<F>(_: F) -> Self
    where
        F: Feature,
        Self: ProvidedBy<F>,
    {
        unsafe { core::mem::zeroed() }
    }

    /// Create a new vector with each lane containing the provided value.
    fn splat<F>(_: F, from: Self::Scalar) -> Self
    where
        F: Feature,
        Self: ProvidedBy<F>;
}

/// A supertrait for vectors that allow arithmetic operations.
pub trait Float:
    Vector
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
impl<S, V> Float for V where
    V: Vector<Scalar = S>
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
pub trait Complex<Real>: Float<Scalar = num_complex::Complex<Real>> {
    /// Multiply by i.
    fn mul_i(self) -> Self;

    /// Multiply by -i.
    fn mul_neg_i(self) -> Self;
}
