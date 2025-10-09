use crate::{struture::generate_head_type, tests::test_helpers::get_simple_struct_sample};

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

mod check_generate_debug_impl_core_method {
    use quote::ToTokens;

    use crate::{struture::{generate_debug_impl_core, util::get_possible_fields}, tests::test_helpers::{get_debug_impl_core_from_struct_sample, get_debug_impl_core_from_tuple, get_simple_struct_sample, get_simple_tuple}};

    #[test]
    pub fn case_simple_struct() {
        let item_struct = get_simple_struct_sample();
        let fields = get_possible_fields(&item_struct).unwrap();
        let impl_token = generate_debug_impl_core(&fields,&item_struct.ident);
        let desired_impl_token = get_debug_impl_core_from_struct_sample();
        assert_eq!(impl_token.to_token_stream().to_string(),desired_impl_token.to_token_stream().to_string())
    }
    #[test]
    pub fn case_tuple() {
        let item_struct = get_simple_tuple();
        let fields = get_possible_fields(&item_struct).unwrap();
        let impl_token = generate_debug_impl_core(&fields,&item_struct.ident);
        let desired_impl_token = get_debug_impl_core_from_tuple();
        assert_eq!(impl_token.to_token_stream().to_string(),desired_impl_token.to_token_stream().to_string())
    }
}
