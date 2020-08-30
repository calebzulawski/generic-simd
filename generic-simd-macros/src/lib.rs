extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn dispatch(args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    let feature = parse_macro_input!(args as Ident);

    let build_fn = |wasm, arm| {
        let nightly = cfg!(feature = "nightly");
        let clone_wasm = if nightly && wasm {
            Some(quote! { #[clone(target = "wasm32+simd128")] })
        } else {
            None
        };
        let clone_arm = if nightly && arm {
            Some(quote! { #[clone(target = "arm+neon")] })
        } else {
            None
        };
        let clone_aarch64 = if nightly {
            Some(quote! { #[clone(target = "aarch64+neon")] })
        } else {
            None
        };
        quote! {
            #[generic_simd::multiversion::multiversion]
            #[clone(target = "[x86|x86_64]+avx")]
            #[clone(target = "[x86|x86_64]+sse4.1")]
            #clone_wasm
            #clone_arm
            #clone_aarch64
            #[crate_path(path = "generic_simd::multiversion")]
            #(#attrs)*
            #vis
            #sig
            {
                #[target_cfg(target = "[x86|x86_64]+sse4.1")]
                let #feature = unsafe { <generic_simd::arch::x86::Sse as generic_simd::arch::Token>::new_unchecked() };

                #[target_cfg(target = "[x86|x86_64]+avx")]
                let #feature = unsafe { <generic_simd::arch::x86::Avx as generic_simd::arch::Token>::new_unchecked() };

                #[target_cfg(target = "wasm32+simd128")]
                let #feature = unsafe { <generic_simd::arch::wasm32::Simd128 as generic_simd::arch::Token>::new_unchecked() };

                #[target_cfg(target = "[arm|aarch64]+neon")]
                let #feature = unsafe { <generic_simd::arch::arm::Neon as generic_simd::arch::Token>::new_unchecked() };

                #[target_cfg(not(any(
                    target = "[x86|x86_64]+sse4.1",
                    target = "[x86|x86_64]+avx",
                    target = "[arm|aarch64]+neon",
                    target = "wasm32+simd128",
                )))]
                let #feature = <generic_simd::arch::generic::Generic as generic_simd::arch::Token>::new().unwrap();

                #block
            }
        }
    };
    let normal = build_fn(false, false);
    let with_wasm = build_fn(true, false);
    let with_arm = build_fn(false, true);
    let output = quote! {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        #with_wasm

        #[cfg(all(target_arch = "arm", target_feature = "v7", target_feature = "neon"))]
        #with_arm

        #[cfg(not(any(
            all(target_arch = "wasm32", target_feature = "simd128"),
            all(target_arch = "arm", target_feature = "v7", target_feature = "neon"),
        )))]
        #normal
    };
    output.into()
}
