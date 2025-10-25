pub use expanse::*;

#[seferize::stringify(SIMPLE_STRUCT_EXPANSE_SAMPLE)]
pub mod expanse {

    #[seferize::ignore]
    pub use simple_struct_expanse::*;

    #[seferize::stringify(SIMPLE_STRUCT_TYPE_SAMPLE)]
    pub type SimpleStruct = simple_struct_expanse::SimpleStruct;

    mod simple_struct_expanse {

        #[seferize::ignore]
        pub use _core::*;

        #[seferize::stringify(SIMPLE_STRUCT_CORE_MODULE_SAMPLE)]
        #[doc(hidden)]
        mod _core {
            #[seferize::stringify(SIMPLE_STRUCT_CORE_SAMPLE)]
            pub struct _Core {
                pub field1: String,
                //#[only_read]
                pub field2: i32,
            }

            #[seferize::stringify(SIMPLE_STRUCT_IMPL_DEBUG_SAMPLE)]
            impl std::fmt::Debug for _Core {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("SimpleStruct")
                        .field("field1", &self.field1)
                        .field("field2", &self.field2)
                        .finish()
                }
            }
        }

        #[seferize::stringify(SIMPLE_STRUCT_WRAPPER_SAMPLE)]
        pub struct SimpleStruct {
            _core: std::sync::Arc<std::sync::RwLock<_core::_Core>>,
        }

        #[seferize::stringify(SIMPLE_STRUCT_WRAPPER_IMPL_BUILDER_SAMPLE)]
        impl SimpleStruct {
            pub(super) fn builder(field1: String, field2: i32) -> Self {
                let core = _core::_Core { field1, field2 };
                Self {
                    _core: std::sync::Arc::new(std::sync::RwLock::new(core)),
                }
            }
        }

        #[seferize::stringify(SIMPLE_STRUCT_WRAPPER_IMPL_ACCESS_SAMPLE)]
        impl SimpleStruct {
            #[stringify(SIMPLE_STRUCT_WRAPPER_READER_FIELD_1)]
            pub(in super::super) fn field1(&self) -> guards::SimpleStructRefLock<'_, String> {
                let guard = self._core.read().unwrap();
                return guards::SimpleStructRefLock::new(&guard.field1 as *const String, guard);
            }

            #[stringify(SIMPLE_STRUCT_WRAPPER_READER_FIELD_2)]
            pub fn field2(&self) -> guards::SimpleStructRefLock<'_, i32> {
                let guard = self._core.read().unwrap();
                return guards::SimpleStructRefLock::new(&guard.field2 as *const i32, guard);
            }

            #[stringify(SIMPLE_STRUCT_WRAPPER_WRITER_FIELD_1)]
            pub(in super::super) fn field1_mut(&self) -> guards::SimpleStructMutLock<'_, String> {
                let mut guard = self._core.write().unwrap();
                let value = &mut guard.field1 as *mut String;
                return guards::SimpleStructMutLock::new(value, guard);
            }

            #[stringify(SIMPLE_STRUCT_WRAPPER_WRITER_FIELD_2)]
            pub fn field2_mut(&self) -> guards::SimpleStructMutLock<'_, i32> {
                let mut guard = self._core.write().unwrap();
                let value = &mut guard.field2 as *mut i32;
                return guards::SimpleStructMutLock::new(value, guard);
            }
        }

        #[seferize::stringify(SIMPLE_STRUCT_WRAPPER_IMPL_DEBUG_SAMPLE)]
        impl std::fmt::Debug for SimpleStruct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self._core.read().unwrap()).fmt(f)
            }
        }

        #[seferize::stringify(SIMPLE_STRUCT_WRAPPER_IMPL_PARTIAL_EQ_SAMPLE)]
        impl PartialEq for SimpleStruct {
            fn eq(&self, other: &Self) -> bool {
                let ptr_number =
                    self._core.as_ref() as *const std::sync::RwLock<_core::_Core> as usize;
                let other_ptr_number =
                    other._core.as_ref() as *const std::sync::RwLock<_core::_Core> as usize;
                ptr_number == other_ptr_number
            }
        }

        #[seferize::stringify(SIMPLE_STRUCT_GUARDS_MOD_SAMPLE)]
        pub mod guards {
            pub struct SimpleStructRefLock<'a, T> {
                _guard: std::sync::RwLockReadGuard<'a, super::_core::_Core>,
                reference: &'a T,
            }

            impl<'a, T> SimpleStructRefLock<'a, T> {
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

            impl<'a, T> std::ops::Deref for SimpleStructRefLock<'a, T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    self.reference
                }
            }

            pub struct SimpleStructMutLock<'a, T> {
                _guard: std::sync::RwLockWriteGuard<'a, super::_core::_Core>,
                reference: &'a T,
                reference_mutable: &'a mut T,
            }

            impl<'a, T> SimpleStructMutLock<'a, T> {
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

            impl<'a, T> std::ops::Deref for SimpleStructMutLock<'a, T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    self.reference
                }
            }

            impl<'a, T> std::ops::DerefMut for SimpleStructMutLock<'a, T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.reference_mutable
                }
            }
        }
    }

    //user Edition

    #[seferize::ignore]
    #[allow(unused)]
    impl SimpleStruct {
        pub fn new(field1: String, field2: i32) -> Self {
            Self::builder(field1, field2)
        }

        pub fn sum_five(&mut self) {
            *self.field2_mut() += 5;
        }
    }
}
