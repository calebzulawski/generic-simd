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
        $width:literal,
        $feature:ident,
        $vector:ident,
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
        #[doc = " from a pointer.\n\nSee [`read_ptr`](trait.Vector.html#method.read_ptr)."]
        #[inline]
        unsafe fn $read_ptr(self, from: *const Scalar) -> Self::$vector {
            Self::$vector::read_ptr(self, from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice without checking the length.\n\nSee [`read_unchecked`](trait.Vector.html#method.read_ptr)."]
        #[inline]
        unsafe fn $read_unchecked(self, from: &[Scalar]) -> Self::$vector {
            Self::$vector::read_unchecked(self, from)
        }

        #[doc = "Read a vector with "]
        #[doc = $width]
        #[doc = " from a slice.\n\nSee [`read`](trait.Vector.html#method.read)."]
        #[inline]
        fn $read(self, from: &[Scalar]) -> Self::$vector {
            Self::$vector::read(self, from)
        }

        #[doc = "Create a vector with "]
        #[doc = $width]
        #[doc = " set to zero.\n\nSee [`zeroed`](trait.Vector.html#method.zeroed)."]
        #[inline]
        fn $zeroed(self) -> Self::$vector {
           Self::$vector::zeroed(self)
        }

        #[doc = "Splat a scalar to "]
        #[doc = $width]
        #[doc = ".\n\nSee [`splat`](trait.Vector.html#tymethod.splat)."]
        #[inline]
        fn $splat(self, scalar: Scalar) -> Self::$vector {
            Self::$vector::splat(self, scalar)
        }

        #[doc = "Align a slice of scalars to vectors with "]
        #[doc = $width]
        #[doc = ".\n\nSee [`align`](../slice/fn.align.html)."]
        #[inline]
        fn $align(self, slice: &[Scalar]) -> (&[Scalar], &[Self::$vector], &[Scalar]) {
            let shrank: Self::$feature = self.shrink().unwrap(); // coerce type
            crate::slice::align(shrank, slice)
        }

        #[doc = "Align a slice of scalars to vectors with "]
        #[doc = $width]
        #[doc = ".\n\nSee [`align`](../slice/fn.align.html)."]
        #[inline]
        fn $align_mut(self, slice: &mut [Scalar]) -> (&mut [Scalar], &mut [Self::$vector], &mut [Scalar]) {
            let shrank: Self::$feature = self.shrink().unwrap(); // coerce type
            crate::slice::align_mut(shrank, slice)
        }

        #[doc = "Create a slice of overlapping vectors of "]
        #[doc = $width]
        #[doc = "from a slice of scalars.\n\nSee [`overlapping`](../slice/fn.overlapping.html)."]
        #[inline]
        fn $overlapping(self, slice: &[Scalar]) -> crate::slice::Overlapping<'_, Self::$vector> {
            let shrank: Self::$feature = self.shrink().unwrap(); // coerce type
            crate::slice::Overlapping::new(shrank, slice)
        }

        #[doc = "Create a mutable slice of overlapping vectors of "]
        #[doc = $width]
        #[doc = "from a slice of scalars.\n\nSee [`overlapping_mut`](../slice/fn.overlapping_mut.html)."]
        #[inline]
        fn $overlapping_mut(
            self,
            slice: &mut [Scalar],
        ) -> crate::slice::OverlappingMut<'_, Self::$vector> {
            let shrank: Self::$feature = self.shrink().unwrap(); // coerce type
            crate::slice::OverlappingMut::new(shrank, slice)
        }
    }
}

/// Indicates the fastest feature sets for a feature set.
pub trait Handle<Scalar>: Features + Identity
where
    Scalar: Copy,
{
    /// The native vector type.
    type VectorNative: Vector<Scalar = Scalar, Feature = Self>;

    /// Read the native vector from a pointer.
    ///
    /// See [`read_ptr`](trait.Vector.html#method.read_ptr).
    #[inline]
    unsafe fn read_native_ptr(self, from: *const Scalar) -> Self::VectorNative {
        Self::VectorNative::read_ptr(self, from)
    }

    /// Read the native vector from a slice without checking the length.
    ///
    /// See [`read_unchecked`](trait.Vector.html#method.read_unchecked).
    #[inline]
    unsafe fn read_native_unchecked(self, from: &[Scalar]) -> Self::VectorNative {
        Self::VectorNative::read_unchecked(self, from)
    }

    /// Read the native vector from a slice.
    ///
    /// See [`read`](trait.Vector.html#method.read).
    #[inline]
    fn read_native(self, from: &[Scalar]) -> Self::VectorNative {
        Self::VectorNative::read(self, from)
    }

    /// Create a native vector set to zero.
    ///
    /// See [`zeroed`](trait.Vector.html#method.zeroed).
    #[inline]
    fn zeroed_native(self) -> Self::VectorNative {
        Self::VectorNative::zeroed(self)
    }

    /// Splat a scalar to a native vector.
    ///
    /// See [`splat`](trait.Vector.html#tymethod.splat).
    #[inline]
    fn splat_native(self, scalar: Scalar) -> Self::VectorNative {
        Self::VectorNative::splat(self, scalar)
    }

    /// Align a slice of scalars to native vectors.
    ///
    /// See [`align`](../slice/fn.align.html).
    #[inline]
    fn align_native(self, slice: &[Scalar]) -> (&[Scalar], &[Self::VectorNative], &[Scalar]) {
        crate::slice::align(self, slice)
    }

    /// Align a slice of scalars to native vectors.
    ///
    /// See [`align`](../slice/fn.align.html).
    #[inline]
    fn align_native_mut(
        self,
        slice: &mut [Scalar],
    ) -> (&mut [Scalar], &mut [Self::VectorNative], &mut [Scalar]) {
        crate::slice::align_mut(self, slice)
    }

    /// Create a slice of overlapping native vectors from a slice of scalars.
    ///
    ///
    /// See [`overlapping`](../slice/fn.overlapping.html).
    #[inline]
    fn overlapping_native(
        self,
        slice: &[Scalar],
    ) -> crate::slice::Overlapping<'_, Self::VectorNative> {
        crate::slice::Overlapping::new(self, slice)
    }

    /// Create a mutable slice of overlapping native vectors from a slice of scalars.
    ///
    ///
    /// See [`overlapping_mut`](../slice/fn.overlapping_mut.html).
    #[inline]
    fn overlapping_native_mut(
        self,
        slice: &mut [Scalar],
    ) -> crate::slice::OverlappingMut<'_, Self::VectorNative> {
        crate::slice::OverlappingMut::new(self, slice)
    }

    handle_impl! { "1 lane",  Feature1, Vector1, read1_ptr, read1_unchecked, read1, zeroed1, splat1, align1, align1_mut, overlapping1, overlapping1_mut }
    handle_impl! { "2 lanes", Feature2, Vector2, read2_ptr, read2_unchecked, read2, zeroed2, splat2, align2, align2_mut, overlapping2, overlapping2_mut }
    handle_impl! { "4 lanes", Feature4, Vector4, read4_ptr, read4_unchecked, read4, zeroed4, splat4, align4, align4_mut, overlapping4, overlapping4_mut }
    handle_impl! { "8 lanes", Feature8, Vector8, read8_ptr, read8_unchecked, read8, zeroed8, splat8, align8, align8_mut, overlapping8, overlapping8_mut }
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

/// A supertrait for vectors that allow arithmetic operations.
pub trait Signed: Ops + Neg<Output = Self> {}
impl<V> Signed for V where V: Ops + Neg<Output = V> {}

/// Complex valued vectors.
pub trait Complex: Signed {
    /// The real scalar type.
    type RealScalar: Copy;

    /// Multiply by i.
    fn mul_i(self) -> Self;

    /// Multiply by -i.
    fn mul_neg_i(self) -> Self;
}
