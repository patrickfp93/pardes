use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_quote, ItemImpl, ItemStruct};

pub fn get_simple_struct_sample() -> ItemStruct {
    let item_struct_token = quote! {
            pub struct SimpleStruct {
                pub(super) field1: String,
                //#[only_read]
                pub field2: i32,
            }
        };
    parse2(item_struct_token).expect("Failed to parse for ItemStruct")
}

pub fn get_debug_impl_core_from_struct_sample() -> ItemImpl{
    parse_quote! {
        impl std::fmt::Debug for _Core {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("SimpleStruct")
                    .field("field1", &self.field1)
                    .field("field2", &self.field2)
                    .finish()
            }
        }
    }
}

pub fn get_simple_tuple() -> ItemStruct {
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
