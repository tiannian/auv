use proc_macro::TokenStream;
use quote::quote;
// use crate::import::process_plain_fn_signature;

pub fn export(input: TokenStream) -> TokenStream {
    let mut f: syn::ItemFn = syn::parse(input).unwrap();
    let sig = &mut f.sig;
    if sig.constness.is_none()
        && sig.asyncness.is_none()
        && sig.unsafety.is_none()
        && sig.abi.is_none()
    {
        // plain fn
        // sig.inputs = process_plain_fn_signature(&sig.inputs);
    } else {
        panic!("unsupport!");
    }
    let t = quote!{
        #f
    };
    TokenStream::from(t)
}
