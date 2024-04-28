use std::ops::Deref;

////use chrono::prelude::*;
////use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, Ident, ItemFn, Pat, Stmt};
///

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let data = &ast.data;
    let attrs = &ast.attrs;
    let generics = &ast.generics;

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

fn ty_to_string(syn_type: syn::Type) -> String {
    match syn_type {
        syn::Type::Array(_) => "Array".to_owned(),
        syn::Type::BareFn(_) => "BareFn".to_owned(),
        syn::Type::Group(_) => "Group".to_owned(),
        syn::Type::ImplTrait(_) => "ImplTrait".to_owned(),
        syn::Type::Infer(_) => "Infer".to_owned(),
        syn::Type::Macro(_) => "Macro".to_owned(),
        syn::Type::Never(_) => "Never".to_owned(),
        syn::Type::Paren(_) => "Paren".to_owned(),
        syn::Type::Path(what) => {
            let q_self = what.qself;
            match q_self {
                Some(q) => {
                    let typ = q.ty;
                    ty_to_string(*typ).to_owned()
                }
                None => "Path(none)".to_owned(),
            }
        }
        syn::Type::Ptr(_) => "Ptr".to_owned(),
        syn::Type::Reference(_) => "Reference".to_owned(),
        syn::Type::Slice(_) => "Slice".to_owned(),
        syn::Type::TraitObject(_) => "TraitObject".to_owned(),
        syn::Type::Tuple(_) => "Tuple".to_owned(),
        syn::Type::Verbatim(_) => "Verbatim".to_owned(),
        _ => "???".to_owned(),
    }
}
struct I {}
//
#[proc_macro_derive(ObjObj)]
pub fn yakito_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let data: &syn::Data = &ast.data;

    let ast_attrs = &ast.attrs;
    for ast_attr in ast_attrs {
        let attr_path_ident = ast_attr.path.get_ident().unwrap();

        println!("{:?}", attr_path_ident);
    }

    let struck = match data {
        syn::Data::Struct(datastruct) => datastruct,
        syn::Data::Enum(_) => panic!(),
        syn::Data::Union(_) => panic!(),
    };

    for field in struck.fields.clone() {
        let field_ty = ty_to_string(field.ty.clone());

        let syn_attrs: Vec<syn::Attribute> = field.attrs;
        for syn_attr in syn_attrs {
            let syn_attr_path_ident = syn_attr.path.get_ident().unwrap();

            println!("{:?}", syn_attr_path_ident);
        }

        println!(
            "Field in {name}: ident:{:?}, type: {:?}.",
            field.ident.clone(),
            field_ty
        );
    }

    let trait_impl = quote! {

        trait ObjObj {
            fn objobj_id(&self) -> u64;
            fn objobj_object_type(&self) -> ObjectType;
        }


        impl ObjObj for #name {
            fn objobj_id(&self) -> u64{21}
            fn objobj_object_type(&self) -> ObjectType{ObjectType::Pii}
        }
    };

    trait_impl.into()
}
