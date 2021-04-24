extern crate proc_macro;

use proc_macro::TokenStream;

mod import;

#[proc_macro_attribute]
pub fn cimport(_header: TokenStream, input: TokenStream) -> TokenStream {
    import::import(input)
}
