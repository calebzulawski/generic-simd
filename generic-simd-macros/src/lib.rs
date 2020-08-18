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

    let output = quote! {
        #[generic_simd::multiversion::multiversion]
        #[clone(target = "[x86|x86_64]+avx")]
        #[clone(target = "[x86|x86_64]+sse4.1")]
        #[clone(target = "wasm32+simd128")]
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
            let #feature = unsafe { <generic_simd::arch::wasm::Wasm as generic_simd::arch::Token>::new_unchecked() };

            #[target_cfg(not(any(
                target = "[x86|x86_64]+sse4.1",
                target = "[x86|x86_64]+avx",
                target = "wasm32+simd128",
            )))]
            let #feature = <generic_simd::arch::generic::Generic as generic_simd::arch::Token>::new().unwrap();

            #block
        }
    };
    output.into()
}
