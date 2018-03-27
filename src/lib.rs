//!

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Thunder)]
pub fn thunder_clap(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {

    };

    // Hand the output tokens back to the compiler
    expanded.into()
}