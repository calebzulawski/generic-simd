//! Vector type interfaces.

use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// Detects a CPU feature.
pub trait FeatureDetect: Copy {
    /// Detect support of this CPU feature.
    fn detect() -> Option<Self>;

    /// Create a new CPU feature handle without checking if the feature is supported.
    ///
    /// # Safety
    /// This feature must be supported by the CPU.
    unsafe fn new() -> Self;
}

/// Operations using the widest vector available to a CPU feature.
pub trait Widest<T>: FeatureDetect
where
    T: Copy,
{
    /// The widest vector available
    type Widest: Vector<Scalar = T, Feature = Self>;

    /// Splat a scalar to the widest vector.
    ///
    /// See [`splat`](trait.Vector.html#tymethod.splat).
    #[inline]
    fn splat_widest(self, scalar: T) -> Self::Widest {
        Self::Widest::splat(self, scalar)
    }

    /// Create a zeroed copy of the widest vector.
    ///
    /// See [`zeroed`](trait.Vector.html#tymethod.zeroed).
    #[inline]
    fn zeroed_widest(self) -> Self::Widest {
        Self::Widest::zeroed(self)
    }

    /// Align a slice of scalars to the widest vector.
    ///
    /// See [`align`](../slice/fn.align.html).
    #[inline]
    fn align_widest(self, slice: &[T]) -> (&[T], &[Self::Widest], &[T]) {
        crate::slice::align(self, slice)
    }

    /// Align a mutable slice of scalars to the widest vector.
    ///
    /// See [`align_mut`](../slice/fn.align_mut.html).
    #[inline]
    fn align_widest_mut(self, slice: &mut [T]) -> (&mut [T], &mut [Self::Widest], &mut [T]) {
        crate::slice::align_mut(self, slice)
    }

    /// Create a slice of overlapping vectors from a slice of scalars.
    ///
    /// See [`overlapping`](../slice/fn.overlapping.html).
    #[inline]
    fn overlapping_widest(self, slice: &[T]) -> crate::slice::Overlapping<'_, Self::Widest> {
        crate::slice::Overlapping::new(self, slice)
    }

    /// Create a mutable slice of overlapping vectors from a slice of scalars.
    ///
    /// See [`overlapping_mut`](../slice/fn.overlapping_mut.html).
    #[inline]
    fn overlapping_widest_mut(
        self,
        slice: &mut [T],
    ) -> crate::slice::OverlappingMut<'_, Self::Widest> {
        crate::slice::OverlappingMut::new(self, slice)
    }
}

pub trait Feature:
    Widest<f32> + Widest<f64> + Widest<num_complex::Complex<f32>> + Widest<num_complex::Complex<f64>>
{
}

/// The fundamental vector type.
///
/// # Safety
/// This trait may only be implemented for types that have the memory layout of an array of
/// `Scalar` with length `WIDTH`.
pub unsafe trait Vector: Copy {
    /// The type of elements in the vector.
    type Scalar: Copy;

    /// The feature required to use this vector type.
    type Feature: FeatureDetect;

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
    unsafe fn read_ptr(
        #[allow(unused_variables)] feature: Self::Feature,
        from: *const Self::Scalar,
    ) -> Self {
        (from as *const Self).read_unaligned()
    }

    /// Read from a slice without checking the length.
    ///
    /// # Safety
    /// * `from` be length at least `WIDTH`.
    #[inline]
    unsafe fn read_unchecked(feature: Self::Feature, from: &[Self::Scalar]) -> Self {
        Self::read_ptr(feature, from.as_ptr())
    }

    /// Read from a slice.
    ///
    /// # Panic
    /// Panics if the length of `from` is less than `WIDTH`.
    #[inline]
    fn read(feature: Self::Feature, from: &[Self::Scalar]) -> Self {
        assert!(
            from.len() >= Self::WIDTH,
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
        (to as *mut Self).write_unaligned(self);
    }

    /// Write to a slice without checking the length.
    ///
    /// # Safety
    /// `from` must be length at least `WIDTH`.
    #[inline]
    unsafe fn write_unchecked(self, to: &mut [Self::Scalar]) {
        self.write_ptr(to.as_mut_ptr());
    }

    /// Write to a slice.
    ///
    /// # Panics
    /// Panics if the length of `from` is less than `WIDTH`.
    #[inline]
    fn write(self, to: &mut [Self::Scalar]) {
        assert!(
            to.len() >= Self::WIDTH,
            "destination not large enough to store vector"
        );
        unsafe { self.write_unchecked(to) };
    }

    /// Create a new vector with each lane containing zeroes.
    #[inline]
    fn zeroed(#[allow(unused_variables)] feature: Self::Feature) -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Create a new vector with each lane containing the provided value.
    fn splat(feature: Self::Feature, from: Self::Scalar) -> Self;
}

/// A supertrait for vectors supporting typical operations.
pub trait Ops<T>:
    Vector
    + AsRef<[T]>
    + AsMut<[T]>
    + Deref<Target = [T]>
    + DerefMut
    + Add<Self, Output = Self>
    + Add<T, Output = Self>
    + AddAssign<Self>
    + AddAssign<T>
    + Sub<Self, Output = Self>
    + Sub<T, Output = Self>
    + SubAssign<Self>
    + SubAssign<T>
    + Mul<Self, Output = Self>
    + Mul<T, Output = Self>
    + MulAssign<Self>
    + MulAssign<T>
    + Div<Self, Output = Self>
    + Div<T, Output = Self>
    + DivAssign<Self>
    + DivAssign<T>
{
}
impl<T, V> Ops<T> for V
where
    T: Copy,
    V: Vector<Scalar = T>
        + AsRef<[T]>
        + AsMut<[T]>
        + Deref<Target = [T]>
        + DerefMut
        + Add<V, Output = V>
        + Add<T, Output = V>
        + AddAssign<V>
        + AddAssign<T>
        + Sub<V, Output = V>
        + Sub<T, Output = V>
        + SubAssign<V>
        + SubAssign<T>
        + Mul<V, Output = V>
        + Mul<T, Output = V>
        + MulAssign<V>
        + MulAssign<T>
        + Div<V, Output = V>
        + Div<T, Output = V>
        + DivAssign<V>
        + DivAssign<T>,
{
}

/// A supertrait for vectors that allow arithmetic operations.
pub trait Signed<T>: Ops<T> + Neg<Output = Self> {}
impl<T, V> Signed<T> for V
where
    T: Copy,
    V: Ops<T> + Neg<Output = V>,
{
}

/// Complex valued vectors.
pub trait Complex<Real>: Signed<num_complex::Complex<Real>>
where
    Real: Copy,
{
    /// Multiply by i.
    fn mul_i(self) -> Self;

    /// Multiply by -i.
    fn mul_neg_i(self) -> Self;
}
