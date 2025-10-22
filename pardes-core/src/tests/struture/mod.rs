use crate::struture::*;
use crate::tests::struture::util::ToStringItem;
use proc_macro2::TokenStream;
use rstest::rstest;
use syn::{ItemImpl, ItemMod};
use syn::{ItemStruct, parse_str};

use crate::struture::core_generators::*;
use crate::*;
pub mod util;

pub mod normalizer;

pub mod core_generators;

pub mod wrapper_generators;

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_TYPE_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE, TUPLE_TYPE_SAMPLE)]
pub fn check_generate_head_type_method(
    #[case] item_struct_str: &'static str,
    #[case] type_str: &'static str,
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();
    let head_type_generated: TokenStream = generate_head_type(&item_struct);
    let head_type_sample: TokenStream = parse_str(type_str).unwrap();
    assert_eq!(
        head_type_generated.to_string(),
        head_type_sample.to_string()
    )
}

