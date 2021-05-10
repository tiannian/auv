use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, FnArg, Token, Type, parse_quote, Pat, ReturnType};

pub fn match_basic_args_type(t: Type) -> bool {
    // is pointer or is function.
    let pointer = match t {
        Type::Ptr(_) => true,
        Type::BareFn(_) => true,
        _ => false
    };
    // is base type
    let basic_type = t == parse_quote!(u8) ||
        t == parse_quote!(u16) ||
        t == parse_quote!(u32) ||
        t == parse_quote!(u64) ||
        t == parse_quote!(u128) ||
        t == parse_quote!(i8) ||
        t == parse_quote!(i16) ||
        t == parse_quote!(i32) ||
        t == parse_quote!(i64) ||
        t == parse_quote!(i128) ||
        t == parse_quote!(usize) ||
        t == parse_quote!(isize);
    pointer || basic_type
}

pub fn match_bytes_args_type(t: Type) -> bool {
    // is &[u8]
    // is Vec[u8]
    // is [u8; N]
    let is_array = if let Type::Array(t_inner) = t.clone() {
        t_inner == parse_quote!(u8)
    } else {
        false
    };
    t == parse_quote!(&[u8]) ||
        t == parse_quote!(Vec<u8>) ||
        is_array
}

pub fn process_plain_fn_input(
    args: &Punctuated<FnArg, Token![,]>,
) -> Punctuated<FnArg, Token![,]> {
    let mut result = Punctuated::new();
    for fn_arg in args {
        if let FnArg::Typed(pat) = fn_arg.clone() {
            if match_basic_args_type(*pat.ty.clone()) {
                // if argument is basic type, keep.
                result.push(fn_arg.clone());
            } else if match_bytes_args_type(*pat.ty.clone()) {
                // if argument is bytes, convert to ptr and len.
                if let Pat::Ident(p) = *pat.pat {
                    let name_ident = p.ident;
                    let name_ptr_ident = quote::format_ident!("{}_ptr", name_ident);
                    result.push(parse_quote!(#name_ptr_ident: *const u8));
                    let name_len_ident = quote::format_ident!("{}_len", name_ident);
                    result.push(parse_quote!(#name_len_ident: usize));
                }
            }
        }
    }
    result
}

pub fn import(input: TokenStream) -> TokenStream {
    let mut f: syn::ForeignItemFn = syn::parse(input).unwrap();
    let sig = &mut f.sig;
    if sig.constness.is_none()
        && sig.asyncness.is_none()
        && sig.unsafety.is_none()
        && sig.abi.is_none()
    {
        // plain fn
        // process fn input
        sig.inputs = process_plain_fn_input(&sig.inputs);
        // process fn output
        let output_vaild = match &sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, t) => match_basic_args_type(*(t.clone())),
        };
        if !output_vaild {
            panic!("unsupport! Return type must be empty or basic type");
        }
    } else {
        panic!("unsupport!");
    }
    let result = quote! {
        #f
    };
    TokenStream::from(result)
}
