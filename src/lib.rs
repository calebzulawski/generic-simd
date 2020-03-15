pub mod vector;

#[macro_use]
mod implementation;

pub mod slice;

pub mod generic;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

/// Dispatches a closure or generic with the highest supported CPU feature, as if by [`apply`].
///
/// [`apply`]: vector/trait.Feature.html#tymethod.apply
#[macro_export]
macro_rules! dispatch {
    {
        $closure:expr
    } => {
        {
            let f = || {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    if let Some(sse) = $crate::x86::sse::Sse::new() {
                        return sse.apply($closure);
                    }
                }
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    if let Some(avx) = $crate::x86::avx::Avx::new() {
                        return avx.apply($closure);
                    }
                }
                $crate::generic::Generic::default().apply($closure)
            };
            f()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vector::{Capability, Feature};

    #[test]
    fn dispatch_macro() {
        let x = 1.0f64;
        assert_eq!(
            dispatch!(|f| {
                let x = f.splat(x);
                (x + x)[0]
            }),
            2.0
        );
    }
}
