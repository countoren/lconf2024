pub mod System {
    use super::*;
    pub mod Collections {
        use super::*;
        pub mod Generic {
            use super::*;
            use crate::Array_::copyTo;
            use crate::Array_::fill;
            use crate::Interfaces_::System::Collections::Generic::IComparer_1;
            use crate::Interfaces_::System::Collections::Generic::IEnumerable_1;
            use crate::Interfaces_::System::Collections::Generic::IEnumerator_1;
            use crate::Interfaces_::System::Collections::Generic::IEqualityComparer_1;
            use crate::NativeArray_::add;
            use crate::NativeArray_::count as count_1;
            use crate::NativeArray_::new_init;
            use crate::NativeArray_::new_with_capacity;
            use crate::NativeArray_::Array;
            use crate::Native_::compare;
            use crate::Native_::defaultOf;
            use crate::Native_::getHashCode as getHashCode_1;
            use crate::Native_::Func0;
            use crate::Native_::Func1;
            use crate::Native_::Func2;
            use crate::Native_::LrcPtr;
            use crate::Native_::MutCell;
            use crate::Range_::rangeNumeric;
            use crate::Seq_::delay;
            use crate::Seq_::map;
            use crate::Seq_::toArray;
            use crate::String_::string;
            #[derive(Clone, Debug)]
            pub struct Comparer_1<T: PartialOrd + Clone + 'static> {
                comparison: Func2<T, T, i32>,
            }
            impl<T: PartialOrd + Clone + 'static> System::Collections::Generic::Comparer_1<T> {
                pub fn _ctor__47C913C(
                    comparison: Func2<T, T, i32>,
                ) -> LrcPtr<System::Collections::Generic::Comparer_1<T>> {
                    let comparison_1;
                    ();
                    comparison_1 = comparison;
                    ();
                    LrcPtr::new(System::Collections::Generic::Comparer_1::<T> {
                        comparison: comparison_1,
                    })
                }
                pub fn get_Default() -> LrcPtr<System::Collections::Generic::Comparer_1<T>> {
                    System::Collections::Generic::Comparer_1::_ctor__47C913C(Func2::new(
                        move |e: T, e_1: T| compare(e, e_1),
                    ))
                }
                pub fn Create__47C913C(
                    comparison: Func2<T, T, i32>,
                ) -> LrcPtr<System::Collections::Generic::Comparer_1<T>> {
                    System::Collections::Generic::Comparer_1::_ctor__47C913C(comparison)
                }
                pub fn Compare_5BDDA0(&self, x: T, y: T) -> i32 {
                    let _self_ = self;
                    (_self_.comparison)(x, y)
                }
            }
            impl<T: PartialOrd + Clone + 'static> core::fmt::Display
                for System::Collections::Generic::Comparer_1<T>
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}", core::any::type_name::<Self>())
                }
            }
            impl<T: PartialOrd + Clone + 'static> IComparer_1<T> for Comparer_1<T> {
                fn Compare(&self, x: T, y: T) -> i32 {
                    let _self_ = self;
                    (_self_.comparison)(x, y)
                }
            }
            #[derive(Clone, Debug)]
            pub struct EqualityComparer_1<T: Eq + core::hash::Hash + Clone + 'static> {
                getHashCode: Func1<T, i32>,
                equals: Func2<T, T, bool>,
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static>
                System::Collections::Generic::EqualityComparer_1<T>
            {
                pub fn _ctor__Z6EE254AB(
                    equals: Func2<T, T, bool>,
                    getHashCode: Func1<T, i32>,
                ) -> LrcPtr<System::Collections::Generic::EqualityComparer_1<T>> {
                    let getHashCode_1;
                    let equals_1;
                    ();
                    equals_1 = equals;
                    getHashCode_1 = getHashCode;
                    ();
                    LrcPtr::new(System::Collections::Generic::EqualityComparer_1::<T> {
                        getHashCode: getHashCode_1,
                        equals: equals_1,
                    })
                }
                pub fn get_Default() -> LrcPtr<System::Collections::Generic::EqualityComparer_1<T>>
                {
                    System::Collections::Generic::EqualityComparer_1::_ctor__Z6EE254AB(
                        Func2::new(move |e: T, e_1: T| e == e_1),
                        Func1::new(move |obj: T| getHashCode_1(obj)),
                    )
                }
                pub fn Create__Z6EE254AB(
                    equals: Func2<T, T, bool>,
                    getHashCode: Func1<T, i32>,
                ) -> LrcPtr<System::Collections::Generic::EqualityComparer_1<T>> {
                    System::Collections::Generic::EqualityComparer_1::_ctor__Z6EE254AB(
                        equals,
                        getHashCode,
                    )
                }
                pub fn Equals_5BDDA0(&self, x: T, y: T) -> bool {
                    let _self_ = self;
                    (_self_.equals)(x, y)
                }
                pub fn GetHashCode_2B595(&self, x: T) -> i32 {
                    let _self_ = self;
                    (_self_.getHashCode)(x)
                }
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> core::fmt::Display
                for System::Collections::Generic::EqualityComparer_1<T>
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}", core::any::type_name::<Self>())
                }
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> IEqualityComparer_1<T> for EqualityComparer_1<T> {
                fn Equals(&self, x: T, y: T) -> bool {
                    let _self_ = self;
                    (_self_.equals)(x, y)
                }
                fn GetHashCode(&self, x: T) -> i32 {
                    let _self_ = self;
                    (_self_.getHashCode)(x)
                }
            }
            #[derive(Clone, Debug, Default)]
            pub struct Stack_1<T: Eq + core::hash::Hash + Clone + 'static> {
                contents: MutCell<Array<T>>,
                count: MutCell<i32>,
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> System::Collections::Generic::Stack_1<T> {
                fn _ctor__Z3B4C077E(
                    initialContents: Array<T>,
                    initialCount: i32,
                ) -> LrcPtr<System::Collections::Generic::Stack_1<T>> {
                    let contents: Array<T>;
                    let count: i32;
                    ();
                    contents = initialContents;
                    count = initialCount;
                    ();
                    LrcPtr::new(System::Collections::Generic::Stack_1::<T> {
                        contents: MutCell::new(contents),
                        count: MutCell::new(count),
                    })
                }
                pub fn _ctor__Z524259A4(
                    initialCapacity: i32,
                ) -> LrcPtr<System::Collections::Generic::Stack_1<T>> {
                    System::Collections::Generic::Stack_1::_ctor__Z3B4C077E(
                        new_init(&defaultOf(), initialCapacity),
                        0_i32,
                    )
                }
                pub fn _ctor() -> LrcPtr<System::Collections::Generic::Stack_1<T>> {
                    System::Collections::Generic::Stack_1::_ctor__Z524259A4(4_i32)
                }
                pub fn _ctor__BB573A(
                    xs: LrcPtr<dyn IEnumerable_1<T>>,
                ) -> LrcPtr<System::Collections::Generic::Stack_1<T>> {
                    let arr: Array<T> = toArray(xs);
                    System::Collections::Generic::Stack_1::_ctor__Z3B4C077E(
                        arr.clone(),
                        count_1(arr),
                    )
                }
                pub fn Ensure_Z524259A4(&self, newSize: i32) {
                    let _self_ = self;
                    let oldSize: i32 = count_1(_self_.contents.get().clone());
                    if newSize > oldSize {
                        let old: Array<T> = _self_.contents.get().clone();
                        _self_
                            .contents
                            .set(new_init(&defaultOf(), newSize.max(oldSize * 2_i32)));
                        copyTo(
                            old,
                            0_i32,
                            _self_.contents.get().clone(),
                            0_i32,
                            _self_.count.get().clone(),
                        )
                    }
                }
                pub fn get_Count(&self) -> i32 {
                    let _self_ = self;
                    _self_.count.get().clone()
                }
                pub fn Pop(&self) -> T {
                    let _self_ = self;
                    _self_.count.set(_self_.count.get().clone() - 1_i32);
                    (_self_.contents.get().clone())[_self_.count.get().clone()].clone()
                }
                pub fn Peek(&self) -> T {
                    let _self_ = self;
                    (_self_.contents.get().clone())[_self_.count.get().clone() - 1_i32].clone()
                }
                pub fn Contains_2B595(&self, x: T) -> bool {
                    let _self_ = self;
                    let found: MutCell<bool> = MutCell::new(false);
                    let i: MutCell<i32> = MutCell::new(0_i32);
                    while if i.get().clone() < _self_.count.get().clone() {
                        !found.get().clone()
                    } else {
                        false
                    } {
                        if x.clone() == (_self_.contents.get().clone())[i.get().clone()].clone() {
                            found.set(true)
                        } else {
                            i.set(i.get().clone() + 1_i32)
                        };
                    }
                    found.get().clone()
                }
                pub fn TryPeek_1F3DB691(&self, result: &MutCell<T>) -> bool {
                    let _self_ = self;
                    if _self_.count.get().clone() > 0_i32 {
                        result.set(_self_.Peek());
                        true
                    } else {
                        false
                    }
                }
                pub fn TryPop_1F3DB691(&self, result: &MutCell<T>) -> bool {
                    let _self_ = self;
                    if _self_.count.get().clone() > 0_i32 {
                        result.set(_self_.Pop());
                        true
                    } else {
                        false
                    }
                }
                pub fn Push_2B595(&self, x: T) {
                    let _self_ = self;
                    _self_.Ensure_Z524259A4(_self_.count.get().clone() + 1_i32);
                    (_self_.contents.get().clone()).get_mut()
                        [_self_.count.get().clone() as usize] = x;
                    _self_.count.set(_self_.count.get().clone() + 1_i32)
                }
                pub fn Clear(&self) {
                    let _self_ = self;
                    _self_.count.set(0_i32);
                    fill(
                        _self_.contents.get().clone(),
                        0_i32,
                        count_1(_self_.contents.get().clone()),
                        defaultOf(),
                    )
                }
                pub fn TrimExcess(&self) {
                    let _self_ = self;
                    if _self_.count.get().clone() as f64
                        / count_1(_self_.contents.get().clone()) as f64
                        > 0.9_f64
                    {
                        _self_.Ensure_Z524259A4(_self_.count.get().clone());
                    }
                }
                pub fn ToArray(&self) -> Array<T> {
                    let _self_ = self;
                    let res: Array<T> = new_with_capacity::<T>(_self_.count.get().clone());
                    for i in 0_i32..=_self_.count.get().clone() - 1_i32 {
                        add(
                            res.clone(),
                            (_self_.contents.get().clone())[_self_.count.get().clone() - 1_i32 - i]
                                .clone(),
                        );
                    }
                    res.clone()
                }
                pub fn toSeq(&self) -> LrcPtr<dyn IEnumerable_1<T>> {
                    let _self_ = self;
                    let count: i32 = _self_.count.get().clone();
                    let contents: Array<T> = _self_.contents.get().clone();
                    delay(Func0::new({
                        let contents = contents.clone();
                        let count = count.clone();
                        move || {
                            map(
                                Func1::new({
                                    let contents = contents.clone();
                                    move |i: i32| contents[i].clone()
                                }),
                                rangeNumeric(count - 1_i32, -1_i32, 0_i32),
                            )
                        }
                    }))
                }
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> core::fmt::Display
                for System::Collections::Generic::Stack_1<T>
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}", core::any::type_name::<Self>())
                }
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> IEnumerable_1<T> for Stack_1<T> {
                fn GetEnumerator(&self) -> LrcPtr<dyn IEnumerator_1<T>> {
                    let _self_ = self;
                    IEnumerable_1::GetEnumerator(_self_.toSeq().as_ref())
                }
            }
            #[derive(Clone, Debug, Default)]
            pub struct Queue_1<T: Eq + core::hash::Hash + Clone + 'static> {
                contents: MutCell<Array<T>>,
                count: MutCell<i32>,
                head: MutCell<i32>,
                tail: MutCell<i32>,
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> System::Collections::Generic::Queue_1<T> {
                fn _ctor__Z3B4C077E(
                    initialContents: Array<T>,
                    initialCount: i32,
                ) -> LrcPtr<System::Collections::Generic::Queue_1<T>> {
                    let contents: Array<T>;
                    let count: i32;
                    let head: i32;
                    let tail: i32;
                    ();
                    contents = initialContents;
                    count = initialCount;
                    head = 0_i32;
                    tail = if initialCount == count_1(contents.clone()) {
                        0_i32
                    } else {
                        initialCount
                    };
                    ();
                    LrcPtr::new(System::Collections::Generic::Queue_1::<T> {
                        contents: MutCell::new(contents),
                        count: MutCell::new(count),
                        head: MutCell::new(head),
                        tail: MutCell::new(tail),
                    })
                }
                pub fn _ctor__Z524259A4(
                    initialCapacity: i32,
                ) -> LrcPtr<System::Collections::Generic::Queue_1<T>> {
                    if initialCapacity < 0_i32 {
                        panic!("{}", string("capacity is less than 0"),);
                    }
                    System::Collections::Generic::Queue_1::_ctor__Z3B4C077E(
                        new_init(&defaultOf(), initialCapacity),
                        0_i32,
                    )
                }
                pub fn _ctor() -> LrcPtr<System::Collections::Generic::Queue_1<T>> {
                    System::Collections::Generic::Queue_1::_ctor__Z524259A4(4_i32)
                }
                pub fn _ctor__BB573A(
                    xs: LrcPtr<dyn IEnumerable_1<T>>,
                ) -> LrcPtr<System::Collections::Generic::Queue_1<T>> {
                    let arr: Array<T> = toArray(xs);
                    System::Collections::Generic::Queue_1::_ctor__Z3B4C077E(
                        arr.clone(),
                        count_1(arr),
                    )
                }
                pub fn get_Count(&self) -> i32 {
                    let _self_ = self;
                    _self_.count.get().clone()
                }
                pub fn Enqueue_2B595(&self, value: T) {
                    let _self_ = self;
                    if _self_.count.get().clone() == _self_.size() {
                        _self_.ensure_Z524259A4(_self_.count.get().clone() + 1_i32);
                    }
                    (_self_.contents.get().clone()).get_mut()[_self_.tail.get().clone() as usize] =
                        value;
                    _self_
                        .tail
                        .set((_self_.tail.get().clone() + 1_i32) % _self_.size());
                    _self_.count.set(_self_.count.get().clone() + 1_i32)
                }
                pub fn Dequeue(&self) -> T {
                    let _self_ = self;
                    if _self_.count.get().clone() == 0_i32 {
                        panic!("{}", string("Queue is empty"),);
                    }
                    {
                        let value: T =
                            (_self_.contents.get().clone())[_self_.head.get().clone()].clone();
                        _self_
                            .head
                            .set((_self_.head.get().clone() + 1_i32) % _self_.size());
                        _self_.count.set(_self_.count.get().clone() - 1_i32);
                        value
                    }
                }
                pub fn Peek(&self) -> T {
                    let _self_ = self;
                    if _self_.count.get().clone() == 0_i32 {
                        panic!("{}", string("Queue is empty"),);
                    }
                    (_self_.contents.get().clone())[_self_.head.get().clone()].clone()
                }
                pub fn TryDequeue_1F3DB691(&self, result: &MutCell<T>) -> bool {
                    let _self_ = self;
                    if _self_.count.get().clone() == 0_i32 {
                        false
                    } else {
                        result.set(_self_.Dequeue());
                        true
                    }
                }
                pub fn TryPeek_1F3DB691(&self, result: &MutCell<T>) -> bool {
                    let _self_ = self;
                    if _self_.count.get().clone() == 0_i32 {
                        false
                    } else {
                        result.set(_self_.Peek());
                        true
                    }
                }
                pub fn Contains_2B595(&self, x: T) -> bool {
                    let _self_ = self;
                    let found: MutCell<bool> = MutCell::new(false);
                    let i: MutCell<i32> = MutCell::new(0_i32);
                    while if i.get().clone() < _self_.count.get().clone() {
                        !found.get().clone()
                    } else {
                        false
                    } {
                        if x.clone()
                            == (_self_.contents.get().clone())
                                [_self_.toIndex_Z524259A4(i.get().clone())]
                            .clone()
                        {
                            found.set(true)
                        } else {
                            i.set(i.get().clone() + 1_i32)
                        };
                    }
                    found.get().clone()
                }
                pub fn Clear(&self) {
                    let _self_ = self;
                    _self_.count.set(0_i32);
                    _self_.head.set(0_i32);
                    _self_.tail.set(0_i32);
                    fill(
                        _self_.contents.get().clone(),
                        0_i32,
                        _self_.size(),
                        defaultOf(),
                    )
                }
                pub fn TrimExcess(&self) {
                    let _self_ = self;
                    if _self_.count.get().clone() as f64
                        / count_1(_self_.contents.get().clone()) as f64
                        > 0.9_f64
                    {
                        _self_.ensure_Z524259A4(_self_.count.get().clone());
                    }
                }
                pub fn ToArray(&self) -> Array<T> {
                    let _self_ = self;
                    let res: Array<T> = new_with_capacity::<T>(_self_.count.get().clone());
                    for i in 0_i32..=_self_.count.get().clone() - 1_i32 {
                        add(
                            res.clone(),
                            (_self_.contents.get().clone())[_self_.toIndex_Z524259A4(i)].clone(),
                        );
                    }
                    res.clone()
                }
                pub fn CopyTo_Z3B4C077E(&self, target: Array<T>, start: i32) {
                    let _self_ = self;
                    let i: MutCell<i32> = MutCell::new(start);
                    for i_1 in 0_i32..=_self_.count.get().clone() - 1_i32 {
                        target.get_mut()[(start + i_1) as usize] =
                            (_self_.contents.get().clone())[_self_.toIndex_Z524259A4(i_1)].clone();
                    }
                }
                pub fn size(&self) -> i32 {
                    let _self_ = self;
                    count_1(_self_.contents.get().clone())
                }
                pub fn toIndex_Z524259A4(&self, i: i32) -> i32 {
                    let _self_ = self;
                    (_self_.head.get().clone() + i) % _self_.size()
                }
                pub fn ensure_Z524259A4(&self, requiredSize: i32) {
                    let _self_ = self;
                    let newBuffer: Array<T> = new_init(&defaultOf(), requiredSize);
                    if _self_.head.get().clone() < _self_.tail.get().clone() {
                        copyTo(
                            _self_.contents.get().clone(),
                            _self_.head.get().clone(),
                            newBuffer.clone(),
                            0_i32,
                            _self_.count.get().clone(),
                        )
                    } else {
                        copyTo(
                            _self_.contents.get().clone(),
                            _self_.head.get().clone(),
                            newBuffer.clone(),
                            0_i32,
                            _self_.size() - _self_.head.get().clone(),
                        );
                        copyTo(
                            _self_.contents.get().clone(),
                            0_i32,
                            newBuffer.clone(),
                            _self_.size() - _self_.head.get().clone(),
                            _self_.tail.get().clone(),
                        )
                    }
                    _self_.head.set(0_i32);
                    _self_.contents.set(newBuffer);
                    _self_
                        .tail
                        .set(if _self_.count.get().clone() == _self_.size() {
                            0_i32
                        } else {
                            _self_.count.get().clone()
                        })
                }
                pub fn toSeq(&self) -> LrcPtr<dyn IEnumerable_1<T>> {
                    let _self_ = self;
                    let head: i32 = _self_.head.get().clone();
                    let count: i32 = _self_.count.get().clone();
                    let contents: Array<T> = _self_.contents.get().clone();
                    delay(Func0::new({
                        let contents = contents.clone();
                        let count = count.clone();
                        let head = head.clone();
                        move || {
                            map(
                                Func1::new({
                                    let contents = contents.clone();
                                    move |i: i32| {
                                        contents[(head + i) % count_1(contents.clone())].clone()
                                    }
                                }),
                                rangeNumeric(0_i32, 1_i32, count - 1_i32),
                            )
                        }
                    }))
                }
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> core::fmt::Display
                for System::Collections::Generic::Queue_1<T>
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}", core::any::type_name::<Self>())
                }
            }
            impl<T: Eq + core::hash::Hash + Clone + 'static> IEnumerable_1<T> for Queue_1<T> {
                fn GetEnumerator(&self) -> LrcPtr<dyn IEnumerator_1<T>> {
                    let _self_ = self;
                    IEnumerable_1::GetEnumerator(_self_.toSeq().as_ref())
                }
            }
        }
    }
}
