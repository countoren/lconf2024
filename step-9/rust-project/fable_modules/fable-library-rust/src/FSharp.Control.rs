pub mod FSharp {
    use super::*;
    pub mod Control {
        use super::*;
        pub mod LazyExtensions {
            use super::*;
            use crate::Native_::Func0;
            use crate::Native_::LrcPtr;
            use crate::System::Lazy_1;
            pub fn Create<T: Clone + 'static>(f: Func0<T>) -> LrcPtr<crate::System::Lazy_1<T>> {
                Lazy_1::_ctor__Z1FE5A521(f, true)
            }
            pub fn CreateFromValue<T: Clone + 'static>(v: T) -> LrcPtr<crate::System::Lazy_1<T>> {
                Lazy_1::_ctor__Z1FE5A521(
                    Func0::new({
                        let v = v.clone();
                        move || v.clone()
                    }),
                    true,
                )
            }
        }
    }
}
