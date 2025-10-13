use crate::samples::struture::tuple::expected::tuple_expanse::Tuple;

#[seferize::stringify("TUPLE_TYPE_SAMPLE")]
pub type TUPLE = tuple_expanse::Tuple;

#[allow(unused)]
pub use tuple_expanse::*;
#[seferize::stringify("TUPLE_EXPANSE_SAMPLE")]
mod tuple_expanse {
    #[seferize::ignore]
    #[allow(unused)]
    pub use _core::*;
    #[seferize::stringify("TUPLE_CORE_MODULE_SAMPLE")]
    #[doc(hidden)]
    mod _core {
        #[seferize::stringify("TUPLE_CORE_SAMPLE")]
        #[allow(unused)]
        pub struct _Core(pub String, pub i32);

        #[seferize::stringify("TUPLE_IMPL_DEBUG_SAMPLE")]
        impl std::fmt::Debug for _Core {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("Tuple")
                    .field(&self.0)
                    .field(&self.1)
                    .finish()
            }
        }
    }

    #[seferize::stringify("TUPLE_WRAPPER_SAMPLE")]
    pub struct Tuple {
        _core: std::sync::Arc<std::sync::RwLock<_core::_Core>>,
    }

    #[seferize::stringify("TUPLE_WRAPPER_IMPL_BUILDER_SAMPLE")]
    impl Tuple {
        pub(super) fn builder(value: (String, i32)) -> Self {
            let core = _core::_Core(value.0, value.1);
            Self {
                _core: std::sync::Arc::new(std::sync::RwLock::new(core)),
            }
        }
    }

    #[seferize::stringify("TUPLE_WRAPPER_IMPL_ACCESS_SAMPLE")]
    impl Tuple {
        pub(in super::super) fn f0(&self) -> guards::TupleRefLock<'_, String> {
            let guard = self._core.read().unwrap();
            guards::TupleRefLock::new(&guard.0 as *const String, guard)
        }

        pub(in super::super) fn f0_mut(&self) -> guards::TupleMutLock<'_, String> {
            let mut guard = self._core.write().unwrap();
            let value = &mut guard.0 as *mut String;
            return guards::TupleMutLock::new(value, guard);
        }

        pub fn f1(&self) -> guards::TupleRefLock<'_, i32> {
            let guard = self._core.read().unwrap();
            guards::TupleRefLock::new(&guard.1 as *const i32, guard)
        }

        pub fn f1_mut(&self) -> guards::TupleMutLock<'_, i32> {
            let mut guard = self._core.write().unwrap();
            let value = &mut guard.1 as *mut i32;
            return guards::TupleMutLock::new(value, guard);
        }
    }

    #[seferize::stringify("TUPLE_WRAPPER_IMPL_DEBUG_SAMPLE")]
    impl std::fmt::Debug for Tuple {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            (*self._core.read().unwrap()).fmt(f)
        }
    }

    #[seferize::stringify("TUPLE_WRAPPER_IMPL_PARTIAL_EQ_SAMPLE")]
    impl PartialEq for Tuple {
        fn eq(&self, other: &Self) -> bool {
            let ptr_number = self._core.as_ref() as *const std::sync::RwLock<_core::_Core> as usize;
            let other_ptr_number =
                other._core.as_ref() as *const std::sync::RwLock<_core::_Core> as usize;
            ptr_number == other_ptr_number
        }
    }

    #[seferize::stringify("TUPLE_GUARDS_MOD_SAMPLE")]
    pub mod guards {
        pub struct TupleRefLock<'a, T> {
            _guard: std::sync::RwLockReadGuard<'a, super::_core::_Core>,
            reference: &'a T,
        }

        impl<'a, T> TupleRefLock<'a, T> {
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

        impl<'a, T> std::ops::Deref for TupleRefLock<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.reference
            }
        }

        pub struct TupleMutLock<'a, T> {
            _guard: std::sync::RwLockWriteGuard<'a, super::_core::_Core>,
            reference: &'a T,
            reference_mutable: &'a mut T,
        }

        impl<'a, T> TupleMutLock<'a, T> {
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

        impl<'a, T> std::ops::Deref for TupleMutLock<'a, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.reference
            }
        }

        impl<'a, T> std::ops::DerefMut for TupleMutLock<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.reference_mutable
            }
        }
    }
}

impl Tuple {
    pub fn new(value: (String, i32)) -> Self {
        Self::builder(value)
    }

    pub fn sum_five(&mut self) {
        *self.f1_mut() += 5;
    }
}
