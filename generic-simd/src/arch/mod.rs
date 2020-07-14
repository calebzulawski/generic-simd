//! Architecture-specific types.

pub unsafe trait Cpu: Copy + From<Self> + Into<Self> {
    fn new() -> Option<Self>;
    unsafe fn new_unchecked() -> Self;
}

macro_rules! impl_cpu {
    { $name:ident => $($features:tt),+ } => {
        unsafe impl $crate::arch::Cpu for $name {
            fn new() -> Option<Self> {
                if multiversion::are_cpu_features_detected!($($features),*) {
                    Some(Self(()))
                } else {
                    None
                }
            }

            unsafe fn new_unchecked() -> Self {
                Self(())
            }
        }

        impl core::convert::From<$name> for $crate::arch::generic::Generic {
            fn from(_: $name) -> Self {
                Self
            }
        }
    }
}

pub mod generic;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;
