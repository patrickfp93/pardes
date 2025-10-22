use syn::ItemMod;

use super::*;

pub fn generate_core_module(item_struct: &ItemStruct) -> ItemMod {
    let mut core_item_struct = item_struct.clone();
    struct_core_normalizer(&mut core_item_struct);
    core_item_struct.vis = parse_str("pub").unwrap();
    let debug_impl = generate_debug_impl_core(item_struct);

    parse_quote! {
        #[doc(hidden)]
        mod _core{
            #core_item_struct
            #debug_impl
        }
    }
}

pub fn generate_debug_impl_core(item_struct: &ItemStruct) -> ItemImpl {
    let core_ident: TokenStream = parse_str(CORE_STRUCT_NAME).unwrap();
    let struct_name = item_struct.ident.to_string();
    let field_tokens: Vec<TokenStream> = item_struct.fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let index: TokenStream = parse_str(&format!("{}", i)).unwrap();
            let possible_name = f.ident.as_ref().map(|f| f.to_string());
            let ident: TokenStream = f
                .ident
                .clone()
                .map(|id| id.to_token_stream())
                .or_else(|| Some(index))
                .unwrap();
            if let Some(name) = possible_name {
                quote! { .field(#name, &self.#ident)}
            } else {
                quote! { .field(&self.#ident)}
            }
        })
        .collect();
    let type_debug_method_token: TokenStream = parse_str(
        item_struct.fields.iter().nth(0).unwrap()
            .ident
            .clone()
            .map_or_else(|| "debug_tuple", |_| "debug_struct"),
    )
    .unwrap();
    parse_quote! {
        impl std::fmt::Debug for #core_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.#type_debug_method_token (#struct_name)
                    #(#field_tokens)*
                    .finish()
            }
        }
    }
}
