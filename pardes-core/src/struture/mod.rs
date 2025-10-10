pub(crate) mod util;
pub(crate) mod normalizer;

use crate::{struture::normalizer::struct_core_normalizer, Result};
use crate::struture::error::ErrorStruture;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Field, Ident, ItemImpl, ItemStruct, parse_quote, parse_str};

const CORE_STRUCT_NAME: &'static str = "_Core";
const EXPANSE_MODULE_NAME: &'static str = "expanse";

use crate::struture::util::{get_ident_expanse_module, get_possible_fields};

pub fn expantion(item_struct: ItemStruct) -> Result<TokenStream> {
    //create core module
    let head_type = generate_head_type(&item_struct);
    let fields = get_possible_fields(&item_struct).ok_or(ErrorStruture::NoFields)?;
    let expanse_module_ident = get_ident_expanse_module(&item_struct);
    let core_module = generate_core_module(&item_struct, &fields);

    Ok(quote! {
        #head_type
        mod #expanse_module_ident{
            #core_module
        }
    })
}

pub fn generate_head_type(item_struct: &ItemStruct) -> TokenStream {
    let vis = &item_struct.vis;
    let ident = &item_struct.ident;
    let module_ident = util::get_ident_expanse_module(item_struct);
    return quote! {
        #vis type #ident = #module_ident::#ident;
    };
}

pub fn generate_core_module(item_struct: &ItemStruct, fields: &[Field]) -> TokenStream {
    let mut core_item_struct = item_struct.clone();
    struct_core_normalizer(&mut core_item_struct);
    core_item_struct.vis = parse_str("pub").unwrap();
    let debug_impl = generate_debug_impl_core(fields, &item_struct.ident);

    quote! {
        #[doc(hidden)]
        mod _core{
            #core_item_struct
            #debug_impl
        }
    }
}
///precisa ser testado

pub fn generate_debug_impl_core(fields: &[Field], struct_ident: &Ident) -> ItemImpl {
    let core_ident: TokenStream = parse_str(CORE_STRUCT_NAME).unwrap();
    let struct_name = struct_ident.to_string();
    let field_tokens: Vec<TokenStream> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let index: TokenStream = parse_str(&format!("{}", i)).unwrap();
            let possible_name = f.ident.as_ref().map(|f| f.to_string());
            let ident: TokenStream = f
                .ident
                .clone()
                .map(|id| id.to_token_stream())
                .or_else(|| Some(index))
                .unwrap();
            if let Some(name) = possible_name {
                quote! { .field(#name, &self.#ident)}
            } else {
                quote! { .field(&self.#ident)}
            }
        })
        .collect();
    let type_debug_method_token: TokenStream = parse_str(
        fields[0]
            .ident
            .clone()
            .map_or_else(|| "debug_tuple", |_| "debug_struct"),
    )
    .unwrap();
    parse_quote! {
        impl std::fmt::Debug for #core_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.#type_debug_method_token (#struct_name)
                    #(#field_tokens)*
                    .finish()
            }
        }
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ErrorStruture {
        #[error("Not compatible with Unit structures.")]
        NoFields,
    }
}
