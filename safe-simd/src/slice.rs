//! Slices of vectors.

use crate::vector::VectorCore;
use core::marker::PhantomData;

/// Wrapper for producing a mutable reference from an unaligned pointer.
pub struct RefMut<'a, V>
where
    V: VectorCore,
{
    source: *mut V::Scalar,
    temp: V,
    lifetime: PhantomData<&'a V::Scalar>,
}

impl<'a, V> RefMut<'a, V>
where
    V: VectorCore,
{
    unsafe fn new(source: *mut V::Scalar) -> Self {
        Self {
            source,
            temp: V::zeroed(),
            lifetime: PhantomData,
        }
    }
}

impl<'a, V> core::ops::Deref for RefMut<'a, V>
where
    V: VectorCore,
{
    type Target = V;
    fn deref(&self) -> &V {
        &self.temp
    }
}

impl<'a, V> core::ops::DerefMut for RefMut<'a, V>
where
    V: VectorCore,
{
    fn deref_mut(&mut self) -> &mut V {
        &mut self.temp
    }
}

impl<'a, V> core::ops::Drop for RefMut<'a, V>
where
    V: VectorCore,
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
    V: VectorCore,
{
    slice: &'a [V::Scalar],
    phantom: PhantomData<V>,
}

impl<'a, V> Overlapping<'a, V>
where
    V: VectorCore,
{
    /// Create a new overlapping vector slice.
    ///
    /// Safety: the CPU must support the vector type.
    #[inline]
    pub unsafe fn new(slice: &'a [V::Scalar]) -> Self {
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
    /// Safety: index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V {
        V::read_ptr(self.slice.as_ptr().add(index))
    }
}

/// Wrapper for indexing into overlapping mutable vectors.
pub struct OverlappingMut<'a, V>
where
    V: VectorCore,
{
    slice: &'a mut [V::Scalar],
    phantom: PhantomData<V>,
}

impl<'a, V> OverlappingMut<'a, V>
where
    V: VectorCore,
{
    /// Create a new overlapping vector slice.
    ///
    /// Safety: the CPU must support the vector type.
    #[inline]
    pub unsafe fn new(slice: &'a mut [V::Scalar]) -> Self {
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
    /// Safety: index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V {
        V::read_ptr(self.slice.as_ptr().add(index))
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
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked_mut(&'a mut self, index: usize) -> RefMut<'a, V> {
        RefMut::new(self.slice.as_mut_ptr().add(index))
    }
}
