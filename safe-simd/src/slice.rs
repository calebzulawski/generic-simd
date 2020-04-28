//! Slices of vectors.

use crate::vector::{Feature, ProvidedBy, Vector};
use core::marker::PhantomData;

/// Extract a slice of aligned vectors, as if by [`align_to`].
///
/// [`align_to`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to
#[inline]
pub fn align<V, F>(_: F, slice: &[V::Scalar]) -> (&[V::Scalar], &[V], &[V::Scalar])
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
{
    unsafe { slice.align_to() }
}

/// Extract a slice of aligned mutable vectors, as if by [`align_to_mut`].
///
/// [`align_to_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut
#[inline]
fn align_mut<V, F>(_: F, slice: &mut [V::Scalar]) -> (&mut [V::Scalar], &mut [V], &mut [V::Scalar])
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
{
    unsafe { slice.align_to_mut() }
}

/// Create a slice of overlapping vectors from a slice of scalars.
#[inline]
fn overlapping<'a, V, F>(feature: F, slice: &'a [V::Scalar]) -> Overlapping<'a, V, F>
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
{
    Overlapping::new(feature, slice)
}

/// Create a mutable slice of overlapping vectors from a slice of scalars.
#[inline]
fn overlapping_mut<'a, V, F>(feature: F, slice: &'a mut [V::Scalar]) -> OverlappingMut<'a, V, F>
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
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
    fn new<F>(feature: F, source: *mut V::Scalar) -> Self
    where
        F: Feature,
        V: ProvidedBy<F>,
    {
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
pub struct Overlapping<'a, V, F>
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
{
    feature: F,
    slice: &'a [V::Scalar],
    phantom: PhantomData<V>,
}

impl<'a, V, F> Overlapping<'a, V, F>
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
{
    /// Create a new overlapping vector slice.
    #[inline]
    pub fn new(feature: F, slice: &'a [V::Scalar]) -> Self {
        assert!(
            slice.len() >= V::WIDTH,
            "slice must be at least as wide as the vector"
        );
        Self {
            feature,
            slice,
            phantom: PhantomData,
        }
    }

    /// Returns the number of overlapping vectors.
    ///
    /// Equal to `slice.len() - V::WIDTH + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len() - V::WIDTH + 1
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
    /// Safety: index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::WIDTH` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V
    where
        V: Vector,
    {
        V::read_ptr(self.feature, self.slice.as_ptr().add(index))
    }
}

/// Wrapper for indexing into overlapping mutable vectors.
pub struct OverlappingMut<'a, V, F>
where
    V: Vector,
    F: Feature,
{
    feature: F,
    slice: &'a mut [V::Scalar],
    phantom: PhantomData<V>,
}

impl<'a, V, F> OverlappingMut<'a, V, F>
where
    F: Feature,
    V: Vector + ProvidedBy<F>,
{
    /// Create a new overlapping vector slice.
    #[inline]
    pub fn new(feature: F, slice: &'a mut [V::Scalar]) -> Self {
        assert!(
            slice.len() >= V::WIDTH,
            "slice must be at least as wide as the vector"
        );
        Self {
            feature,
            slice,
            phantom: PhantomData,
        }
    }

    /// Returns the number of overlapping vectors.
    ///
    /// Equal to `slice.len() - V::WIDTH + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len() - V::WIDTH + 1
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
    /// Safety: index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::WIDTH` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V {
        V::read_ptr(self.feature, self.slice.as_ptr().add(index))
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
    /// Safety: index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::WIDTH` long.
    #[inline]
    pub unsafe fn get_unchecked_mut(&'a mut self, index: usize) -> RefMut<'a, V> {
        RefMut::new(self.feature, self.slice.as_mut_ptr().add(index))
    }
}
