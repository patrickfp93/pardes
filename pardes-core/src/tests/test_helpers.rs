use quote::quote;
use syn::{ItemStruct, parse2};

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
