pub mod System {
    use super::*;
    use crate::Exception_::try_catch;
    use crate::Native_::Func0;
    use crate::Native_::LrcPtr;
    use crate::Native_::MutCell;
    use crate::String_::append;
    use crate::String_::isEmpty;
    use crate::String_::string;
    #[derive(Clone, Debug, Default)]
    pub struct Array {}
    impl System::Array {
        pub fn _ctor() -> LrcPtr<System::Array> {
            ();
            ();
            LrcPtr::new(System::Array {})
        }
    }
    impl core::fmt::Display for System::Array {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Enum {}
    impl System::Enum {
        pub fn _ctor() -> LrcPtr<System::Enum> {
            ();
            ();
            LrcPtr::new(System::Enum {})
        }
    }
    impl core::fmt::Display for System::Enum {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Exception {
        message: string,
    }
    impl System::Exception {
        pub fn _ctor__Z721C83C5(message: string) -> LrcPtr<System::Exception> {
            let message_1: string;
            ();
            message_1 = message;
            ();
            LrcPtr::new(System::Exception { message: message_1 })
        }
        pub fn _ctor() -> LrcPtr<System::Exception> {
            System::Exception::_ctor__Z721C83C5(string(""))
        }
        pub fn get_Message(&self) -> string {
            let _self_ = self;
            if isEmpty(_self_.message.clone()) {
                string("Specified argument was out of the range of valid values.")
            } else {
                _self_.message.clone()
            }
        }
    }
    impl core::fmt::Display for System::Exception {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct InvalidOperationException {
        message: string,
    }
    impl System::InvalidOperationException {
        pub fn _ctor__Z721C83C5(message: string) -> LrcPtr<System::InvalidOperationException> {
            let message_1: string;
            ();
            message_1 = message;
            ();
            LrcPtr::new(System::InvalidOperationException { message: message_1 })
        }
        pub fn _ctor() -> LrcPtr<System::InvalidOperationException> {
            System::InvalidOperationException::_ctor__Z721C83C5(string(""))
        }
        pub fn get_Message(&self) -> string {
            let _self_ = self;
            if isEmpty(_self_.message.clone()) {
                string("Operation is not valid due to the current state of the object.")
            } else {
                _self_.message.clone()
            }
        }
    }
    impl core::fmt::Display for System::InvalidOperationException {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct ArgumentException {
        paramName: string,
        message: string,
    }
    impl System::ArgumentException {
        pub fn _ctor__Z384F8060(
            message: string,
            paramName: string,
        ) -> LrcPtr<System::ArgumentException> {
            let paramName_1: string;
            let message_1: string;
            ();
            message_1 = message;
            paramName_1 = paramName;
            ();
            LrcPtr::new(System::ArgumentException {
                paramName: paramName_1,
                message: message_1,
            })
        }
        pub fn _ctor() -> LrcPtr<System::ArgumentException> {
            System::ArgumentException::_ctor__Z384F8060(string(""), string(""))
        }
        pub fn _ctor__Z721C83C5(message: string) -> LrcPtr<System::ArgumentException> {
            System::ArgumentException::_ctor__Z384F8060(message, string(""))
        }
        pub fn get_Message(&self) -> string {
            let _self_ = self;
            let message: string = if isEmpty(_self_.message.clone()) {
                string("Value does not fall within the expected range.")
            } else {
                _self_.message.clone()
            };
            if isEmpty(_self_.paramName.clone()) {
                message.clone()
            } else {
                append(
                    append(
                        append(message, string(" (Parameter \'")),
                        _self_.paramName.clone(),
                    ),
                    string("\')"),
                )
            }
        }
        pub fn get_ParamName(&self) -> string {
            let _self_ = self;
            _self_.paramName.clone()
        }
    }
    impl core::fmt::Display for System::ArgumentException {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct ArgumentOutOfRangeException {
        paramName: string,
        message: string,
    }
    impl System::ArgumentOutOfRangeException {
        pub fn _ctor__Z384F8060(
            paramName: string,
            message: string,
        ) -> LrcPtr<System::ArgumentOutOfRangeException> {
            let paramName_1: string;
            let message_1: string;
            ();
            paramName_1 = paramName;
            message_1 = message;
            ();
            LrcPtr::new(System::ArgumentOutOfRangeException {
                paramName: paramName_1,
                message: message_1,
            })
        }
        pub fn _ctor() -> LrcPtr<System::ArgumentOutOfRangeException> {
            System::ArgumentOutOfRangeException::_ctor__Z384F8060(string(""), string(""))
        }
        pub fn _ctor__Z721C83C5(paramName: string) -> LrcPtr<System::ArgumentOutOfRangeException> {
            System::ArgumentOutOfRangeException::_ctor__Z384F8060(paramName, string(""))
        }
        pub fn get_Message(&self) -> string {
            let _self_ = self;
            let message: string = if isEmpty(_self_.message.clone()) {
                string("Specified argument was out of the range of valid values.")
            } else {
                _self_.message.clone()
            };
            if isEmpty(_self_.paramName.clone()) {
                message.clone()
            } else {
                append(
                    append(
                        append(message, string(" (Parameter \'")),
                        _self_.paramName.clone(),
                    ),
                    string("\')"),
                )
            }
        }
        pub fn get_ParamName(&self) -> string {
            let _self_ = self;
            _self_.paramName.clone()
        }
    }
    impl core::fmt::Display for System::ArgumentOutOfRangeException {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug)]
    pub enum LazyState_1<T: Clone + 'static> {
        Initial(Func0<T>),
        Success(T),
        Failure(LrcPtr<crate::System::Exception>),
    }
    impl<T: Clone + 'static> core::fmt::Display for System::LazyState_1<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug)]
    pub struct Lazy_1<T: Clone + 'static> {
        lazyState: MutCell<LrcPtr<System::LazyState_1<T>>>,
    }
    impl<T: Clone + 'static> System::Lazy_1<T> {
        pub fn _ctor__Z62F44226(
            state: LrcPtr<System::LazyState_1<T>>,
            isThreadSafe: bool,
        ) -> LrcPtr<System::Lazy_1<T>> {
            let lazyState: LrcPtr<System::LazyState_1<T>>;
            ();
            lazyState = state;
            ();
            LrcPtr::new(System::Lazy_1::<T> {
                lazyState: MutCell::new(lazyState),
            })
        }
        pub fn _ctor__2B595(f: Func0<T>) -> LrcPtr<System::Lazy_1<T>> {
            System::Lazy_1::_ctor__Z62F44226(
                LrcPtr::new(System::LazyState_1::Initial::<T>(Func0::new({
                    let f = f.clone();
                    move || f()
                }))),
                true,
            )
        }
        pub fn _ctor__Z1FE5A521(f: Func0<T>, isThreadSafe: bool) -> LrcPtr<System::Lazy_1<T>> {
            System::Lazy_1::_ctor__Z62F44226(
                LrcPtr::new(System::LazyState_1::Initial::<T>(Func0::new({
                    let f = f.clone();
                    move || f()
                }))),
                isThreadSafe,
            )
        }
        pub fn get_IsValueCreated(&self) -> bool {
            let _self_ = self;
            if let System::LazyState_1::Success(_) = _self_.lazyState.get().clone().as_ref() {
                true
            } else {
                false
            }
        }
        pub fn Force(&self) -> T {
            let _self_ = self;
            let matchValue: LrcPtr<System::LazyState_1<T>> = _self_.lazyState.get().clone();
            match matchValue.as_ref() {
                System::LazyState_1::Failure(matchValue_2_0) => {
                    panic!("{}", matchValue_2_0.get_Message(),)
                }
                System::LazyState_1::Initial(matchValue_0_0) => try_catch(
                    || {
                        let v_1: T = (matchValue_0_0)();
                        _self_
                            .lazyState
                            .set(LrcPtr::new(System::LazyState_1::Success::<T>(v_1.clone())));
                        v_1
                    },
                    |e_1: LrcPtr<crate::System::Exception>| {
                        _self_
                            .lazyState
                            .set(LrcPtr::new(System::LazyState_1::Failure::<T>(e_1.clone())));
                        ();
                        panic!("{}", e_1.get_Message(),)
                    },
                ),
                System::LazyState_1::Success(matchValue_1_0) => matchValue_1_0.clone(),
            }
        }
        pub fn get_Value(&self) -> T {
            let _self_ = self;
            _self_.Force()
        }
    }
    impl<T: Clone + 'static> core::fmt::Display for System::Lazy_1<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
}
