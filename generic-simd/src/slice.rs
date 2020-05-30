//! Slices of vectors.

use crate::vector::Vector;
use arch_types::{marker::Superset, Features};
use core::marker::PhantomData;

/// Extract a slice of aligned vectors, as if by [`align_to`].
///
/// [`align_to`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to
#[inline]
pub fn align<V>(
    #[allow(unused_variables)] feature: impl Superset<V::Feature>,
    slice: &[V::Scalar],
) -> (&[V::Scalar], &[V], &[V::Scalar])
where
    V: Vector,
{
    unsafe { slice.align_to() }
}

/// Extract a slice of aligned mutable vectors, as if by [`align_to_mut`].
///
/// [`align_to_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut
#[inline]
pub fn align_mut<V>(
    #[allow(unused_variables)] feature: impl Superset<V::Feature>,
    slice: &mut [V::Scalar],
) -> (&mut [V::Scalar], &mut [V], &mut [V::Scalar])
where
    V: Vector,
{
    unsafe { slice.align_to_mut() }
}

/// Create a slice of overlapping vectors from a slice of scalars.
#[inline]
pub fn overlapping<V>(feature: impl Superset<V::Feature>, slice: &[V::Scalar]) -> Overlapping<'_, V>
where
    V: Vector,
{
    Overlapping::new(feature, slice)
}

/// Create a mutable slice of overlapping vectors from a slice of scalars.
#[inline]
pub fn overlapping_mut<V>(
    feature: impl Superset<V::Feature>,
    slice: &mut [V::Scalar],
) -> OverlappingMut<'_, V>
where
    V: Vector,
{
    OverlappingMut::new(feature, slice)
}

/// Wrapper for producing a mutable reference from an unaligned pointer.
pub struct RefMut<'a, V>
where
    V: Vector,
{
    source: *mut V::Scalar,
    temp: V,
    lifetime: PhantomData<&'a V::Scalar>,
}

impl<'a, V> RefMut<'a, V>
where
    V: Vector,
{
    fn new(feature: impl Superset<V::Feature>, source: *mut V::Scalar) -> Self {
        Self {
            source,
            temp: V::zeroed(feature),
            lifetime: PhantomData,
        }
    }
}

impl<'a, V> core::ops::Deref for RefMut<'a, V>
where
    V: Vector,
{
    type Target = V;
    fn deref(&self) -> &V {
        &self.temp
    }
}

impl<'a, V> core::ops::DerefMut for RefMut<'a, V>
where
    V: Vector,
{
    fn deref_mut(&mut self) -> &mut V {
        &mut self.temp
    }
}

impl<'a, V> core::ops::Drop for RefMut<'a, V>
where
    V: Vector,
{
    fn drop(&mut self) {
        unsafe {
            self.temp.write_ptr(self.source);
        }
    }
}

/// Wrapper for indexing into overlapping vectors.
pub struct Overlapping<'a, V>
where
    V: Vector,
{
    slice: &'a [V::Scalar],
    phantom: PhantomData<V>,
}

#[allow(clippy::len_without_is_empty)]
impl<'a, V> Overlapping<'a, V>
where
    V: Vector,
{
    /// Create a new overlapping vector slice.
    #[inline]
    pub fn new(
        #[allow(unused_variables)] feature: impl Superset<V::Feature>,
        slice: &'a [V::Scalar],
    ) -> Self {
        assert!(
            slice.len() >= V::width(),
            "slice must be at least as wide as the vector"
        );
        Self {
            slice,
            phantom: PhantomData,
        }
    }

    /// Returns the number of overlapping vectors.
    ///
    /// Equal to `slice.len() - V::width() + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len() - V::width() + 1
    }

    /// Returns the vector offset `index` into the slice of scalars.
    #[inline]
    pub fn get(&self, index: usize) -> Option<V> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns the vector offset `index` into the slice of scalars.
    ///
    /// # Safety
    /// Index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V
    where
        V: Vector,
    {
        V::read_ptr(V::Feature::new_unchecked(), self.slice.as_ptr().add(index))
    }
}

/// Wrapper for indexing into overlapping mutable vectors.
pub struct OverlappingMut<'a, V>
where
    V: Vector,
{
    slice: &'a mut [V::Scalar],
    phantom: PhantomData<V>,
}

#[allow(clippy::len_without_is_empty)]
impl<'a, V> OverlappingMut<'a, V>
where
    V: Vector,
{
    /// Create a new overlapping vector slice.
    #[inline]
    pub fn new(
        #[allow(unused_variables)] feature: impl Superset<V::Feature>,
        slice: &'a mut [V::Scalar],
    ) -> Self {
        assert!(
            slice.len() >= V::width(),
            "slice must be at least as wide as the vector"
        );
        Self {
            slice,
            phantom: PhantomData,
        }
    }

    /// Returns the number of overlapping vectors.
    ///
    /// Equal to `slice.len() - V::width() + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len() - V::width() + 1
    }

    /// Returns the vector offset `index` into the slice of scalars.
    #[inline]
    pub fn get(&self, index: usize) -> Option<V> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns the vector offset `index` into the slice of scalars.
    ///
    /// # Safety
    /// Index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V {
        V::read_ptr(V::Feature::new_unchecked(), self.slice.as_ptr().add(index))
    }

    /// Returns the mutable vector offset `index` into the slice of scalars.
    #[inline]
    pub fn get_mut(&'a mut self, index: usize) -> Option<RefMut<'a, V>> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }

    /// Returns the mutable vector offset `index` into the slice of scalars.
    ///
    /// # Safety
    /// Index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked_mut(&'a mut self, index: usize) -> RefMut<'a, V> {
        RefMut::new(
            V::Feature::new_unchecked(),
            self.slice.as_mut_ptr().add(index),
        )
    }
}
