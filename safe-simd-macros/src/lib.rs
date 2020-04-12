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
        #(#attrs)*
        #[multiversion::target_clones(
            "[x86|x86_64]+avx",
            "[x86|x86_64]+sse"
        )]
        #vis
        #sig
        {
            #[target_cfg(target = "[x86|x86_64]+sse")]
            let #feature = unsafe { safe_simd::x86::sse::Sse::new_unchecked() };

            #[target_cfg(target = "[x86|x86_64]+avx")]
            let #feature = unsafe { safe_simd::x86::avx::Avx::new_unchecked() };

            #[target_cfg(not(any(target = "[x86|x86_64]+sse", target = "[x86|x86_64]+avx")))]
            let #feature = unsafe { safe_simd::generic::Generic::new_unchecked() };

            #block
        }
    };
    output.into()
}
