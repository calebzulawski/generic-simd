//! Vector type interfaces.

use arch_types::{
    marker::{Identity, Subset, Superset},
    Features,
};
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

macro_rules! handle_impl {
    {
        $width:literal, $feature:ident, $vector:ident, $read_ptr:ident, $read_unchecked:ident, $read:ident, $zeroed:ident, $splat:ident
    } => {
        #[doc = "Feature for creating vectors with "]
        #[doc = $width]
        #[doc = "."]
        type $feature: Features + Identity + Subset<Self>;

        #[doc = "Vector with "]
        #[doc = $width]
        #[doc = "."]
        type $vector: Vector<Scalar = Scalar, Feature = Self::$feature>;

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a pointer."]
        #[inline]
        unsafe fn $read_ptr(self, from: *const Scalar) -> Self::$vector {
            Self::$vector::read_ptr(self, from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice without checking the length."]
        #[inline]
        unsafe fn $read_unchecked(self, from: &[Scalar]) -> Self::$vector {
            Self::$vector::read_unchecked(self, from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice."]
        #[inline]
        fn $read(self, from: &[Scalar]) -> Self::$vector {
            Self::$vector::read(self, from)
        }

        #[doc = "Create a vector with "]
        #[doc = $width]
        #[doc = " set to zero."]
        #[inline]
        fn $zeroed(self) -> Self::$vector {
           Self::$vector::zeroed(self)
        }

        #[doc = "Splat a scalar to "]
        #[doc = $width]
        #[doc = "."]
        #[inline]
        fn $splat(self, scalar: Scalar) -> Self::$vector {
            Self::$vector::splat(self, scalar)
        }
    }
}

pub trait Handle<Scalar>: Features + Identity
where
    Scalar: Copy,
{
    handle_impl! { "the native number of lanes", FeatureNative, VectorNative, read_native_ptr, read_native_unchecked, read_native, zeroed_native, splat_native }
    handle_impl! { "1 lane",  Feature1, Vector1, read1_ptr, read1_unchecked, read1, zeroed1, splat1 }
    handle_impl! { "2 lanes", Feature2, Vector2, read2_ptr, read2_unchecked, read2, zeroed2, splat2 }
    handle_impl! { "4 lanes", Feature4, Vector4, read4_ptr, read4_unchecked, read4, zeroed4, splat4 }
    handle_impl! { "8 lanes", Feature8, Vector8, read8_ptr, read8_unchecked, read8, zeroed8, splat8 }
}

/// Operations using the native vector type for a CPU feature.
pub trait Native<T>: Features + Identity
where
    T: Copy,
{
    /// The widest vector available
    type Vector: Vector<Scalar = T, Feature = Self>;

    /// Splat a scalar to the native vector.
    ///
    /// See [`splat`](trait.Vector.html#tymethod.splat).
    #[inline]
    fn splat_native(self, scalar: T) -> Self::Vector {
        Self::Vector::splat(self, scalar)
    }

    /// Create a zeroed copy of the native vector.
    ///
    /// See [`zeroed`](trait.Vector.html#tymethod.zeroed).
    #[inline]
    fn zeroed_native(self) -> Self::Vector {
        Self::Vector::zeroed(self)
    }

    /// Align a slice of scalars to the native vector.
    ///
    /// See [`align`](../slice/fn.align.html).
    #[inline]
    fn align_native(self, slice: &[T]) -> (&[T], &[Self::Vector], &[T]) {
        crate::slice::align(self, slice)
    }

    /// Align a mutable slice of scalars to the native vector.
    ///
    /// See [`align_mut`](../slice/fn.align_mut.html).
    #[inline]
    fn align_native_mut(self, slice: &mut [T]) -> (&mut [T], &mut [Self::Vector], &mut [T]) {
        crate::slice::align_mut(self, slice)
    }

    /// Create a slice of overlapping vectors from a slice of scalars.
    ///
    /// See [`overlapping`](../slice/fn.overlapping.html).
    #[inline]
    fn overlapping_native(self, slice: &[T]) -> crate::slice::Overlapping<'_, Self::Vector> {
        crate::slice::Overlapping::new(self, slice)
    }

    /// Create a mutable slice of overlapping vectors from a slice of scalars.
    ///
    /// See [`overlapping_mut`](../slice/fn.overlapping_mut.html).
    #[inline]
    fn overlapping_native_mut(
        self,
        slice: &mut [T],
    ) -> crate::slice::OverlappingMut<'_, Self::Vector> {
        crate::slice::OverlappingMut::new(self, slice)
    }
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
    type Feature: Features + Identity;

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
        #[allow(unused_variables)] feature: impl Superset<Self::Feature>,
        from: *const Self::Scalar,
    ) -> Self {
        (from as *const Self).read_unaligned()
    }

    /// Read from a slice without checking the length.
    ///
    /// # Safety
    /// * `from` be length at least `WIDTH`.
    #[inline]
    unsafe fn read_unchecked(feature: impl Superset<Self::Feature>, from: &[Self::Scalar]) -> Self {
        Self::read_ptr(feature, from.as_ptr())
    }

    /// Read from a slice.
    ///
    /// # Panic
    /// Panics if the length of `from` is less than `WIDTH`.
    #[inline]
    fn read(feature: impl Superset<Self::Feature>, from: &[Self::Scalar]) -> Self {
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
    fn zeroed(#[allow(unused_variables)] feature: impl Superset<Self::Feature>) -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Create a new vector with each lane containing the provided value.
    fn splat(feature: impl Superset<Self::Feature>, from: Self::Scalar) -> Self;
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
