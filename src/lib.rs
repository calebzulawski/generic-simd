pub mod vector;

#[macro_use]
mod implementation;

pub mod generic;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;
