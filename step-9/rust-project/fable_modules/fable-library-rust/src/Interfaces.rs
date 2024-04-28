pub mod Interfaces_ {
    use super::*;
    pub mod System {
        use super::*;
        use crate::Native_::Any;
        use crate::Native_::LrcPtr;
        use crate::Native_::TypeId;
        use crate::String_::string;
        pub trait IDisposable {
            fn Dispose(&self);
        }
        impl<V: IDisposable + Clone + 'static> IDisposable for LrcPtr<V> {
            #[inline]
            fn Dispose(&self) {
                (**self).Dispose()
            }
        }
        pub trait IEquatable_1<T: Clone + 'static> {
            fn Equals(&self, arg0: T) -> bool;
        }
        impl<V: IEquatable_1<T> + Clone + 'static, T: Clone + 'static> IEquatable_1<T> for LrcPtr<V> {
            #[inline]
            fn Equals(&self, arg0: T) -> bool {
                (**self).Equals(arg0)
            }
        }
        pub trait IComparable {
            fn CompareTo(&self, arg0: &dyn Any) -> i32;
        }
        impl<V: IComparable + Clone + 'static> IComparable for LrcPtr<V> {
            #[inline]
            fn CompareTo(&self, arg0: &dyn Any) -> i32 {
                (**self).CompareTo(arg0)
            }
        }
        pub trait IComparable_1<T: Clone + 'static> {
            fn CompareTo(&self, arg0: T) -> i32;
        }
        impl<V: IComparable_1<T> + Clone + 'static, T: Clone + 'static> IComparable_1<T> for LrcPtr<V> {
            #[inline]
            fn CompareTo(&self, arg0: T) -> i32 {
                (**self).CompareTo(arg0)
            }
        }
        pub trait IFormatProvider {
            fn GetFormat(&self, arg0: TypeId) -> &dyn Any;
        }
        impl<V: IFormatProvider + Clone + 'static> IFormatProvider for LrcPtr<V> {
            #[inline]
            fn GetFormat(&self, arg0: TypeId) -> &dyn Any {
                (**self).GetFormat(arg0)
            }
        }
        pub trait IFormattable {
            fn ToString_(
                &self,
                arg0: string,
                arg1: LrcPtr<dyn Interfaces_::System::IFormatProvider>,
            ) -> string;
        }
        impl<V: IFormattable + Clone + 'static> IFormattable for LrcPtr<V> {
            #[inline]
            fn ToString_(
                &self,
                arg0: string,
                arg1: LrcPtr<dyn Interfaces_::System::IFormatProvider>,
            ) -> string {
                (**self).ToString_(arg0, arg1)
            }
        }
        pub mod Collections {
            use super::*;
            pub trait IComparer {
                fn Compare(&self, arg0: &dyn Any, arg1: &dyn Any) -> i32;
            }
            impl<V: IComparer + Clone + 'static> IComparer for LrcPtr<V> {
                #[inline]
                fn Compare(&self, arg0: &dyn Any, arg1: &dyn Any) -> i32 {
                    (**self).Compare(arg0, arg1)
                }
            }
            pub trait IEqualityComparer {
                fn Equals(&self, arg0: &dyn Any, arg1: &dyn Any) -> bool;
                fn GetHashCode(&self, arg0: &dyn Any) -> i32;
            }
            impl<V: IEqualityComparer + Clone + 'static> IEqualityComparer for LrcPtr<V> {
                #[inline]
                fn Equals(&self, arg0: &dyn Any, arg1: &dyn Any) -> bool {
                    (**self).Equals(arg0, arg1)
                }
                #[inline]
                fn GetHashCode(&self, arg0: &dyn Any) -> i32 {
                    (**self).GetHashCode(arg0)
                }
            }
            pub trait IEnumerator {
                fn MoveNext(&self) -> bool;
                fn Reset(&self);
                fn get_Current(&self) -> &dyn Any;
                fn Dispose(&self);
            }
            impl<V: IEnumerator + Clone + 'static> IEnumerator for LrcPtr<V> {
                #[inline]
                fn MoveNext(&self) -> bool {
                    (**self).MoveNext()
                }
                #[inline]
                fn Reset(&self) {
                    (**self).Reset()
                }
                #[inline]
                fn get_Current(&self) -> &dyn Any {
                    (**self).get_Current()
                }
                #[inline]
                fn Dispose(&self) {
                    (**self).Dispose()
                }
            }
            pub trait IEnumerable {
                fn GetEnumerator(
                    &self,
                ) -> LrcPtr<dyn Interfaces_::System::Collections::IEnumerator>;
            }
            impl<V: IEnumerable + Clone + 'static> IEnumerable for LrcPtr<V> {
                #[inline]
                fn GetEnumerator(
                    &self,
                ) -> LrcPtr<dyn Interfaces_::System::Collections::IEnumerator> {
                    (**self).GetEnumerator()
                }
            }
            pub trait IStructuralComparable {
                fn CompareTo(
                    &self,
                    arg0: &dyn Any,
                    arg1: LrcPtr<dyn Interfaces_::System::Collections::IComparer>,
                ) -> i32;
            }
            impl<V: IStructuralComparable + Clone + 'static> IStructuralComparable for LrcPtr<V> {
                #[inline]
                fn CompareTo(
                    &self,
                    arg0: &dyn Any,
                    arg1: LrcPtr<dyn Interfaces_::System::Collections::IComparer>,
                ) -> i32 {
                    (**self).CompareTo(arg0, arg1)
                }
            }
            pub trait IStructuralEquatable {
                fn Equals(
                    &self,
                    arg0: &dyn Any,
                    arg1: LrcPtr<dyn Interfaces_::System::Collections::IEqualityComparer>,
                ) -> bool;
                fn GetHashCode(
                    &self,
                    arg0: LrcPtr<dyn Interfaces_::System::Collections::IEqualityComparer>,
                ) -> i32;
            }
            impl<V: IStructuralEquatable + Clone + 'static> IStructuralEquatable for LrcPtr<V> {
                #[inline]
                fn Equals(
                    &self,
                    arg0: &dyn Any,
                    arg1: LrcPtr<dyn Interfaces_::System::Collections::IEqualityComparer>,
                ) -> bool {
                    (**self).Equals(arg0, arg1)
                }
                #[inline]
                fn GetHashCode(
                    &self,
                    arg0: LrcPtr<dyn Interfaces_::System::Collections::IEqualityComparer>,
                ) -> i32 {
                    (**self).GetHashCode(arg0)
                }
            }
            pub mod Generic {
                use super::*;
                use crate::NativeArray_::Array;
                use crate::Native_::MutCell;
                pub trait IComparer_1<T: Clone + 'static> {
                    fn Compare(&self, arg0: T, arg1: T) -> i32;
                }
                impl<V: IComparer_1<T> + Clone + 'static, T: Clone + 'static> IComparer_1<T> for LrcPtr<V> {
                    #[inline]
                    fn Compare(&self, arg0: T, arg1: T) -> i32 {
                        (**self).Compare(arg0, arg1)
                    }
                }
                pub trait IEqualityComparer_1<T: Clone + 'static> {
                    fn Equals(&self, arg0: T, arg1: T) -> bool;
                    fn GetHashCode(&self, arg0: T) -> i32;
                }
                impl<V: IEqualityComparer_1<T> + Clone + 'static, T: Clone + 'static>
                    IEqualityComparer_1<T> for LrcPtr<V>
                {
                    #[inline]
                    fn Equals(&self, arg0: T, arg1: T) -> bool {
                        (**self).Equals(arg0, arg1)
                    }
                    #[inline]
                    fn GetHashCode(&self, arg0: T) -> i32 {
                        (**self).GetHashCode(arg0)
                    }
                }
                pub trait IEnumerator_1<T: Clone + 'static> {
                    fn MoveNext(&self) -> bool;
                    fn Reset(&self);
                    fn get_Current(&self) -> T;
                    fn Dispose(&self);
                }
                impl<V: IEnumerator_1<T> + Clone + 'static, T: Clone + 'static> IEnumerator_1<T> for LrcPtr<V> {
                    #[inline]
                    fn MoveNext(&self) -> bool {
                        (**self).MoveNext()
                    }
                    #[inline]
                    fn Reset(&self) {
                        (**self).Reset()
                    }
                    #[inline]
                    fn get_Current(&self) -> T {
                        (**self).get_Current()
                    }
                    #[inline]
                    fn Dispose(&self) {
                        (**self).Dispose()
                    }
                }
                pub trait IEnumerable_1<T: Clone + 'static> {
                    fn GetEnumerator(
                        &self,
                    ) -> LrcPtr<dyn Interfaces_::System::Collections::Generic::IEnumerator_1<T>>;
                }
                impl<V: IEnumerable_1<T> + Clone + 'static, T: Clone + 'static> IEnumerable_1<T> for LrcPtr<V> {
                    #[inline]
                    fn GetEnumerator(
                        &self,
                    ) -> LrcPtr<dyn Interfaces_::System::Collections::Generic::IEnumerator_1<T>>
                    {
                        (**self).GetEnumerator()
                    }
                }
                pub trait ICollection_1<T: Clone + 'static> {
                    fn Add(&self, arg0: T);
                    fn Clear(&self);
                    fn Contains(&self, arg0: T) -> bool;
                    fn CopyTo(&self, arg0: Array<T>, arg1: i32);
                    fn Remove(&self, arg0: T) -> bool;
                    fn get_Count(&self) -> i32;
                    fn get_IsReadOnly(&self) -> bool;
                }
                impl<V: ICollection_1<T> + Clone + 'static, T: Clone + 'static> ICollection_1<T> for LrcPtr<V> {
                    #[inline]
                    fn Add(&self, arg0: T) {
                        (**self).Add(arg0)
                    }
                    #[inline]
                    fn Clear(&self) {
                        (**self).Clear()
                    }
                    #[inline]
                    fn Contains(&self, arg0: T) -> bool {
                        (**self).Contains(arg0)
                    }
                    #[inline]
                    fn CopyTo(&self, arg0: Array<T>, arg1: i32) {
                        (**self).CopyTo(arg0, arg1)
                    }
                    #[inline]
                    fn Remove(&self, arg0: T) -> bool {
                        (**self).Remove(arg0)
                    }
                    #[inline]
                    fn get_Count(&self) -> i32 {
                        (**self).get_Count()
                    }
                    #[inline]
                    fn get_IsReadOnly(&self) -> bool {
                        (**self).get_IsReadOnly()
                    }
                }
                pub trait IDictionary_2<K: Clone + 'static, V: Clone + 'static> {
                    fn Add(&self, arg0: K, arg1: V);
                    fn ContainsKey(&self, arg0: K) -> bool;
                    fn Remove(&self, arg0: K) -> bool;
                    fn TryGetValue(&self, arg0: K, arg1: &MutCell<V>) -> bool;
                    fn get_Item(&self, arg0: K) -> V;
                    fn get_Keys(
                        &self,
                    ) -> LrcPtr<dyn Interfaces_::System::Collections::Generic::ICollection_1<K>>;
                    fn get_Values(
                        &self,
                    ) -> LrcPtr<dyn Interfaces_::System::Collections::Generic::ICollection_1<V>>;
                }
                impl<
                        V_1: IDictionary_2<K, V> + Clone + 'static,
                        K: Clone + 'static,
                        V: Clone + 'static,
                    > IDictionary_2<K, V> for LrcPtr<V_1>
                {
                    #[inline]
                    fn Add(&self, arg0: K, arg1: V) {
                        (**self).Add(arg0, arg1)
                    }
                    #[inline]
                    fn ContainsKey(&self, arg0: K) -> bool {
                        (**self).ContainsKey(arg0)
                    }
                    #[inline]
                    fn Remove(&self, arg0: K) -> bool {
                        (**self).Remove(arg0)
                    }
                    #[inline]
                    fn TryGetValue(&self, arg0: K, arg1: &MutCell<V>) -> bool {
                        (**self).TryGetValue(arg0, arg1)
                    }
                    #[inline]
                    fn get_Item(&self, arg0: K) -> V {
                        (**self).get_Item(arg0)
                    }
                    #[inline]
                    fn get_Keys(
                        &self,
                    ) -> LrcPtr<dyn Interfaces_::System::Collections::Generic::ICollection_1<K>>
                    {
                        (**self).get_Keys()
                    }
                    #[inline]
                    fn get_Values(
                        &self,
                    ) -> LrcPtr<dyn Interfaces_::System::Collections::Generic::ICollection_1<V>>
                    {
                        (**self).get_Values()
                    }
                }
            }
        }
    }
}
