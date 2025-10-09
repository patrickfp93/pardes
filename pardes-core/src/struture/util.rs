use std::default;

use crate::struture::EXPANSE_MODULE_NAME;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Field, Fields, Ident, ItemStruct};

pub fn get_ident_expanse_module(item_struct: &ItemStruct) -> Ident {
    let module_ident_string = item_struct.ident.to_string().to_case(Case::Snake);
    parse_str(&format!("{}_{}", module_ident_string, EXPANSE_MODULE_NAME)).unwrap()
}

pub fn get_possible_named_field_idents(item_struct: &ItemStruct) -> Option<Vec<Field>> {
    if let Fields::Named(fields_named) = &item_struct.fields {
        Some(fields_named
            .named
            .iter()
            .map(|f| {
                f.clone()
                /*let name = f.ident.as_ref().unwrap().to_string(); // nome do campo como string
                let ident = &f.ident; // Ident para acessar self.nome
                quote! { .field(#name, &self.#ident) }*/
            })
            .collect())
    }else{
        None
    }
}


