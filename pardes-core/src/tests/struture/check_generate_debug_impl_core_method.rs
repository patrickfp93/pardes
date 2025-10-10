use quote::ToTokens;

use crate::{struture::{generate_debug_impl_core, util::get_possible_fields}, tests::test_helpers::{get_simple_struct_impl_debug_sample, get_debug_impl_core_from_tuple, get_simple_struct_sample, get_tuple_sample}};

#[test]
pub fn case_simple_struct() {
    let item_struct = get_simple_struct_sample();
    let fields = get_possible_fields(&item_struct).unwrap();
    let impl_token = generate_debug_impl_core(&fields,&item_struct.ident);
    let desired_impl_token = get_simple_struct_impl_debug_sample();
    assert_eq!(impl_token.to_token_stream().to_string(),desired_impl_token.to_token_stream().to_string())
}
#[test]
pub fn case_tuple() {
    let item_struct = get_tuple_sample();
    let fields = get_possible_fields(&item_struct).unwrap();
    let impl_token = generate_debug_impl_core(&fields,&item_struct.ident);
    let desired_impl_token = get_debug_impl_core_from_tuple();
    assert_eq!(impl_token.to_token_stream().to_string(),desired_impl_token.to_token_stream().to_string())
}
