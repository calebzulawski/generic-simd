#![cfg_attr(not(feature = "std"), no_std)]

pub use safe_simd_macros::dispatch;

pub mod vector;

#[macro_use]
mod implementation;

pub mod slice;

pub mod generic;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;
