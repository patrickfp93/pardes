pub(crate) mod core_generators;
pub(crate) mod wrapper_generators;
pub(crate) mod normalizer;
pub(crate) mod util;
pub(crate) mod guard_generators;
pub mod error;

pub(crate) use core_generators::*;
pub(crate) use wrapper_generators::*;


use crate::struture::error::ErrorStruture;
use crate::struture::guard_generators::generate_guards;
use crate::{Result, struture::normalizer::struct_core_normalizer};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Field, Ident, ItemImpl,ItemFn, ItemStruct,Visibility, parse_quote, parse_str};
const CORE_STRUCT_NAME: &'static str = "_Core";
const EXPANSE_MODULE_NAME: &'static str = "expanse";

use crate::struture::util::get_ident_expanse_module;

pub fn expantion(item_struct: ItemStruct) -> Result<TokenStream> {
    if item_struct.fields.len() == 0 {return Err(ErrorStruture::NoFields.into())}
    let head_type = generate_head_type(&item_struct);
    let expanse_module_ident = get_ident_expanse_module(&item_struct);
    let core_module = generate_core_module(&item_struct);
    let wrapper = generate_wrapper(&item_struct);
    let guards = generate_guards(&item_struct.ident);
    Ok(quote! {
        #head_type
        mod #expanse_module_ident{
            #core_module
            #wrapper
            #guards
        }
    })
} //testar expantion depois

pub fn generate_head_type(item_struct: &ItemStruct) -> TokenStream {
    let vis = &item_struct.vis;
    let ident = &item_struct.ident;
    let module_ident = util::get_ident_expanse_module(item_struct);
    return quote! {
        #vis type #ident = #module_ident::#ident;
    };
}
