use quote::ToTokens;
use rstest::rstest;
use syn::{parse_str, ItemStruct};
use crate::*;
//use crate::samples::struture::tuple::expected::tuple_expanse::_core::TUPLE_CORE_SAMPLE;
//use crate::samples::struture::tuple::original::TUPLE_SAMPLE;

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_CORE_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE, TUPLE_CORE_SAMPLE)]
pub fn check_struct_core_normalizer_method(
    #[case] item_struct: &'static str,
    #[case] desired_core: &'static str,
) {
    use crate::struture::normalizer::struct_core_normalizer;
    let mut item_struct : ItemStruct = parse_str(item_struct).unwrap();
    let desired_core : ItemStruct = parse_str(desired_core).unwrap();
    struct_core_normalizer(&mut item_struct);
    assert_eq!(item_struct.to_token_stream().to_string(), desired_core.to_token_stream().to_string())
}
