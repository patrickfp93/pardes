use quote::{quote, ToTokens};
use syn::{parse2, ItemStruct};

use crate::{struture::util::{get_ident_expanse_module, get_possible_fields}, tests::test_helpers::get_simple_struct_sample};


#[test]
pub fn check_get_ident_expanse_module_method() {
    let item_struct = get_simple_struct_sample(); 
    
    let ident_expanse_module = get_ident_expanse_module(&item_struct);

    assert_eq!(ident_expanse_module.to_string(),"simple_struct_expanse".to_string())
}

#[test]
pub fn check_get_possible_field_method(){
    let item_struct = get_simple_struct_sample();
    let possible_named_fields = get_possible_fields(&item_struct);
    assert!(possible_named_fields.is_some());
    let named_fields = possible_named_fields.unwrap();
    assert_eq!(named_fields.len(), 2);
    assert_eq!(named_fields[0].to_token_stream().to_string(), "pub (super) field1 : String");
    assert_eq!(named_fields[1].to_token_stream().to_string(), "pub field2 : i32");
}