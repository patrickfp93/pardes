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

pub mod check_generate_debug_impl_core_method;

pub mod normalizer;
