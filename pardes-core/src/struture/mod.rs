pub(crate) mod util;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemStruct, parse_str};

const CORE_STRUCT_NAME: &'static str = "_Core";
const EXPANSE_MODULE_NAME: &'static str = "expanse";

use crate::struture::util::get_ident_expanse_module;

pub fn expantion(item_struct: ItemStruct) -> TokenStream {
    //create core module
    let head_type = generate_head_type(&item_struct);
    let core_module = generate_core_module(&item_struct);
    let expanse_module_ident = get_ident_expanse_module(&item_struct);

    quote! {
        #head_type
        mod #expanse_module_ident{
            #core_module
        }
    }
}

pub fn generate_head_type(item_struct: &ItemStruct) -> TokenStream {
    let vis = &item_struct.vis;
    let ident = &item_struct.ident;
    let module_ident = util::get_ident_expanse_module(item_struct);
    return quote! {
        #vis type #ident = #module_ident::#ident;
    };
}

pub fn generate_core_module(item_struct: &ItemStruct) -> TokenStream {
    let ident_core: Ident = parse_str(CORE_STRUCT_NAME).unwrap();
    let mut core_item_struct = item_struct.clone();
    core_item_struct.ident = ident_core;
    core_item_struct.vis = parse_str("pub").unwrap();
    let debug_impl = generate_debug_impl_core(item_struct);

    quote! {
        mod _core{
            #[doc(hidden)]
            #core_item_struct
            #debug_impl
        }
    }
}

pub fn generate_debug_impl_core(item_struct: &ItemStruct) -> TokenStream {
    let core_ident: TokenStream = parse_str(CORE_STRUCT_NAME).unwrap();

    todo!(
        "verificar se possue named fields. Necessário decidir como será tratado os casos diferentes."
    );
    quote! {
        impl std::fmt::Debug for #core_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Wrapper")
                    .field("value", &self.field1)
                    .field("value_2", &self.field2)
                    .finish()
            }
        }
    }
}
