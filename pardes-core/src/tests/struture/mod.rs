use crate::{struture::generate_head_type, tests::test_helpers::get_simple_struct_sample};
use quote::ToTokens;
use rstest::rstest;
use syn::ItemImpl;

use crate::struture::{generate_debug_impl_core, util::get_possible_fields};
use crate::*;
pub mod util;

#[test]
pub fn check_generate_head_type_method() {
    let item_struct = get_simple_struct_sample();
    let head_type_method = generate_head_type(&item_struct);
    assert_eq!(
        head_type_method.to_string(),
        "pub type SimpleStruct = simple_struct_expanse :: SimpleStruct ;".to_string()
    )
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_IMPL_DEBUG_SAMPLE)]
#[case::simple_struct(TUPLE_SAMPLE,TUPLE_IMPL_DEBUG_SAMPLE)]
pub fn check_generate_debug_impl_core_method(
    #[case] item_struct_str: &'static str,
    #[case] desired_impl_token_str: &'static str){
        use syn::{parse_str, ItemStruct};
    
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let fields: Vec<syn::Field> = get_possible_fields(&item_struct).unwrap();
    let impl_token: ItemImpl = generate_debug_impl_core(&fields,&item_struct.ident);
    let desired_impl_token : ItemImpl = parse_str(desired_impl_token_str).unwrap();
    assert_eq!(impl_token.to_token_stream().to_string(),desired_impl_token.to_token_stream().to_string())
}

pub mod normalizer;
