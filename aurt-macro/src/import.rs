use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, FnArg, Token, Type, parse_quote};

pub fn match_basic_args_type(t: Type) -> bool {
    if t == parse_quote!(u8) {
        true
    } else {
        false
    }
}

pub fn match_bytes_args_type(t: Type) -> bool {
    false
}

pub fn process_plain_fn_signature(
    args: Punctuated<FnArg, Token![,]>,
) -> Punctuated<FnArg, Token![,]> {
    let mut result = Punctuated::new();
    for fn_arg in args {
        if let FnArg::Typed(pat) = fn_arg.clone() {
            if match_basic_args_type(*pat.ty.clone()) {
                println!("aaaaa: {:?}", fn_arg);
                result.push(fn_arg);
            } else if match_bytes_args_type(*pat.ty) {
                result.push(fn_arg);
            }
        }
    }
    result
}

pub fn import(input: TokenStream) -> TokenStream {
    let f: syn::ForeignItemFn = syn::parse(input).unwrap();
    println!("{:?}", f);
    let mut sig = f.sig;
    if sig.constness.is_none()
        && sig.asyncness.is_none()
        && sig.unsafety.is_none()
        && sig.abi.is_none()
    {
        // plain fn
        sig.inputs = process_plain_fn_signature(sig.inputs);
    } else {
        panic!("unsupport!");
    }
    let result = quote! {
        f
    };
    TokenStream::from(result)
}
