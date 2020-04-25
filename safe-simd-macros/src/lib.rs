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
        #[multiversion::multiversion]
        #[clone(target = "[x86|x86_64]+avx")]
        #[clone(target = "[x86|x86_64]+sse3")]
        #(#attrs)*
        #vis
        #sig
        {
            #[target_cfg(target = "[x86|x86_64]+sse3")]
            let #feature = unsafe { <safe_simd::x86::sse::Sse as safe_simd::vector::Feature>::new_unchecked() };

            #[target_cfg(target = "[x86|x86_64]+avx")]
            let #feature = unsafe { <safe_simd::x86::avx::Avx as safe_simd::vector::Feature>::new_unchecked() };

            #[target_cfg(not(any(target = "[x86|x86_64]+sse3", target = "[x86|x86_64]+avx")))]
            let #feature = <safe_simd::generic::Generic as safe_simd::vector::Feature>::new().unwrap();

            #block
        }
    };
    output.into()
}
