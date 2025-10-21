use syn::ItemFn;

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
    let wrapper = generate_wrapper_struct(&item_struct);
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
    let fields: Vec<syn::Field> = get_possible_fields(&item_struct).unwrap();
    let wrapper_impl_builder_generated = generate_wrapper_impl_builder(&item_struct, &fields);
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
    let fields = item_struct.fields;
    let ident_struct = &item_struct.ident;
    //check if same len
    assert_eq!(expected_reader_methods.len(),fields.len());
    
    let generated_reader_methods : Vec<ItemFn> = fields.iter().enumerate().map(|(index,field)|{
        testable_generate_write_accessor(field, index,ident_struct)
    }).collect();
    //check each item
    generated_reader_methods.iter().zip(expected_reader_methods).for_each(|(g_r_m,e_r_m)|{
        assert_eq!(g_r_m.to_token_string(),e_r_m.to_token_string())
    });

}