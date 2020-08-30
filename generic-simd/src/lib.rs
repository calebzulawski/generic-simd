#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(feature = "nightly", target_arch = "wasm32"),
    feature(wasm_simd, wasm_target_feature)
)]
#![cfg_attr(
    all(feature = "nightly", target_arch = "aarch64"),
    feature(stdsimd, aarch64_target_feature)
)]
#![cfg_attr(
    all(
        feature = "nightly",
        target_arch = "arm",
        target_feature = "v7",
        target_feature = "neon"
    ),
    feature(stdsimd, arm_target_feature)
)]
//! `generic-simd` provides zero-cost abstractions for writing explicit cross-platform SIMD
//! operations.
//!
//! # Supported architectures
//! All architectures are supported via scalar fallbacks, but the following instruction sets are
//! also supported:
//! * SSE4.1 (x86/x86-64)
//! * AVX (x86/x86-64)
//! * NEON (arm/aarch64, with `nightly` cargo feature, and `v7` and `neon` target features on arm)
//! * SIMD128 (wasm32, with `nightly` cargo feature and `simd128` target feature)
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
//!     arch::Token,
//!     dispatch,
//!     scalar::ScalarExt,
//!     slice::SliceExt,
//!     vector::NativeVector,
//! };
//!
//! // This function provides a generic implementation for any instruction set.
//! // Here we use the "native" vector type, i.e. the widest vector directly supported by the
//! // architecture.
//! #[inline]
//! fn sum_impl<T>(token: T, input: &[f32]) -> f32
//! where
//!     T: Token,
//!     f32: ScalarExt<T> + core::iter::Sum<NativeVector<f32, T>>,
//! {
//!     // Use aligned loads in this example, which may be better on some architectures.
//!     let (start, vectors, end) = input.align_native(token);
//!
//!     // Sum across the vector lanes, plus the unaligned portions
//!     vectors.iter().copied().sum::<f32>() + start.iter().chain(end).sum::<f32>()
//! }
//!
//! // This function selects the best instruction set at runtime.
//! // The "dispatch" macro compiles this function for each supported architecture.
//! #[dispatch(token)]
//! fn sum(input: &[f32]) -> f32 {
//!     sum_impl(token, input)
//! }
//!
//! assert_eq!(sum(&[1f32; 10]), 10.);
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
//! use generic_simd::{
//!     arch::Token,
//!     dispatch,
//!     scalar::Scalar,
//!     vector::{Signed, Vector, width},
//! };
//!
//! // Equivalent to an array of 4 2-dimensional coordinates,
//! // but with a vectorizable memory layout.
//! struct Coordinates {
//!     x: [f64; 4],
//!     y: [f64; 4],
//! }
//!
//! // A generic mean implementation for any instruction set.
//! fn mean_impl<T>(token: T, input: &[Coordinates]) -> (f64, f64)
//! where
//!     T: Token,
//!     f64: Scalar<T, width::W4>,
//!     <f64 as Scalar<T, width::W4>>::Vector: Signed,
//! {
//!     let mut xsum = f64::zeroed(token);
//!     let mut ysum = f64::zeroed(token);
//!
//!     for Coordinates { x, y } in input {
//!         // read the arrays into vectors
//!         xsum += f64::read(token, x);
//!         ysum += f64::read(token, y);
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

/// Multiversions a function over all supported instruction sets.
///
/// Tagging a function with `#[dispatch(token)]` creates a version of the function for each
/// supported instruction set and provides its token as `token`.
/// The best supported function variant is selected at runtime.
///
/// # Implementation
/// This attribute is a wrapper for [`multiversion`] and supports all of its
/// conditional compilation and static dispatch features.
///
/// # Example
/// ```
/// use generic_simd::slice::SliceExt;
///
/// #[generic_simd::dispatch(token)]
/// pub fn add_one(x: &mut [f32]) {
///     let (start, vecs, end) = x.align_native_mut(token);
///     for s in start.iter_mut().chain(end.iter_mut()) {
///         *s += 1.;
///     }
///
///     for v in vecs {
///         *v += 1.;
///     }
/// }
///
/// #[generic_simd::dispatch(_token)]
/// pub fn add_two(x: &mut [f32]) {
///     // Static dispatching provided by `multiversion`.
///     // This does not perform runtime feature selection and allows inlining.
///     dispatch!(add_one(x));
///     dispatch!(add_one(x));
/// }
/// ```
///
/// [Abstractions]: index.html#abstractions
/// [Vector shims]: index.html#vector-shims
/// [`multiversion`]: ../multiversion/attr.multiversion.html
pub use generic_simd_macros::dispatch;

#[macro_use]
mod implementation;

pub mod alignment;
pub mod arch;
pub mod pointer;
pub mod scalar;
pub mod shim;
pub mod slice;
pub mod vector;
