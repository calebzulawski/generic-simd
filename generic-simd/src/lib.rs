#![cfg_attr(not(feature = "std"), no_std)]
//! `generic-simd` provides zero-cost abstractions for writing explicit cross-platform SIMD
//! operations.
//!
//! # Supported architectures
//! All architectures are supported via scalar fallbacks, but the following instruction sets are
//! also supported:
//! * SSE3 (x86/x86-64)
//! * AVX (x86/x86-64)
//!
//! The various architecture-specific types are available in the [`arch`](arch/index.html) module.
//!
//! # Abstractions
//! Vector abstractions are provided via the traits in the [`vector`](vector/index.html) module.
//! Generics that use these traits are able to utilize any of the supported instruction sets.
//!
//! The following example performs a vector-accelerated sum of an input slice:
//! ```
//! use generic_simd::{
//!     dispatch,
//!     vector::{Native, NativeVector, NativeWidth, Signed, SizedHandle, Vector},
//! };
//!
//! // This function provides a generic implementation for any instruction set.
//! // Here we use the "native" vector type, i.e. the widest vector directly supported by the
//! // architecture.
//! #[inline]
//! fn sum_impl<H>(handle: H, input: &[f32]) -> f32
//! where
//!     H: Native<f32> + SizedHandle<f32, NativeWidth<f32, H>>,
//!     f32: core::iter::Sum<NativeVector<f32, H>>,
//! {
//!     // Use aligned loads in this example, which may be better on some architectures.
//!     let (start, vectors, end) = handle.align(input);
//!
//!     // Sum across the vector lanes, plus the unaligned portions
//!     vectors.iter().copied().sum::<f32>() + start.iter().chain(end).sum::<f32>()
//! }
//!
//! // This function selects the best instruction set at runtime.
//! // The "dispatch" macro compiles this function for each supported architecture.
//! #[dispatch(handle)]
//! fn sum(input: &[f32]) -> f32 {
//!     sum_impl(handle, input)
//! }
//!
//! fn main() {
//!     assert_eq!(sum(&[1f32; 10]), 10.);
//! }
//! ```
//!
//! # Vector shims
//! Various instruction sets provide vectors with different widths, so shims are provided to
//! create vectors of particular widths regardless of architecture.  These are available in the
//! [`shim`](shim/index.html) module.
//!
//! For example, the following function performs an [Array of Structures of Arrays](https://en.wikipedia.org/wiki/AoS_and_SoA)
//! operation using arrays of 4 `f64`s regardless of instruction set:
//! ```
//! use generic_simd::{dispatch, vector::{Signed, SizedHandle, Vector, width}};
//!
//! // Equivalent to an array of 4 2-dimensional coordinates,
//! // but with a vectorizable memory layout.
//! struct Coordinates {
//!     x: [f64; 4],
//!     y: [f64; 4],
//! }
//!
//! // A generic mean implementation for any instruction set.
//! fn mean_impl<H>(handle: H, input: &[Coordinates]) -> (f64, f64)
//! where
//!     H: SizedHandle<f64, width::W4>,
//!     <H as SizedHandle<f64, width::W4>>::Vector: Signed,
//! {
//!     let mut xsum = handle.zeroed();
//!     let mut ysum = handle.zeroed();
//!
//!     for Coordinates { x, y } in input {
//!         // read the arrays into vectors
//!         xsum += handle.read(x);
//!         ysum += handle.read(y);
//!     }
//!
//!     // sum across the vector lanes
//!     (
//!         xsum.iter().sum::<f64>() / (input.len() * 4) as f64,
//!         ysum.iter().sum::<f64>() / (input.len() * 4) as f64,
//!     )
//! }
//!
//! // Selects the best instruction set at runtime.
//! #[dispatch(handle)]
//! fn mean(input: &[Coordinates]) -> (f64, f64) {
//!     mean_impl(handle, input)
//! }
//! ```

// Re-export for use from macros.
#[doc(hidden)]
pub use multiversion;

pub use generic_simd_macros::dispatch;

#[macro_use]
mod implementation;

pub mod arch;
pub mod shim;
pub mod vector;

pub mod slice;
