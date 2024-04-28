pub mod System {
    use super::*;
    pub mod Text {
        use super::*;
        use crate::NativeArray_::add;
        use crate::NativeArray_::count as count_1;
        use crate::NativeArray_::new_with_capacity;
        use crate::NativeArray_::Array;
        use crate::Native_::fromFluent;
        use crate::Native_::Lrc;
        use crate::Native_::LrcPtr;
        use crate::Native_::MutCell;
        use crate::String_::append;
        use crate::String_::concat;
        use crate::String_::fromChars;
        use crate::String_::getCharAt;
        use crate::String_::getSlice;
        use crate::String_::isEmpty;
        use crate::String_::length;
        use crate::String_::ofBoolean;
        use crate::String_::ofChar;
        use crate::String_::replace;
        use crate::String_::string;
        use crate::String_::substring2;
        use crate::String_::toString;
        #[derive(Clone, Debug, Default)]
        pub struct StringBuilder {
            buf: Array<string>,
        }
        impl System::Text::StringBuilder {
            pub fn _ctor__Z18115A39(
                value: string,
                capacity: i32,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let buf: Array<string>;
                ();
                buf = new_with_capacity::<string>(capacity);
                if !isEmpty(value.clone()) {
                    add(buf.clone(), value);
                }
                ();
                LrcPtr::new(System::Text::StringBuilder { buf: buf })
            }
            pub fn _ctor__Z524259A4(capacity: i32) -> LrcPtr<System::Text::StringBuilder> {
                System::Text::StringBuilder::_ctor__Z18115A39(string(""), capacity)
            }
            pub fn _ctor__Z721C83C5(value: string) -> LrcPtr<System::Text::StringBuilder> {
                System::Text::StringBuilder::_ctor__Z18115A39(value, 16_i32)
            }
            pub fn _ctor() -> LrcPtr<System::Text::StringBuilder> {
                System::Text::StringBuilder::_ctor__Z18115A39(string(""), 16_i32)
            }
            pub fn Append_Z721C83C5(
                self: &Lrc<Self>,
                s: string,
            ) -> LrcPtr<System::Text::StringBuilder> {
                fromFluent({
                    let _self_ = self;
                    add(_self_.buf.clone(), s);
                    _self_.clone()
                })
            }
            pub fn Append_Z1FBCCD16(
                self: &Lrc<Self>,
                o: bool,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(ofBoolean(o))
            }
            pub fn Append_244C7CD6(
                self: &Lrc<Self>,
                c: char,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(ofChar(c))
            }
            pub fn Append_Z510FF069(
                self: &Lrc<Self>,
                o: i8,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_244D3E44(self: &Lrc<Self>, o: u8) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z524259E6(
                self: &Lrc<Self>,
                o: i16,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z6EF82811(
                self: &Lrc<Self>,
                o: u16,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z524259A4(
                self: &Lrc<Self>,
                o: i32,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z6EF827D7(
                self: &Lrc<Self>,
                o: u32,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z524259C1(
                self: &Lrc<Self>,
                o: i64,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z6EF827B6(
                self: &Lrc<Self>,
                o: u64,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_Z7138B98C(
                self: &Lrc<Self>,
                o: f32,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_5E38073B(
                self: &Lrc<Self>,
                o: f64,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(o))
            }
            pub fn Append_487EF8FB(
                self: &Lrc<Self>,
                s: string,
                index: i32,
                count: i32,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(substring2(s, index, count))
            }
            pub fn Append_Z372E4D23(
                self: &Lrc<Self>,
                cs: Array<char>,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(fromChars(cs))
            }
            pub fn Append_43A65C09(
                self: &Lrc<Self>,
                sb: LrcPtr<System::Text::StringBuilder>,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(toString(sb))
            }
            pub fn AppendLine(self: &Lrc<Self>) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                _self_.Append_Z721C83C5(string("\n"))
            }
            pub fn AppendLine_Z721C83C5(
                self: &Lrc<Self>,
                s: string,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                (_self_.Append_Z721C83C5(s)).AppendLine()
            }
            pub fn Clear(self: &Lrc<Self>) -> LrcPtr<System::Text::StringBuilder> {
                fromFluent({
                    let _self_ = self;
                    _self_.buf.get_mut().clear();
                    _self_.clone()
                })
            }
            pub fn get_Chars_Z524259A4(&self, index: i32) -> char {
                let _self_ = self;
                let len: MutCell<i32> = MutCell::new(0_i32);
                let i: MutCell<i32> = MutCell::new(-1_i32);
                while if i.get().clone() + 1_i32 < count_1(_self_.buf.clone()) {
                    len.get().clone() < index
                } else {
                    false
                } {
                    i.set(i.get().clone() + 1_i32);
                    len.set(len.get().clone() + length((_self_.buf)[i.get().clone()].clone()))
                }
                if if if index < 0_i32 {
                    true
                } else {
                    i.get().clone() < 0_i32
                } {
                    true
                } else {
                    i.get().clone() >= count_1(_self_.buf.clone())
                } {
                    panic!("{}", string("Index was outside the bounds of the array"),)
                } else {
                    let pos: i32 = len.get().clone() - index - 1_i32;
                    getCharAt((_self_.buf)[i.get().clone()].clone(), pos)
                }
            }
            pub fn set_Chars_413E0D0A(&self, index: i32, value: char) {
                let _self_ = self;
                let len: MutCell<i32> = MutCell::new(0_i32);
                let i: MutCell<i32> = MutCell::new(-1_i32);
                while if i.get().clone() + 1_i32 < count_1(_self_.buf.clone()) {
                    len.get().clone() < index
                } else {
                    false
                } {
                    i.set(i.get().clone() + 1_i32);
                    len.set(len.get().clone() + length((_self_.buf)[i.get().clone()].clone()))
                }
                if if if index < 0_i32 {
                    true
                } else {
                    i.get().clone() < 0_i32
                } {
                    true
                } else {
                    i.get().clone() >= count_1(_self_.buf.clone())
                } {
                    panic!("{}", string("Index was outside the bounds of the array"),)
                } else {
                    let pos: i32 = len.get().clone() - index - 1_i32;
                    (_self_.buf).get_mut()[i.get().clone() as usize] = append(
                        append(
                            getSlice(
                                (_self_.buf)[i.get().clone()].clone(),
                                Some(0_i32),
                                Some(pos - 1_i32),
                            ),
                            ofChar(value),
                        ),
                        getSlice(
                            (_self_.buf)[i.get().clone()].clone(),
                            Some(pos + 1_i32),
                            None::<i32>,
                        ),
                    )
                }
            }
            pub fn get_Length(&self) -> i32 {
                let _self_ = self;
                let len: MutCell<i32> = MutCell::new(0_i32);
                for i in (0_i32..=count_1(_self_.buf.clone()) - 1_i32).rev() {
                    len.set(len.get().clone() + length((_self_.buf)[i].clone()));
                }
                len.get().clone()
            }
            pub fn Replace_Z766F94C0(
                self: &Lrc<Self>,
                oldValue: char,
                newValue: char,
            ) -> LrcPtr<System::Text::StringBuilder> {
                fromFluent({
                    let _self_ = self;
                    let oldValue_1: string = ofChar(oldValue);
                    let newValue_1: string = ofChar(newValue);
                    for i in (0_i32..=count_1(_self_.buf.clone()) - 1_i32).rev() {
                        (_self_.buf).get_mut()[i as usize] = replace(
                            (_self_.buf)[i].clone(),
                            oldValue_1.clone(),
                            newValue_1.clone(),
                        );
                    }
                    _self_.clone()
                })
            }
            pub fn Replace_Z384F8060(
                self: &Lrc<Self>,
                oldValue: string,
                newValue: string,
            ) -> LrcPtr<System::Text::StringBuilder> {
                let _self_ = self;
                let str: string = replace(toString(_self_.clone()), oldValue, newValue);
                (_self_.Clear()).Append_Z721C83C5(str)
            }
            pub fn ToString_(&self) -> string {
                let _self_ = self;
                concat(_self_.buf.clone())
            }
            pub fn ToString_Z37302880(&self, index: i32, count: i32) -> string {
                let _self_ = self;
                substring2(toString(_self_.clone()), index, count)
            }
        }
        impl core::fmt::Display for System::Text::StringBuilder {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.ToString_())
            }
        }
    }
}
