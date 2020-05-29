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
//! use generic_simd::{dispatch, vector::{Handle, Signed, Vector}};
//!
//! // This function provides a generic implementation for any vector type.
//! #[inline]
//! fn sum_impl<H>(handle: H, input: &[f32]) -> f32
//! where
//!     H: Handle<f32>,
//!     <H as Handle<f32>>::VectorNative: Signed,
//! {
//!     // Use aligned loads in this example, which may be better on some architectures.
//!     // Here we use the "native" vector type, i.e. the widest vector directly supported by the
//!     // architecture.
//!     let (start, vectors, end) = handle.align_native(input);
//!
//!     // Sum each vector lane
//!     let mut sums = handle.zeroed_native();
//!     for v in vectors {
//!         sums += *v;
//!     }
//!
//!     // Sum across the vector lanes, plus the unaligned portions
//!     sums.iter().chain(start).chain(end).sum::<f32>()
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
//! use generic_simd::{dispatch, vector::{Handle, Signed, Vector}};
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
//!     H: Handle<f64>,
//!     <H as Handle<f64>>::Vector4: Signed,
//! {
//!     let mut xsum = handle.zeroed4();
//!     let mut ysum = handle.zeroed4();
//!
//!     for Coordinates { x, y } in input {
//!         // read the arrays into vectors
//!         xsum += handle.read4(x);
//!         ysum += handle.read4(y);
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
pub use arch_types;

pub use generic_simd_macros::dispatch;

#[macro_use]
mod implementation;

pub mod arch;
pub mod shim;
pub mod vector;

pub mod slice;
