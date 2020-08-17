//! Alignment helpers.

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{
    alloc::{alloc, Layout},
    boxed::Box,
};

#[cfg(feature = "std")]
use std::alloc::{alloc, Layout};

use crate::{
    arch, scalar,
    vector::{width, SizedVector},
};

#[repr(C)]
#[derive(Copy, Clone)]
struct Vectors<Token: arch::Token, Scalar: scalar::ScalarExt<Token>>(
    SizedVector<Scalar, width::W1, Token>,
    SizedVector<Scalar, width::W2, Token>,
    SizedVector<Scalar, width::W4, Token>,
    SizedVector<Scalar, width::W8, Token>,
);

macro_rules! max_alignment {
    { $first:path, $($rest:path,)* } => {

        #[doc(hidden)]
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct AllVectors<Scalar: scalar::ScalarExt<$first> $(+ scalar::ScalarExt<$rest>)*>(
            Vectors<$first, Scalar>,
            $(
            Vectors<$rest, Scalar>,
            )*
        );

        /// Allocate a boxed slice of scalars with maximum possible vector alignment for a
        /// particular scalar on the current architecture.
        ///
        /// # Panics
        /// Panics if `count` is 0 or memory allocation fails.
        #[cfg(any(feature = "std", feature = "alloc"))]
        pub fn allocate_max_aligned_slice<Scalar: Default + scalar::ScalarExt<$first> $(+ scalar::ScalarExt<$rest>)*>(count: usize) -> Box<[Scalar]> {
            allocate_aligned_slice::<AllVectors<Scalar>, Scalar>(count)
        }
    }
}

crate::call_macro_with_tokens! { max_alignment }

/// Aligns a value to another type's alignment.
#[repr(C)]
pub struct Aligned<AlignTo, T> {
    alignment: [AlignTo; 0],
    value: T,
}

impl<AlignTo, T> Aligned<AlignTo, T> {
    pub fn new(value: T) -> Self {
        Self {
            alignment: [],
            value,
        }
    }
}

impl<AlignTo, T> core::ops::Deref for Aligned<AlignTo, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<AlignTo, T> core::ops::DerefMut for Aligned<AlignTo, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<AlignTo: Copy, T: Copy> Copy for Aligned<AlignTo, T> {}

impl<AlignTo, T: Clone> Clone for Aligned<AlignTo, T> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<AlignTo, T: Default> Default for Aligned<AlignTo, T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<AlignTo, T: core::fmt::Debug> core::fmt::Debug for Aligned<AlignTo, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Aligned").field(&self.value).finish()
    }
}

impl<AlignTo, T: core::cmp::PartialEq> core::cmp::PartialEq for Aligned<AlignTo, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<AlignTo, T: core::cmp::Eq> core::cmp::Eq for Aligned<AlignTo, T> {}

impl<AlignTo, T: core::cmp::PartialOrd> core::cmp::PartialOrd for Aligned<AlignTo, T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<AlignTo, T: core::cmp::Ord> core::cmp::Ord for Aligned<AlignTo, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<AlignTo, T: core::hash::Hash> core::hash::Hash for Aligned<AlignTo, T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
        self.value.hash(hasher)
    }
}

/// Allocate a boxed slice of `count` `T`s aligned to the `AlignTo`.
///
/// # Panics
/// Panics if `count` is 0 or memory allocation fails.
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn allocate_aligned_slice<AlignTo, T: Default>(count: usize) -> Box<[T]> {
    assert!(count > 0, "size must be nonzero");
    let layout = Layout::from_size_align(
        count * core::mem::size_of::<T>(),
        core::cmp::max(core::mem::align_of::<AlignTo>(), core::mem::align_of::<T>()),
    )
    .unwrap();
    unsafe {
        let ptr = alloc(layout) as *mut T;
        assert!(!ptr.is_null());
        for i in 0..count {
            ptr.add(i).write(T::default());
        }
        Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, count))
    }
}

/// Aligns a type to the maximum possible vector alignment for a particular scalar on the current
/// architecture.
pub type MaxAligned<Scalar, T> = Aligned<AllVectors<Scalar>, T>;

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn check_x86() {
        type Foo = [f32; 8];
        type AlignedFoo = MaxAligned<f32, Foo>;
        assert_eq!(core::mem::align_of::<AlignedFoo>(), 32);
    }
}
