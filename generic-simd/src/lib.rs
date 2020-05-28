#![cfg_attr(not(feature = "std"), no_std)]

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
