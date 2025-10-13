use super::*;

pub fn generate_wrapper_struct(item_struct: &ItemStruct) -> ItemStruct {
    let ident = item_struct.ident.clone();
    parse_quote! {
        pub struct #ident {
        _core: std::sync::Arc<std::sync::RwLock<_core::_Core>>,
        }
    }
}

pub fn generate_wrapper_impl_access(item_struct: &ItemStruct, fields: &[Field]) -> ItemImpl {
    let ident = item_struct.ident.clone();
    let access_fn: Vec<ItemFn> = fields.iter().map(|f| generate_access(f)).collect();
    parse_quote! {
        impl #ident{
            #(#access_fn)*
        }
    }
}
//precisa ser testado


pub fn generate_wrapper_impl_builder(item_struct: &ItemStruct, fields: &[Field]) -> ItemImpl {
    let ident = item_struct.ident.clone();
    let is_named_fields = fields.iter().find(|f| f.ident.is_some()).is_some();

    let core_init = if is_named_fields {
        // Caso: fields nomeados
        let field_names: Vec<&Ident> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
        quote! {
            _core::_Core { #(#field_names),* }
        }
    } else {
        // Caso: fields não nomeados (tuple struct)
        let field_indices = (0..fields.len()).map(syn::Index::from);
        quote! {
            _core::_Core ( #(#field_indices),* )
        }
    };
    parse_quote! {
        impl #ident{
        fn builder() -> Self {
            let core = #core_init;
            Self {
                _core: std::sync::Arc::new(std::sync::RwLock::new(core)),
            }
        }
    }
    }
}



fn generate_access(field: &Field) -> ItemFn {
    todo!()
}