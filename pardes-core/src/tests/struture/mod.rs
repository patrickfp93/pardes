use crate::struture::*;
use crate::tests::struture::util::{extract_content_from_module, ToStringItem};
use proc_macro2::TokenStream;
use rstest::rstest;
use syn::{ItemImpl, ItemMod};
use syn::{ItemStruct, parse_str};

#[cfg(test)]
use pretty_assertions as pa;

use crate::struture::core_generators::*;
use super::*;
pub mod util;

pub mod normalizer;

pub mod core_generators;

pub mod wrapper_generators;

pub mod guard_generators;

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
    pa::assert_eq!(
        head_type_generated.to_string(),
        head_type_sample.to_string()
    )
}


#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_EXPANSE_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE, TUPLE_EXPANSE_SAMPLE)]
pub fn check_expantion(
    #[case] item_struct_str: &'static str,
    #[case] expected_expanse_str: &'static str,
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();
    let generate_expanse: TokenStream = expantion(item_struct).unwrap();
    let expected_expanse: ItemMod = parse_str(expected_expanse_str).unwrap();
    let expected_expanse = extract_content_from_module(&expected_expanse).unwrap();
    pa::assert_eq!(
        generate_expanse.to_string(),
        expected_expanse.to_string()
    )
}

