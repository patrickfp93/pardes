use crate::struture::EXPANSE_MODULE_NAME;
use convert_case::{Case, Casing};
use syn::{parse_str, Field, Fields, Ident, ItemStruct};

pub fn get_ident_expanse_module(item_struct: &ItemStruct) -> Ident {
    let module_ident_string = item_struct.ident.to_string().to_case(Case::Snake);
    parse_str(&format!("{}_{}", module_ident_string, EXPANSE_MODULE_NAME)).unwrap()
}

pub fn get_possible_fields(item_struct: &ItemStruct) -> Option<Vec<Field>>{
    match &item_struct.fields {
        Fields::Named(fields_named) => Some(fields_named.named.iter().map(|f| f.clone()).collect()),
        Fields::Unnamed(fields_unnamed) => {            
            Some(fields_unnamed.unnamed.iter().map(|f| f.clone()).collect())
        },
        Fields::Unit => None,
    }
}


