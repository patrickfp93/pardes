use quote::ToTokens;
use rstest::rstest;

use super::*;
use crate::struture::util::*;

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, "simple_struct_expanse")]
#[case::tuple(TUPLE_SAMPLE, "tuple_expanse")]
pub fn check_get_ident_expanse_module_method(
    #[case] item_struct_str: &'static str,
    #[case] ident_sample: &'static str,
) {
    let item_struct: ItemStruct = parse_str(item_struct_str).unwrap();

    let ident_expanse_module = get_ident_expanse_module(&item_struct);

    assert_eq!(ident_expanse_module.to_string(), ident_sample)
}

pub fn extract_content_from_module(item_mod: &ItemMod) -> Option<TokenStream> {
    if let Some(content) = item_mod.clone().content.map(|c| c.1) {
        return Some(quote::quote! (#(#content)*));
    }
    None
}

/*#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE, [FIELD_1_SIMPLE_STRUCT_SAMPLE,FIELD_2_SIMPLE_STRUCT_SAMPLE])]
#[case::tuple(TUPLE_SAMPLE,[FIELD_1_TUPLE_SAMPLE,FIELD_2_TUPLE_SAMPLE])]
pub fn check_get_possible_field_method(
    #[case] item_struct_str: &'static str,
    #[case] fields:  [&'static str;2]
){
    use quote::ToTokens;
    let item_struct = parse_str(item_struct_str).unwrap();
    let possible_named_fields = get_possible_fields(&item_struct);
    assert!(possible_named_fields.is_some());
    let named_fields = possible_named_fields.unwrap();
    assert_eq!(named_fields.len(), 2);
    fields.iter().enumerate().for_each(|(index,&field)|{
        assert_eq!(named_fields[index].to_token_stream().to_string().replace(" ", "")
        , field.to_string().replace(" ", ""));
    });
}*/

pub trait ToStringItem {
    fn to_token_string(&self) -> String;
}

/// Implementação automática para qualquer tipo que implemente `ToTokens`
impl<T: ToTokens> ToStringItem for T {
    fn to_token_string(&self) -> String {
        self.to_token_stream().to_string()
    }
}
