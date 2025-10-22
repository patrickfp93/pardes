use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, parse_str};

use crate::{MUT_LOCKER_NAME, REF_LOCKER_NAME};

pub(super) fn generate_ref_lock_ident(type_ident: &Ident) -> TokenStream {
    Ident::new(
        &format!("{}{}", type_ident, REF_LOCKER_NAME),
        Span::call_site(),
    )
    .to_token_stream()
}//precisa fazer test

pub(super) fn generate_mut_lock_ident(type_ident: &Ident) -> TokenStream {
    let ident: Ident =
        parse_str(&format!("{}{}", &type_ident.to_string(), MUT_LOCKER_NAME)).unwrap();
    ident.to_token_stream()
}//precisa fazer test

pub fn generate_guards(type_ident: &Ident) -> TokenStream {
    let read_ident = generate_ref_lock_ident(type_ident);
    let write_ident = generate_mut_lock_ident(type_ident);
    quote! {
        pub mod guards {
        pub struct #read_ident<'a, T> {
            _guard: std::sync::RwLockReadGuard<'a, super::_core::_Core>,
            reference: &'a T,
        }

        impl<'a, T> #read_ident<'a, T> {
            pub(super) fn new(
                ptr: *const T,
                guard: std::sync::RwLockReadGuard<'a, super::_core::_Core>,
            ) -> Self {
                let reference = unsafe { &*ptr };
                Self {
                    _guard: guard,
                    reference,
                }
            }
        }

        impl<'a, T> std::ops::Deref for #read_ident<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.reference
            }
        }

        pub struct #write_ident<'a, T> {
            _guard: std::sync::RwLockWriteGuard<'a, super::_core::_Core>,
            reference: &'a T,
            reference_mutable: &'a mut T,
        }

        impl<'a, T> #write_ident<'a, T> {
            pub(super) fn new(
                ptr: *mut T,
                guard: std::sync::RwLockWriteGuard<'a, super::_core::_Core>,
            ) -> Self {
                let reference = unsafe { &*ptr };
                let reference_mutable = unsafe { &mut *ptr };
                Self {
                    _guard: guard,
                    reference,
                    reference_mutable,
                }
            }
        }

        impl<'a, T> std::ops::Deref for #write_ident<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.reference
            }
        }

        impl<'a, T> std::ops::DerefMut for #write_ident<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.reference_mutable
            }
        }
    }
    }
}//precisa fazer test
