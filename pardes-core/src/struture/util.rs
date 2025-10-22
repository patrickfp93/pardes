use crate::struture::EXPANSE_MODULE_NAME;
use convert_case::{Case, Casing};
use syn::{parse_str,Ident, ItemStruct};

pub fn get_ident_expanse_module(item_struct: &ItemStruct) -> Ident {
    let module_ident_string = item_struct.ident.to_string().to_case(Case::Snake);
    parse_str(&format!("{}_{}", module_ident_string, EXPANSE_MODULE_NAME)).unwrap()
}

