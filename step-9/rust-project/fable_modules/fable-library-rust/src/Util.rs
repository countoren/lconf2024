pub mod Util_ {
    use super::*;
    use crate::Native_::Func0;
    use crate::Native_::Func1;
    use crate::Native_::LrcPtr;
    use crate::Native_::MutCell;
    use crate::String_::string;
    use crate::System::Exception;
    use crate::System::Text::StringBuilder;
    pub fn divRem<T: core::ops::Div<Output = T> + core::ops::Rem<Output = T> + Clone + 'static>(
        x: T,
        y: T,
    ) -> (T, T) {
        (x.clone() / y.clone(), x % y)
    }
    pub fn divRemOut<
        T: core::ops::Div<Output = T> + core::ops::Rem<Output = T> + Clone + 'static,
    >(
        x: T,
        y: T,
        remainder: &MutCell<T>,
    ) -> T {
        let quotient: T = x.clone() / y.clone();
        remainder.set(x % y);
        quotient
    }
    pub fn bprintf(sb: LrcPtr<StringBuilder>) -> Func1<string, ()> {
        Func1::new({
            let sb = sb.clone();
            move |s: string| {
                sb.Append_Z721C83C5(s);
                ()
            }
        })
    }
    pub fn kbprintf<a: Clone + 'static>(
        cont: Func0<a>,
        sb: LrcPtr<StringBuilder>,
    ) -> Func1<string, a> {
        Func1::new({
            let cont = cont.clone();
            let sb = sb.clone();
            move |s: string| {
                sb.Append_Z721C83C5(s);
                cont()
            }
        })
    }
    pub fn sb_Append(sb: LrcPtr<StringBuilder>, s: string) -> LrcPtr<StringBuilder> {
        sb.Append_Z721C83C5(s)
    }
    pub fn new_Exception(msg: string) -> LrcPtr<Exception> {
        Exception::_ctor__Z721C83C5(msg)
    }
}
