////use chrono::prelude::*;
////use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
////use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, Ident, ItemFn, Pat, Stmt};
///

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {}", stringify!(#name), msg);
            }
            fn warn(&self, msg: &str) {
                println!("[Warn] {}: {}", stringify!(#name), msg);
            }
            fn error(&self, msg: &str) {
                println!("[Err] {}: {}", stringify!(#name), msg);
            }
        }
    };

    trait_impl.into()
}

//
#[proc_macro_derive(Yakito)]
pub fn yakito_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
        pub trait #name {
            fn get_id(&self) -> u64;
            fn get_object_type(&self) -> ObjectType;
        }


        impl #name for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {}", stringify!(#name), msg);
            }
            fn warn(&self, msg: &str) {
                println!("[Warn] {}: {}", stringify!(#name), msg);
            }
            fn error(&self, msg: &str) {
                println!("[Err] {}: {}", stringify!(#name), msg);
            }
        }
    };

    trait_impl.into()
}
