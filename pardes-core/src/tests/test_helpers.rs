use quote::quote;
use syn::{parse2, parse_quote, parse_str, ItemImpl, ItemStruct};
use crate::*;

pub fn get_simple_struct_sample() -> ItemStruct {
    //let item_struct_token = parse_str(SIMPLE_STRUCT_SAMPLE).unwrap();
    //parse2(item_struct_token).expect("Failed to parse for ItemStruct")
    parse_str(SIMPLE_STRUCT_SAMPLE).unwrap()
}

pub fn get_simple_struct_impl_debug_sample() -> ItemImpl{
    parse_str(SIMPLE_STRUCT_IMPL_DEBUG_SAMPLE).unwrap()
}

pub fn get_simple_struct_core_sample() -> ItemStruct{
    parse_str(SIMPLE_STRUCT_CORE_SAMPLE).unwrap()
}

pub fn get_tuple_sample() -> ItemStruct {
    let item_struct_token = quote! {
            pub struct Tuple(pub(super) String, pub i32);
        };
    parse2(item_struct_token).expect("Failed to parse for ItemStruct")
}

pub fn get_debug_impl_core_from_tuple() -> ItemImpl{
    parse_quote! {
        impl std::fmt::Debug for _Core {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("Tuple")
                    .field(&self.0)
                    .field(&self.1)
                    .finish()
            }
        }
    }
}

pub fn get_tuple_core() -> ItemStruct{
    parse_quote!{
        pub struct _Core(pub String, pub i32);
    }
}
