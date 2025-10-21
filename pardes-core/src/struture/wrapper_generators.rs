use syn::Index;

use crate::struture::guard_generator::{generate_mut_lock_ident, generate_ref_lock_ident};

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
        .map(|(i, f)| generate_write_accessor(f, i,&ident))
        .collect();
    parse_quote! {
        impl #ident{
            #(#read_acessor_fn)*
            #(#write_acessor_fn)*
        }
    }
}
//precisa ser testado

pub fn generate_wrapper_impl_builder(item_struct: &ItemStruct, fields: &[Field]) -> ItemImpl {
    let ident = item_struct.ident.clone();
    let is_named_fields = fields.iter().find(|f| f.ident.is_some()).is_some();
     
    let (params,core_init) = if is_named_fields {
        // Caso: fields nomeados 
        let params = fields.iter().map(|field|{
            let ident = field.clone().ident.unwrap();
            let ty = field.ty.clone();
            quote! (#ident : #ty)
        });
        let field_names: Vec<&Ident> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
        (quote! {
           #(#params),* 
        },quote! {
            _core::_Core { #(#field_names),* }
        })
    } else {
        // Caso: fields não nomeados (tuple struct)
        let tys = fields.iter().map(|f|f.ty.to_token_stream());
        let params: TokenStream = quote! {value : ( #(#tys),* )};

        let field_indices = (0..fields.len()).map(syn::Index::from)
        .map(|i| quote! {value.#i});

        (params,quote! {
            _core::_Core ( #(#field_indices),* )
        })
    };
    parse_quote! {
        impl #ident{
        pub(super) fn builder(#params) -> Self {
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
    let (ident_fn,field_access) = get_method_idents(field,index);
    let ty = &field.ty;
    let ref_lock = generate_ref_lock_ident(type_ident);
    parse_quote! {
        #vis fn #ident_fn(&self) -> guards:: #ref_lock <'_, #ty> {
            let guard = self._core.read().unwrap();
            return guards:: #ref_lock ::new(&guard. #field_access as *const #ty, guard);
        }
    }
}

#[seferize::expose_for_tests]
fn generate_write_accessor(field: &Field, index: usize,type_ident : &Ident) -> ItemFn {
    let vis = escalate_visibility(&field.vis);
    let (ident_fn,field_access) = get_method_idents(field,index);

    let ty = &field.ty;
    let method_name: TokenStream = parse_str(&format!("{}_mut", ident_fn)).unwrap();
    let mut_lock = generate_mut_lock_ident(type_ident);
    parse_quote! {
        #vis fn #method_name(&self) -> guards::#mut_lock<'_, #ty> {
            let mut guard = self._core.write().unwrap();
            let value = &mut guard.#field_access as *mut #ty;
            return guards:: #mut_lock ::new(value, guard);
        }
    }
}

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

fn get_method_idents(field: &Field, index: usize) -> (TokenStream,TokenStream){
    field.ident
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
    )
}//precisa ser testado