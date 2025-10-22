use syn::{Field, ItemFn, Visibility};

use crate::tests::utilities::to_items;

use super::*;

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_WRAPPER_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE, TUPLE_WRAPPER_SAMPLE)]
pub fn check_generate_wrapper_struct(
    #[case] item_struct_str: &'static str,
    #[case] wrapper_sample_str: &'static str,
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();
    let wrapper_sample: ItemStruct = parse_str(wrapper_sample_str).unwrap();
    let wrapper = testable_generate_wrapper_struct(&item_struct);
    assert_eq!(wrapper_sample.to_token_string(), wrapper.to_token_string())
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_WRAPPER_IMPL_BUILDER_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE, TUPLE_WRAPPER_IMPL_BUILDER_SAMPLE)]
pub fn check_generate_wrapper_impl_builder(
    #[case] item_struct_str: &'static str,
    #[case] wrapper_impl_builder_str: &'static str,
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();
    let wrapper_impl_builder: ItemImpl = parse_str(wrapper_impl_builder_str).unwrap();
    let wrapper_impl_builder_generated = testable_generate_wrapper_impl_builder(&item_struct);
    assert_eq!(
        wrapper_impl_builder_generated.to_token_string(),
        wrapper_impl_builder.to_token_string(),
    )
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,vec![SIMPLE_STRUCT_WRAPPER_READER_FIELD_1,SIMPLE_STRUCT_WRAPPER_READER_FIELD_2])]
#[case::tuple(TUPLE_SAMPLE,vec![TUPLE_WRAPPER_READER_F_0,TUPLE_WRAPPER_READER_F_1])]
pub fn check_generate_read_accessor(#[case]item_struct_str : &'static str,#[case] reader_method_str_list : Vec<&'static str>){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let expected_reader_methods: Vec<ItemFn> = to_items(reader_method_str_list.as_slice()).unwrap(); 
    let fields = item_struct.fields;
    let ident_struct = &item_struct.ident;
    //check if same len
    assert_eq!(expected_reader_methods.len(),fields.len());
    
    let generated_reader_methods : Vec<ItemFn> = fields.iter().enumerate().map(|(index,field)|{
        testable_generate_read_accessor(field, index,ident_struct)
    }).collect();
    //check each item
    generated_reader_methods.iter().zip(expected_reader_methods).for_each(|(g_r_m,e_r_m)|{
        assert_eq!(g_r_m.to_token_string(),e_r_m.to_token_string())
    });

}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,vec![SIMPLE_STRUCT_WRAPPER_WRITER_FIELD_1,SIMPLE_STRUCT_WRAPPER_WRITER_FIELD_2])]
#[case::tuple(TUPLE_SAMPLE,vec![TUPLE_WRAPPER_WRITER_F_0,TUPLE_WRAPPER_WRITER_F_1])]
pub fn check_generate_write_accessor(#[case]item_struct_str : &'static str,#[case] reader_method_str_list : Vec<&'static str>){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let expected_reader_methods: Vec<ItemFn> = to_items(reader_method_str_list.as_slice()).unwrap(); 
    let ident_struct = &item_struct.ident;
    //check if same len
    assert_eq!(expected_reader_methods.len(),item_struct.fields.len());
    
    let generated_reader_methods : Vec<ItemFn> = item_struct.fields.iter().enumerate().map(|(index,field)|{
        testable_generate_write_accessor(field, index,ident_struct)
    }).collect();
    //check each item
    generated_reader_methods.iter().zip(expected_reader_methods).for_each(|(g_r_m,e_r_m)|{
        assert_eq!(g_r_m.to_token_string(),e_r_m.to_token_string())
    });

}


#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_WRAPPER_IMPL_ACCESS_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_WRAPPER_IMPL_ACCESS_SAMPLE)]
pub fn check_generate_wrapper_impl_access(#[case]item_struct_str : &'static str,
 #[case]impl_access_str : &'static str){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let expected_impl_access : ItemImpl = parse_str(impl_access_str).unwrap();
    let generated_impl_access: ItemImpl = testable_generate_wrapper_impl_access(&item_struct);
    assert_eq!(generated_impl_access.to_token_string(),expected_impl_access.to_token_string())
}

#[rstest]
#[case(syn::parse_quote!(pub), syn::parse_quote!(pub))]
#[case(syn::parse_quote!(pub(super)), syn::parse_quote!(pub(in super::super)))]
#[case(syn::parse_quote!(pub(crate)), syn::parse_quote!(pub(crate)))]
#[case(syn::parse_quote!(pub(in super::super)), syn::parse_quote!(pub(in super::super::super)))]
#[case(syn::parse_quote!(), syn::parse_quote!(pub(super)))] // caso de private, espera escalonar
pub fn check_escalate_visibility(
    #[case] vis_sample: Visibility,
    #[case] vis_expected: Visibility,
) {
    use quote::ToTokens;

    let result = testable_escalate_visibility(&vis_sample);
    assert_eq!(result.to_token_stream().to_string(), vis_expected.to_token_stream().to_string());
}


#[rstest]
#[case((syn::parse_quote!(pub field_1 : usize),0), (syn::parse_quote!(field_1),syn::parse_quote!(field_1)))]
#[case((syn::parse_quote!(pub usize),0), (syn::parse_quote!(f0),syn::parse_quote!(0)))]
pub fn check_get_method_idents(#[case](field,index) : (Field,usize),#[case]idents_exp:(TokenStream,TokenStream)){
    let idents = testable_get_method_idents(&field, index);

    assert_eq!(idents.0.to_string(),idents_exp.0.to_string());
    assert_eq!(idents.1.to_string(),idents_exp.1.to_string());
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_WRAPPER_IMPL_DEBUG_SAMPLE)]
#[case::simple_struct(TUPLE_SAMPLE,TUPLE_WRAPPER_IMPL_DEBUG_SAMPLE)]
pub fn check_generate_wrapper_impl_debug(#[case]item_struct_str : &'static str,
 #[case]expected_impl_debug_str : &'static str){

    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let expected_impl_debug: TokenStream = parse_str(expected_impl_debug_str).unwrap();
    let generated_impl_debug = testable_generate_wrapper_impl_debug(&item_struct.ident);

    assert_eq!(generated_impl_debug.to_string(),expected_impl_debug.to_string())
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_WRAPPER_IMPL_PARTIAL_EQ_SAMPLE)]
#[case::simple_struct(TUPLE_SAMPLE,TUPLE_WRAPPER_IMPL_PARTIAL_EQ_SAMPLE)]
pub fn check_generate_wrapper_impl_partial_eq(#[case]item_struct_str : &'static str,
 #[case]expected_impl_partial_eq_str : &'static str){

    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let expected_impl_partial_eq: TokenStream = parse_str(expected_impl_partial_eq_str).unwrap();
    let generated_impl_debug = testable_generate_wrapper_impl_partial_eq(&item_struct.ident);

    assert_eq!(generated_impl_debug.to_string(),expected_impl_partial_eq.to_string())
}