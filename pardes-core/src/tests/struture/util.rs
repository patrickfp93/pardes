use quote::ToTokens;
use rstest::rstest;

use crate::struture::util::*;
use super::*;


#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, "simple_struct_expanse")]
#[case::tuple(TUPLE_SAMPLE,"tuple_expanse")]
pub fn check_get_ident_expanse_module_method(
    #[case] item_struct_str: &'static str,
    #[case] ident_sample: &'static str
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();
    
    let ident_expanse_module = get_ident_expanse_module(&item_struct);

    assert_eq!(ident_expanse_module.to_string(),ident_sample)
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, SIMPLE_STRUCT_TYPE_SAMPLE)]
#[case::tuple(TUPLE_SAMPLE,TUPLE_TYPE_SAMPLE)]
pub fn check_get_possible_field_method(
    #[case] item_struct_str: &'static str,
    #[case] type_str: &'static str
){
    use quote::ToTokens;
    let item_struct = parse_str(item_struct_str).unwrap();
    let possible_named_fields = get_possible_fields(&item_struct);
    assert!(possible_named_fields.is_some());
    let named_fields = possible_named_fields.unwrap();
    assert_eq!(named_fields.len(), 2);
    assert_eq!(named_fields[0].to_token_stream().to_string(), "pub (super) field1 : String");
    assert_eq!(named_fields[1].to_token_stream().to_string(), "pub field2 : i32");
}//melhorar sistema de test

pub trait ToStringItem {
    fn to_token_string(&self) -> String;
}

/// Implementação automática para qualquer tipo que implemente `ToTokens`
impl<T: ToTokens> ToStringItem for T {
    fn to_token_string(&self) -> String {
        self.to_token_stream().to_string()
    }
}