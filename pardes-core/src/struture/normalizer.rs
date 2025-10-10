use syn::{Ident, ItemStruct, Visibility, parse_str, token::Pub};

use crate::struture::CORE_STRUCT_NAME;

pub fn struct_core_normalizer(item_struct: &mut ItemStruct) {
    let ident_core: Ident = parse_str(CORE_STRUCT_NAME).unwrap();
    item_struct.ident = ident_core;
    item_struct
        .fields
        .iter_mut()
        .for_each(|f| f.vis = Visibility::Public(Pub::default()));
}
