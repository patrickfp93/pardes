use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_str, Ident};

use crate::{MUT_LOCKER_NAME, REF_LOCKER_NAME};



pub fn generate_ref_lock_ident(type_ident : &Ident) -> TokenStream{
    Ident::new(&format!("{}{}",type_ident ,REF_LOCKER_NAME), Span::call_site()).to_token_stream()
}

pub fn generate_mut_lock_ident(type_ident : &Ident) -> TokenStream{
    let ident : Ident = parse_str(&format!("{}{}",&type_ident.to_string(),MUT_LOCKER_NAME)).unwrap();
    ident.to_token_stream()
}
