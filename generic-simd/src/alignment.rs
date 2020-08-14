//! Alignment helpers.

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "std"))]
use std::alloc;

use crate::{
    arch,
    vector::{scalar, width, SizedVector},
};

/// A zero sized type with the same alignment as `T`.
pub struct TypeAlignment<T> {
    _t: [T; 0],
}

unsafe impl<T> Alignment for TypeAlignment<T> {}

impl<T: Copy> Copy for TypeAlignment<T> {}

impl<T> Clone for TypeAlignment<T> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<T> Default for TypeAlignment<T> {
    fn default() -> Self {
        Self { _t: [] }
    }
}

impl<T> core::fmt::Debug for TypeAlignment<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad("TypeAlignment")
    }
}

impl<T> core::cmp::PartialEq for TypeAlignment<T> {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> core::cmp::Eq for TypeAlignment<T> {}

impl<T> core::cmp::PartialOrd for TypeAlignment<T> {
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<T> core::cmp::Ord for TypeAlignment<T> {
    #[inline]
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<T> core::hash::Hash for TypeAlignment<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, _: &mut H) {}
}

/// A zero sized type with vector alignment for a particular token and scalar.
#[repr(C)]
pub struct TokenAlignment<Token: arch::Token, Scalar: scalar::Scalar<Token>> {
    _1: TypeAlignment<SizedVector<Scalar, width::W1, Token>>,
    _2: TypeAlignment<SizedVector<Scalar, width::W2, Token>>,
    _4: TypeAlignment<SizedVector<Scalar, width::W4, Token>>,
    _8: TypeAlignment<SizedVector<Scalar, width::W8, Token>>,
}

unsafe impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> Alignment
    for TokenAlignment<Token, Scalar>
{
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> Copy for TokenAlignment<Token, Scalar> {}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> Clone for TokenAlignment<Token, Scalar> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> core::fmt::Debug
    for TokenAlignment<Token, Scalar>
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad("TokenAlignment")
    }
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> Default for TokenAlignment<Token, Scalar> {
    fn default() -> Self {
        Self {
            _1: Default::default(),
            _2: Default::default(),
            _4: Default::default(),
            _8: Default::default(),
        }
    }
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> core::cmp::PartialEq
    for TokenAlignment<Token, Scalar>
{
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> core::cmp::Eq
    for TokenAlignment<Token, Scalar>
{
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> core::cmp::PartialOrd
    for TokenAlignment<Token, Scalar>
{
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> core::cmp::Ord
    for TokenAlignment<Token, Scalar>
{
    #[inline]
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<Token: arch::Token, Scalar: scalar::Scalar<Token>> core::hash::Hash
    for TokenAlignment<Token, Scalar>
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, _: &mut H) {}
}

macro_rules! max_alignment {
    { $first:path, $($rest:path,)* } => {
        /// A zero sized type with maximum possible vector alignment for a particular scalar on
        /// the current architecture.
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C)]
        pub struct MaxAlignment<Scalar: scalar::Scalar<$first> $(+ scalar::Scalar<$rest>)*>(
            TokenAlignment<$first, Scalar>,
            $(TokenAlignment<$rest, Scalar>,)*
        );

        unsafe impl<Scalar: scalar::Scalar<$first> $(+ scalar::Scalar<$rest>)*> Alignment for MaxAlignment<Scalar> {}

        impl<Scalar: scalar::Scalar<$first> $(+ scalar::Scalar<$rest>)*> core::fmt::Debug
            for MaxAlignment<Scalar>
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad("MaxAlignment")
            }
        }

        /// Allocate a boxed slice of scalars with maximum possible vector alignment for a
        /// particular scalar on the current architecture.
        ///
        /// # Panics
        /// Panics if `size` is 0 or memory allocation fails.
        #[cfg(any(feature = "std", feature = "alloc"))]
        pub fn allocate_max_aligned_slice<Scalar: scalar::Scalar<$first> $(+ scalar::Scalar<$rest>)*>(size: usize) -> Box<[Scalar]> {
            unsafe {
                allocate_arbitrary_aligned_slice::<Scalar>(
                    size,
                    core::mem::align_of::<MaxAlignment<Scalar>>(),
                )
            }
        }
    }
}

crate::call_macro_with_tokens! { max_alignment }

/// Indicates that a type represents an alignment.
///
/// # Safety
/// The type must be zero sized.
pub unsafe trait Alignment {}

/// Aligns a value to a particular vector alignment.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Aligned<Alignment: self::Alignment, T> {
    _alignment: Alignment,
    value: T,
}

impl<Alignment: self::Alignment, T> core::ops::Deref for Aligned<Alignment, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<Alignment: self::Alignment, T> core::ops::DerefMut for Aligned<Alignment, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
unsafe fn allocate_arbitrary_aligned_slice<T>(size: usize, align: usize) -> Box<[T]> {
    assert!(size > 0, "size must be nonzero");
    let layout = alloc::Layout::from_size_align(size * core::mem::size_of::<T>(), align).unwrap();
    let ptr = alloc::alloc_zeroed(layout) as *mut T;
    assert!(!ptr.is_null());
    Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, size))
}

/// Allocate a boxed slice of scalars with vector alignment.
///
/// # Panics
/// Panics if `size` is 0 or memory allocation fails.
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn allocate_aligned_slice<Token: arch::Token, Scalar: scalar::Scalar<Token>>(
    size: usize,
) -> Box<[Scalar]> {
    unsafe {
        allocate_arbitrary_aligned_slice::<Scalar>(
            size,
            core::mem::align_of::<TokenAlignment<Token, Scalar>>(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_zst() {
        use crate::arch::generic::Generic;

        // Alignment
        assert_eq!(core::mem::size_of::<TokenAlignment<Generic, f32>>(), 0);
        assert_eq!(core::mem::size_of::<TokenAlignment<Generic, f64>>(), 0);
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::size_of::<TokenAlignment<Generic, num_complex::Complex<f32>>>(),
            0
        );
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::size_of::<TokenAlignment<Generic, num_complex::Complex<f64>>>(),
            0
        );

        // MaxAlignment
        assert_eq!(core::mem::size_of::<MaxAlignment<f32>>(), 0);
        assert_eq!(core::mem::size_of::<MaxAlignment<f64>>(), 0);
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::size_of::<MaxAlignment<num_complex::Complex<f32>>>(),
            0
        );
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::size_of::<MaxAlignment<num_complex::Complex<f64>>>(),
            0
        );
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn check_x86() {
        use crate::arch::x86::{Avx, Sse};

        // SSE
        assert_eq!(core::mem::align_of::<TokenAlignment<Sse, f32>>(), 16);
        assert_eq!(core::mem::align_of::<TokenAlignment<Sse, f64>>(), 16);
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::align_of::<TokenAlignment<Sse, num_complex::Complex<f32>>>(),
            16
        );
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::align_of::<TokenAlignment<Sse, num_complex::Complex<f64>>>(),
            16
        );

        // AVX
        assert_eq!(core::mem::align_of::<TokenAlignment<Avx, f32>>(), 32);
        assert_eq!(core::mem::align_of::<TokenAlignment<Avx, f64>>(), 32);
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::align_of::<TokenAlignment<Avx, num_complex::Complex<f32>>>(),
            32
        );
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::align_of::<TokenAlignment<Avx, num_complex::Complex<f64>>>(),
            32
        );

        // MaxTokenAlignment
        assert_eq!(core::mem::align_of::<MaxAlignment<f32>>(), 32);
        assert_eq!(core::mem::align_of::<MaxAlignment<f64>>(), 32);
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::align_of::<MaxAlignment<num_complex::Complex<f32>>>(),
            32
        );
        #[cfg(feature = "complex")]
        assert_eq!(
            core::mem::align_of::<MaxAlignment<num_complex::Complex<f64>>>(),
            32
        );
    }
}
