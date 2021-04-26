extern crate proc_macro;

use proc_macro::TokenStream;

mod import;
mod export;

#[proc_macro_attribute]
pub fn import(_header: TokenStream, input: TokenStream) -> TokenStream {
    import::import(input)
}

#[proc_macro_attribute]
pub fn export(_header: TokenStream, input: TokenStream) -> TokenStream {
    export::export(input)
}
