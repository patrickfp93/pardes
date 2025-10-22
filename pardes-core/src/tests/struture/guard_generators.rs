use rstest::rstest;
use syn::{parse_str, Ident, ItemMod, ItemStruct};

use crate::struture::guard_generators::{generate_guards, testable_generate_mut_lock_ident, testable_generate_ref_lock_ident};
use crate::tests::struture::util::ToStringItem;
use crate::*;

#[rstest]
#[case::simple_struct(crate::samples::struture::SIMPLE_STRUCT_SAMPLE,syn::parse_quote!(SimpleStructRefLock))]
#[case::tuple(crate::samples::struture::TUPLE_SAMPLE,syn::parse_quote!(TupleRefLock))]
pub fn check_generate_ref_lock_ident(#[case]item_struct_str: &'static str,#[case]expected_ref_locker_ident : Ident){
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let generated_ref_locker_ident = testable_generate_ref_lock_ident(&item_struct.ident);
    assert_eq!(generated_ref_locker_ident.to_string(),expected_ref_locker_ident.to_string())   
}

#[rstest]
#[case::simple_struct(crate::samples::struture::SIMPLE_STRUCT_SAMPLE,syn::parse_quote!(SimpleStructMutLock))]
#[case::tuple(crate::samples::struture::TUPLE_SAMPLE,syn::parse_quote!(TupleMutLock))]
pub fn check_generate_mut_lock_ident(#[case]item_struct_str: &'static str,#[case]expected_mut_locker_ident : Ident){   

    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();
    let generated_mut_locker_ident = testable_generate_mut_lock_ident(&item_struct.ident);
    assert_eq!(generated_mut_locker_ident.to_string(),expected_mut_locker_ident.to_string())   
}

#[rstest]
#[case::simple_struct(SIMPLE_STRUCT_SAMPLE,SIMPLE_STRUCT_GUARDS_MOD_SAMPLE)]
#[case::simple_struct(TUPLE_SAMPLE,TUPLE_GUARDS_MOD_SAMPLE)]
pub fn check_generate_guards(#[case]item_struct_str: &'static str,#[case]expected_guards_module_str : &'static str){   
    let item_struct : ItemStruct = parse_str(item_struct_str).unwrap();  
    let expected_guards_module : ItemMod = parse_str(expected_guards_module_str).unwrap();
    let generated_guards_module = generate_guards(&item_struct.ident);
    assert_eq!(generated_guards_module.to_string(),expected_guards_module.to_token_string())
}

