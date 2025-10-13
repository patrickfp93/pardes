use crate::tests::struture::util::ToStringItem;
use proc_macro2::TokenStream;
use rstest::rstest;
use syn::{ItemImpl, ItemMod};
use syn::{parse_str, ItemStruct};
use crate::struture::*;


use crate::struture::{core_generators::*, util::get_possible_fields};
use crate::*;
pub mod util;

pub mod normalizer;



#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_TYPE_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_TYPE_SAMPLE)]
pub fn check_generate_head_type_method(
    #[case] item_struct_str: &'static str,
    #[case] type_str: &'static str
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();
    let head_type_generated: TokenStream = generate_head_type(&item_struct);
    let head_type_sample : TokenStream = parse_str(type_str).unwrap();
    assert_eq!(
        head_type_generated.to_string(),
        head_type_sample.to_string()
    )
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_IMPL_DEBUG_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_IMPL_DEBUG_SAMPLE)]
pub fn check_generate_debug_impl_core_method(
    #[case] item_struct_str: &'static str,
    #[case] desired_impl_token_sample_str: &'static str){        
    
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let fields: Vec<syn::Field> = get_possible_fields(&item_struct).unwrap();
    let impl_token: ItemImpl = generate_debug_impl_core(&fields,&item_struct.ident);
    let desired_impl_sample : ItemImpl = parse_str(desired_impl_token_sample_str).unwrap();
    assert_eq!(impl_token.to_token_string(),desired_impl_sample.to_token_string())
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_CORE_MODULE_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_CORE_MODULE_SAMPLE)]
pub fn check_generate_core_module(
    #[case] item_struct_str: &'static str,
    #[case] code_module_sample_str:&'static str 
){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let fields: Vec<syn::Field> = get_possible_fields(&item_struct).unwrap();
    let gen_core_module = generate_core_module(&item_struct, &fields);
    let code_module_sample : ItemMod = parse_str(code_module_sample_str).unwrap();
    assert_eq!(gen_core_module.to_token_string(),code_module_sample.to_token_string())
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_WRAPPER_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_WRAPPER_SAMPLE)]
pub fn check_generate_wrapper_struct(#[case]item_struct_str: &'static str,#[case]wrapper_sample_str : &'static str){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let wrapper_sample: ItemStruct = parse_str(wrapper_sample_str).unwrap();
    let wrapper = generate_wrapper_struct(&item_struct);
    assert_eq!(wrapper_sample.to_token_string(),wrapper.to_token_string())
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_WRAPPER_IMPL_BUILDER_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_WRAPPER_IMPL_BUILDER_SAMPLE)]
pub fn check_generate_wrapper_impl_acess(#[case]item_struct_str : &'static str,#[case]wrapper_impl_builder_str : &'static str){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let wrapper_impl_builder : ItemImpl = parse_str(wrapper_impl_builder_str).unwrap();
    let fields: Vec<syn::Field> = get_possible_fields(&item_struct).unwrap();
    let wrapper_impl_builder_generated = generate_wrapper_impl_builder(&item_struct, &fields);
    assert_eq!(wrapper_impl_builder.to_token_string(),wrapper_impl_builder_generated.to_token_string())
}


