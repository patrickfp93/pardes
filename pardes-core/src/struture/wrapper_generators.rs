use syn::Index;

use crate::struture::guard_generator::generate_ref_lock_ident;

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
    let read_acessor_fn: Vec<ItemFn> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| generate_read_accessor(f, i,&ident))
        .collect();
    let write_acessor_fn: Vec<ItemFn> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| generate_write_accessor(f, i))
        .collect();
    parse_quote! {
        impl #ident{
            #(#read_acessor_fn)*
            #(#read_acessor_fn)*
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


#[seferize::expose_for_tests]
fn generate_read_accessor(field: &Field, index: usize, type_ident : &Ident) -> ItemFn {
    let vis = escalate_visibility(&field.vis);
    let (ident_fn,field_access) = field.ident
    .as_ref()
    .map_or_else(
        || {
            let ident_fn : TokenStream = parse_str(&format!("f{}",index)).unwrap();
            let idx = Index::from(index);            
            (ident_fn,quote! { #idx })
        },
        |ident| {
            let ident_fn: TokenStream = parse_str(&ident.to_string()).unwrap();
            (ident_fn,quote! { #ident })
        }
    );
    let ty = &field.ty;
    let ref_lock = generate_ref_lock_ident(type_ident);
    parse_quote! {
        #vis fn #ident_fn(&self) -> guards:: #ref_lock <'_, #ty> {
            let guard = self._core.read().unwrap();
            guards:: #ref_lock ::new(&guard. #field_access as *const #ty, guard)
        }
    }
} //precisa ser testado

fn generate_write_accessor(field: &Field, index: usize) -> ItemFn {
    let vis = escalate_visibility(&field.vis);
    let ident: TokenStream = if let Some(ident) = field.ident.as_ref() {
        ident.to_token_stream()
    } else {
        parse_str(&format!("{}", index)).unwrap()
    };
    let ty = &field.ty;
    let method_name: TokenStream = parse_str(&format!("{}_mut", ident)).unwrap();

    parse_quote! {
        #vis fn #method_name(&self) -> guards::SimpleStructMutLock<'_, #ty> {
            let mut guard = self._core.write().unwrap();
            let value = &mut guard.#ident as *mut #ty;
            guards::SimpleStructMutLock::new(value, guard)
        }
    }
} //precisa ser testado

fn escalate_visibility(vis: &Visibility) -> Visibility {
    match vis {
        // Caso privado → sobe um nível (pai)
        Visibility::Inherited => parse_quote!(pub(super)),

        // pub(super) → sobe mais um nível
        Visibility::Restricted(r) if r.in_token.is_none() && r.path.is_ident("super") => {
            parse_quote!(pub(in super::super))
        }

        // pub(in some::path) → adiciona um `super::` no início
        Visibility::Restricted(r) if r.in_token.is_some() => {
            let path = &r.path;
            parse_quote!(pub(in super::#path))
        }

        // Mantém visibilidade ampla
        other => other.clone(),
    }
} //precisa ser testado
