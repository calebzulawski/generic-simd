//! Architecture-specific types.

/// Indicates support for a particular CPU feature.
///
/// # Safety
/// Implementing `Token` for a type indicates that the type is only constructible when the
/// associated CPU features are supported.
pub unsafe trait Token: Copy + From<Self> + Into<Self> {
    /// Detects whether the required CPU features are supported.
    fn new() -> Option<Self>;

    /// Creates the token without detecting if the CPU features are supported.
    ///
    /// # Safety
    /// Calling this function causes undefined behavior if the required CPU features are not
    /// supported.
    unsafe fn new_unchecked() -> Self;
}

macro_rules! impl_cpu {
    { $name:ident => $($features:tt),+ } => {
        unsafe impl $crate::arch::Token for $name {
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
