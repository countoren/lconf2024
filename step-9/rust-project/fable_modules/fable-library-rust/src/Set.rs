pub mod Set_ {
    use super::*;
    use crate::Array_::fold as fold_2;
    use crate::Global_::SR;
    use crate::Interfaces_::System::Collections::Generic::IEnumerable_1;
    use crate::List_::cons;
    use crate::List_::empty as empty_1;
    use crate::List_::fold as fold_3;
    use crate::List_::head;
    use crate::List_::isEmpty as isEmpty_1;
    use crate::List_::ofArrayWithTail;
    use crate::List_::singleton as singleton_1;
    use crate::List_::tail;
    use crate::List_::List;
    use crate::NativeArray_::add as add_1;
    use crate::NativeArray_::new_array;
    use crate::NativeArray_::new_with_capacity;
    use crate::NativeArray_::Array;
    use crate::Native_::compare;
    use crate::Native_::Func0;
    use crate::Native_::Func1;
    use crate::Native_::Func2;
    use crate::Native_::LrcPtr;
    use crate::Native_::MutCell;
    use crate::Option_::getValue;
    use crate::Seq_::compareWith;
    use crate::Seq_::delay;
    use crate::Seq_::fold as fold_1;
    use crate::Seq_::reduce;
    use crate::Seq_::unfold;
    use crate::String_::append;
    use crate::String_::string;
    #[derive(Clone, Debug, Default)]
    pub struct SetTree_1<T: Clone + 'static> {
        pub Height: i32,
        pub Key: T,
        pub Left: Set_::Set<T>,
        pub Right: Set_::Set<T>,
    }
    impl<T: Clone + 'static> core::fmt::Display for Set_::SetTree_1<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Set<T: Clone + 'static> {
        pub root: Option<LrcPtr<Set_::SetTree_1<T>>>,
    }
    impl<T: Clone + 'static> core::fmt::Display for Set_::Set<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    fn mkSet<a: Clone + 'static>(root: Option<LrcPtr<Set_::SetTree_1<a>>>) -> Set_::Set<a> {
        Set_::Set::<a> { root: root }
    }
    pub fn empty<T: Clone + 'static>() -> Set_::Set<T> {
        Set_::Set::<T> {
            root: None::<LrcPtr<Set_::SetTree_1<T>>>,
        }
    }
    pub fn isEmpty<T: Clone + 'static>(s: Set_::Set<T>) -> bool {
        s.root.is_none()
    }
    pub fn mkSetTreeLeaf<T: Clone + 'static>(key: T) -> Set_::Set<T> {
        Set_::mkSet(Some(LrcPtr::new(Set_::SetTree_1::<T> {
            Height: 1_i32,
            Key: key,
            Left: Set_::empty(),
            Right: Set_::empty(),
        })))
    }
    pub fn mkSetTreeNode<T: Clone + 'static>(
        key: T,
        left: Set_::Set<T>,
        right: Set_::Set<T>,
        height: i32,
    ) -> Set_::Set<T> {
        Set_::mkSet(Some(LrcPtr::new(Set_::SetTree_1::<T> {
            Height: height,
            Key: key,
            Left: left,
            Right: right,
        })))
    }
    pub fn singleton<T: Clone + 'static>(value: T) -> Set_::Set<T> {
        Set_::mkSetTreeLeaf(value)
    }
    pub fn countAux<T: Clone + 'static>(s: Set_::Set<T>, acc: i32) -> i32 {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    acc + 1_i32
                } else {
                    Set_::countAux(t.Left.clone(), Set_::countAux(t.Right.clone(), acc + 1_i32))
                }
            }
            _ => acc,
        }
    }
    pub fn count<a: Clone + 'static>(s: Set_::Set<a>) -> i32 {
        Set_::countAux(s, 0_i32)
    }
    pub fn mk<T: Clone + 'static>(l: Set_::Set<T>, k: T, r: Set_::Set<T>) -> Set_::Set<T> {
        let hl: i32 = {
            let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = l.root.clone();
            match &matchValue {
                Some(matchValue_0_0) => (matchValue_0_0).Height,
                _ => 0_i32,
            }
        };
        let hr: i32 = {
            let matchValue_1: Option<LrcPtr<Set_::SetTree_1<T>>> = r.root.clone();
            match &matchValue_1 {
                Some(matchValue_1_0_0) => (matchValue_1_0_0).Height,
                _ => 0_i32,
            }
        };
        let m: i32 = if hl < hr { hr } else { hl };
        if m == 0_i32 {
            Set_::mkSetTreeLeaf(k.clone())
        } else {
            Set_::mkSetTreeNode(k, l, r, m + 1_i32)
        }
    }
    pub fn rebalance<T: Clone + 'static>(t1: Set_::Set<T>, v: T, t2: Set_::Set<T>) -> Set_::Set<T> {
        let t1h: i32 = {
            let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = t1.root.clone();
            match &matchValue {
                Some(matchValue_0_0) => (matchValue_0_0).Height,
                _ => 0_i32,
            }
        };
        let t2h: i32 = {
            let matchValue_1: Option<LrcPtr<Set_::SetTree_1<T>>> = t2.root.clone();
            match &matchValue_1 {
                Some(matchValue_1_0_0) => (matchValue_1_0_0).Height,
                _ => 0_i32,
            }
        };
        if t2h > t1h + 2_i32 {
            let t2_: LrcPtr<Set_::SetTree_1<T>> = getValue(t2.root.clone());
            if {
                let matchValue_2: Option<LrcPtr<Set_::SetTree_1<T>>> = (t2_.Left).root.clone();
                match &matchValue_2 {
                    Some(matchValue_2_0_0) => (matchValue_2_0_0).Height,
                    _ => 0_i32,
                }
            } > t1h + 1_i32
            {
                let t2l: LrcPtr<Set_::SetTree_1<T>> = getValue((t2_.Left).root.clone());
                Set_::mk(
                    Set_::mk(t1.clone(), v.clone(), t2l.Left.clone()),
                    t2l.Key.clone(),
                    Set_::mk(t2l.Right.clone(), t2_.Key.clone(), t2_.Right.clone()),
                )
            } else {
                Set_::mk(
                    Set_::mk(t1.clone(), v.clone(), t2_.Left.clone()),
                    t2_.Key.clone(),
                    t2_.Right.clone(),
                )
            }
        } else {
            if t1h > t2h + 2_i32 {
                let t1_: LrcPtr<Set_::SetTree_1<T>> = getValue(t1.root.clone());
                if {
                    let matchValue_3: Option<LrcPtr<Set_::SetTree_1<T>>> = (t1_.Right).root.clone();
                    match &matchValue_3 {
                        Some(matchValue_3_0_0) => (matchValue_3_0_0).Height,
                        _ => 0_i32,
                    }
                } > t2h + 1_i32
                {
                    let t1r: LrcPtr<Set_::SetTree_1<T>> = getValue((t1_.Right).root.clone());
                    Set_::mk(
                        Set_::mk(t1_.Left.clone(), t1_.Key.clone(), t1r.Left.clone()),
                        t1r.Key.clone(),
                        Set_::mk(t1r.Right.clone(), v.clone(), t2.clone()),
                    )
                } else {
                    Set_::mk(
                        t1_.Left.clone(),
                        t1_.Key.clone(),
                        Set_::mk(t1_.Right.clone(), v.clone(), t2.clone()),
                    )
                }
            } else {
                Set_::mk(t1, v, t2)
            }
        }
    }
    pub fn add<T: PartialOrd + Clone + 'static>(k: T, s: Set_::Set<T>) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                let c: i32 = compare(k.clone(), t.Key.clone());
                if t.Height == 1_i32 {
                    if c < 0_i32 {
                        Set_::mkSetTreeNode(k.clone(), Set_::empty(), s.clone(), 2_i32)
                    } else {
                        if c == 0_i32 {
                            s.clone()
                        } else {
                            Set_::mkSetTreeNode(k.clone(), s.clone(), Set_::empty(), 2_i32)
                        }
                    }
                } else {
                    if c < 0_i32 {
                        Set_::rebalance(
                            Set_::add(k.clone(), t.Left.clone()),
                            t.Key.clone(),
                            t.Right.clone(),
                        )
                    } else {
                        if c == 0_i32 {
                            s.clone()
                        } else {
                            Set_::rebalance(
                                t.Left.clone(),
                                t.Key.clone(),
                                Set_::add(k.clone(), t.Right.clone()),
                            )
                        }
                    }
                }
            }
            _ => Set_::mkSetTreeLeaf(k.clone()),
        }
    }
    pub fn balance<T: PartialOrd + Clone + 'static>(
        s1: Set_::Set<T>,
        k: T,
        s2: Set_::Set<T>,
    ) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s1.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t1: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                let matchValue_1: Option<LrcPtr<Set_::SetTree_1<T>>> = s2.root.clone();
                match &matchValue_1 {
                    Some(matchValue_1_0_0) => {
                        let t2: LrcPtr<Set_::SetTree_1<T>> = matchValue_1_0_0.clone();
                        if t1.Height == 1_i32 {
                            Set_::add(k.clone(), Set_::add(t1.Key.clone(), s2.clone()))
                        } else {
                            if t2.Height == 1_i32 {
                                Set_::add(k.clone(), Set_::add(t2.Key.clone(), s1.clone()))
                            } else {
                                if t1.Height + 2_i32 < t2.Height {
                                    Set_::rebalance(
                                        Set_::balance(s1.clone(), k.clone(), t2.Left.clone()),
                                        t2.Key.clone(),
                                        t2.Right.clone(),
                                    )
                                } else {
                                    if t2.Height + 2_i32 < t1.Height {
                                        Set_::rebalance(
                                            t1.Left.clone(),
                                            t1.Key.clone(),
                                            Set_::balance(t1.Right.clone(), k.clone(), s2.clone()),
                                        )
                                    } else {
                                        Set_::mk(s1.clone(), k.clone(), s2.clone())
                                    }
                                }
                            }
                        }
                    }
                    _ => Set_::add(k.clone(), s1.clone()),
                }
            }
            _ => Set_::add(k.clone(), s2.clone()),
        }
    }
    pub fn split<T: PartialOrd + Clone + 'static>(
        pivot: T,
        s: Set_::Set<T>,
    ) -> LrcPtr<(Set_::Set<T>, bool, Set_::Set<T>)> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    let c: i32 = compare(t.Key.clone(), pivot.clone());
                    if c < 0_i32 {
                        LrcPtr::new((s.clone(), false, Set_::empty()))
                    } else {
                        if c == 0_i32 {
                            LrcPtr::new((Set_::empty(), true, Set_::empty()))
                        } else {
                            LrcPtr::new((Set_::empty(), false, s.clone()))
                        }
                    }
                } else {
                    let c_1: i32 = compare(pivot.clone(), t.Key.clone());
                    if c_1 < 0_i32 {
                        let patternInput: LrcPtr<(Set_::Set<T>, bool, Set_::Set<T>)> =
                            Set_::split(pivot.clone(), t.Left.clone());
                        LrcPtr::new((
                            patternInput.0.clone(),
                            patternInput.1.clone(),
                            Set_::balance(patternInput.2.clone(), t.Key.clone(), t.Right.clone()),
                        ))
                    } else {
                        if c_1 == 0_i32 {
                            LrcPtr::new((t.Left.clone(), true, t.Right.clone()))
                        } else {
                            let patternInput_1: LrcPtr<(Set_::Set<T>, bool, Set_::Set<T>)> =
                                Set_::split(pivot.clone(), t.Right.clone());
                            LrcPtr::new((
                                Set_::balance(
                                    t.Left.clone(),
                                    t.Key.clone(),
                                    patternInput_1.0.clone(),
                                ),
                                patternInput_1.1.clone(),
                                patternInput_1.2.clone(),
                            ))
                        }
                    }
                }
            }
            _ => LrcPtr::new((Set_::empty(), false, Set_::empty())),
        }
    }
    pub fn spliceOutSuccessor<T: Clone + 'static>(s: Set_::Set<T>) -> LrcPtr<(T, Set_::Set<T>)> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    LrcPtr::new((t.Key.clone(), Set_::empty()))
                } else {
                    if Set_::isEmpty(t.Left.clone()) {
                        LrcPtr::new((t.Key.clone(), t.Right.clone()))
                    } else {
                        let patternInput: LrcPtr<(T, Set_::Set<T>)> =
                            Set_::spliceOutSuccessor(t.Left.clone());
                        LrcPtr::new((
                            patternInput.0.clone(),
                            Set_::mk(patternInput.1.clone(), t.Key.clone(), t.Right.clone()),
                        ))
                    }
                }
            }
            _ => panic!("{}", string("internal error: Set.spliceOutSuccessor"),),
        }
    }
    pub fn remove<T: PartialOrd + Clone + 'static>(k: T, s: Set_::Set<T>) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                let c: i32 = compare(k.clone(), t.Key.clone());
                if t.Height == 1_i32 {
                    if c == 0_i32 {
                        Set_::empty()
                    } else {
                        s.clone()
                    }
                } else {
                    if c < 0_i32 {
                        Set_::rebalance(
                            Set_::remove(k.clone(), t.Left.clone()),
                            t.Key.clone(),
                            t.Right.clone(),
                        )
                    } else {
                        if c == 0_i32 {
                            if Set_::isEmpty(t.Left.clone()) {
                                t.Right.clone()
                            } else {
                                if Set_::isEmpty(t.Right.clone()) {
                                    t.Left.clone()
                                } else {
                                    let patternInput: LrcPtr<(T, Set_::Set<T>)> =
                                        Set_::spliceOutSuccessor(t.Right.clone());
                                    Set_::mk(
                                        t.Left.clone(),
                                        patternInput.0.clone(),
                                        patternInput.1.clone(),
                                    )
                                }
                            }
                        } else {
                            Set_::rebalance(
                                t.Left.clone(),
                                t.Key.clone(),
                                Set_::remove(k.clone(), t.Right.clone()),
                            )
                        }
                    }
                }
            }
            _ => s.clone(),
        }
    }
    pub fn contains<T: PartialOrd + Clone + 'static>(k: T, s: Set_::Set<T>) -> bool {
        let k: MutCell<T> = MutCell::new(k.clone());
        let s: MutCell<Set_::Set<T>> = MutCell::new(s);
        '_contains: loop {
            break '_contains ({
                let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                        let c: i32 = compare(k.get().clone(), t.Key.clone());
                        if t.Height == 1_i32 {
                            c == 0_i32
                        } else {
                            if c < 0_i32 {
                                let k_temp: T = k.get().clone();
                                let s_temp: Set_::Set<T> = t.Left.clone();
                                k.set(k_temp);
                                s.set(s_temp);
                                continue '_contains;
                            } else {
                                if c == 0_i32 {
                                    true
                                } else {
                                    let k_temp: T = k.get().clone();
                                    let s_temp: Set_::Set<T> = t.Right.clone();
                                    k.set(k_temp);
                                    s.set(s_temp);
                                    continue '_contains;
                                }
                            }
                        }
                    }
                    _ => false,
                }
            });
        }
    }
    pub fn iterate<T: Clone + 'static>(f: Func1<T, ()>, s: Set_::Set<T>) {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone())
                } else {
                    Set_::iterate(f.clone(), t.Left.clone());
                    f(t.Key.clone());
                    Set_::iterate(f.clone(), t.Right.clone())
                }
            }
            _ => (),
        }
    }
    pub fn foldBack<T: Clone + 'static, a: Clone + 'static>(
        f: Func2<T, a, a>,
        s: Set_::Set<T>,
        x: a,
    ) -> a {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone(), x.clone())
                } else {
                    Set_::foldBack(
                        f.clone(),
                        t.Left.clone(),
                        f(
                            t.Key.clone(),
                            Set_::foldBack(f.clone(), t.Right.clone(), x.clone()),
                        ),
                    )
                }
            }
            _ => x.clone(),
        }
    }
    pub fn fold<a: Clone + 'static, T: Clone + 'static>(
        f: Func2<a, T, a>,
        x: a,
        s: Set_::Set<T>,
    ) -> a {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(x.clone(), t.Key.clone())
                } else {
                    Set_::fold(
                        f.clone(),
                        f(
                            Set_::fold(f.clone(), x.clone(), t.Left.clone()),
                            t.Key.clone(),
                        ),
                        t.Right.clone(),
                    )
                }
            }
            _ => x.clone(),
        }
    }
    pub fn map<T: Clone + 'static, a: PartialOrd + Clone + 'static>(
        mapping: Func1<T, a>,
        s: Set_::Set<T>,
    ) -> Set_::Set<a> {
        Set_::fold(
            Func2::new({
                let mapping = mapping.clone();
                move |acc: Set_::Set<a>, k: T| Set_::add(mapping(k), acc)
            }),
            Set_::empty(),
            s,
        )
    }
    pub fn forAll<T: Clone + 'static>(f: Func1<T, bool>, s: Set_::Set<T>) -> bool {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone())
                } else {
                    if if f(t.Key.clone()) {
                        Set_::forAll(f.clone(), t.Left.clone())
                    } else {
                        false
                    } {
                        Set_::forAll(f.clone(), t.Right.clone())
                    } else {
                        false
                    }
                }
            }
            _ => true,
        }
    }
    pub fn exists<T: Clone + 'static>(f: Func1<T, bool>, s: Set_::Set<T>) -> bool {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone())
                } else {
                    if if f(t.Key.clone()) {
                        true
                    } else {
                        Set_::exists(f.clone(), t.Left.clone())
                    } {
                        true
                    } else {
                        Set_::exists(f.clone(), t.Right.clone())
                    }
                }
            }
            _ => false,
        }
    }
    pub fn isSubset<a: PartialOrd + Clone + 'static>(a: Set_::Set<a>, b: Set_::Set<a>) -> bool {
        Set_::forAll(
            Func1::new({
                let b = b.clone();
                move |x: a| Set_::contains(x, b.clone())
            }),
            a,
        )
    }
    pub fn isSuperset<a: PartialOrd + Clone + 'static>(a: Set_::Set<a>, b: Set_::Set<a>) -> bool {
        Set_::isSubset(b, a)
    }
    pub fn isProperSubset<a: PartialOrd + Clone + 'static>(
        a: Set_::Set<a>,
        b: Set_::Set<a>,
    ) -> bool {
        if Set_::forAll(
            Func1::new({
                let b = b.clone();
                move |x: a| Set_::contains(x, b.clone())
            }),
            a.clone(),
        ) {
            Set_::exists(
                Func1::new({
                    let a = a.clone();
                    move |x_1: a| !Set_::contains(x_1, a.clone())
                }),
                b.clone(),
            )
        } else {
            false
        }
    }
    pub fn isProperSuperset<a: PartialOrd + Clone + 'static>(
        a: Set_::Set<a>,
        b: Set_::Set<a>,
    ) -> bool {
        Set_::isProperSubset(b, a)
    }
    pub fn filterAux<T: PartialOrd + Clone + 'static>(
        f: Func1<T, bool>,
        s: Set_::Set<T>,
        acc: Set_::Set<T>,
    ) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    if f(t.Key.clone()) {
                        Set_::add(t.Key.clone(), acc.clone())
                    } else {
                        acc.clone()
                    }
                } else {
                    Set_::filterAux(
                        f.clone(),
                        t.Left.clone(),
                        Set_::filterAux(
                            f.clone(),
                            t.Right.clone(),
                            if f(t.Key.clone()) {
                                Set_::add(t.Key.clone(), acc.clone())
                            } else {
                                acc.clone()
                            },
                        ),
                    )
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn filter<a: PartialOrd + Clone + 'static>(
        f: Func1<a, bool>,
        s: Set_::Set<a>,
    ) -> Set_::Set<a> {
        Set_::filterAux(f, s, Set_::empty())
    }
    pub fn diffAux<T: PartialOrd + Clone + 'static>(
        s: Set_::Set<T>,
        acc: Set_::Set<T>,
    ) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = acc.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let _acc: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                let matchValue_1: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
                match &matchValue_1 {
                    Some(matchValue_1_0_0) => {
                        let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_1_0_0.clone();
                        if t.Height == 1_i32 {
                            Set_::remove(t.Key.clone(), acc.clone())
                        } else {
                            Set_::diffAux(
                                t.Left.clone(),
                                Set_::diffAux(
                                    t.Right.clone(),
                                    Set_::remove(t.Key.clone(), acc.clone()),
                                ),
                            )
                        }
                    }
                    _ => acc.clone(),
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn difference<a: PartialOrd + Clone + 'static>(
        a: Set_::Set<a>,
        b: Set_::Set<a>,
    ) -> Set_::Set<a> {
        Set_::diffAux(b, a)
    }
    pub fn r#union<T: PartialOrd + Clone + 'static>(
        s1: Set_::Set<T>,
        s2: Set_::Set<T>,
    ) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s1.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t1: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                let matchValue_1: Option<LrcPtr<Set_::SetTree_1<T>>> = s2.root.clone();
                match &matchValue_1 {
                    Some(matchValue_1_0_0) => {
                        let t2: LrcPtr<Set_::SetTree_1<T>> = matchValue_1_0_0.clone();
                        if t1.Height == 1_i32 {
                            Set_::add(t1.Key.clone(), s2.clone())
                        } else {
                            if t2.Height == 1_i32 {
                                Set_::add(t2.Key.clone(), s1.clone())
                            } else {
                                if t1.Height > t2.Height {
                                    let patternInput: LrcPtr<(Set_::Set<T>, bool, Set_::Set<T>)> =
                                        Set_::split(t1.Key.clone(), s2.clone());
                                    Set_::balance(
                                        Set_::r#union(t1.Left.clone(), patternInput.0.clone()),
                                        t1.Key.clone(),
                                        Set_::r#union(t1.Right.clone(), patternInput.2.clone()),
                                    )
                                } else {
                                    let patternInput_1: LrcPtr<(Set_::Set<T>, bool, Set_::Set<T>)> =
                                        Set_::split(t2.Key.clone(), s1.clone());
                                    Set_::balance(
                                        Set_::r#union(t2.Left.clone(), patternInput_1.0.clone()),
                                        t2.Key.clone(),
                                        Set_::r#union(t2.Right.clone(), patternInput_1.2.clone()),
                                    )
                                }
                            }
                        }
                    }
                    _ => s1.clone(),
                }
            }
            _ => s2.clone(),
        }
    }
    pub fn unionMany<T: PartialOrd + Clone + 'static>(
        sets: LrcPtr<dyn IEnumerable_1<Set_::Set<T>>>,
    ) -> Set_::Set<T> {
        fold_1(
            Func2::new(move |s: Set_::Set<T>, s_1: Set_::Set<T>| Set_::r#union(s, s_1)),
            Set_::empty(),
            sets,
        )
    }
    pub fn intersectionAux<T: PartialOrd + Clone + 'static>(
        b: Set_::Set<T>,
        s: Set_::Set<T>,
        acc: Set_::Set<T>,
    ) -> Set_::Set<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    if Set_::contains(t.Key.clone(), b.clone()) {
                        Set_::add(t.Key.clone(), acc.clone())
                    } else {
                        acc.clone()
                    }
                } else {
                    let acc_1: Set_::Set<T> =
                        Set_::intersectionAux(b.clone(), t.Right.clone(), acc.clone());
                    Set_::intersectionAux(
                        b.clone(),
                        t.Left.clone(),
                        if Set_::contains(t.Key.clone(), b.clone()) {
                            Set_::add(t.Key.clone(), acc_1.clone())
                        } else {
                            acc_1
                        },
                    )
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn intersect<a: PartialOrd + Clone + 'static>(
        a: Set_::Set<a>,
        b: Set_::Set<a>,
    ) -> Set_::Set<a> {
        if Set_::isEmpty(b.clone()) {
            b.clone()
        } else {
            Set_::intersectionAux(b, a, Set_::empty())
        }
    }
    pub fn intersectMany<T: PartialOrd + Clone + 'static>(
        sets: LrcPtr<dyn IEnumerable_1<Set_::Set<T>>>,
    ) -> Set_::Set<T> {
        reduce(
            Func2::new(move |a: Set_::Set<T>, b: Set_::Set<T>| Set_::intersect(a, b)),
            sets,
        )
    }
    pub fn partition1<a: PartialOrd + Clone + 'static>(
        f: Func1<a, bool>,
        k: a,
        acc1: Set_::Set<a>,
        acc2: Set_::Set<a>,
    ) -> LrcPtr<(Set_::Set<a>, Set_::Set<a>)> {
        if f(k.clone()) {
            LrcPtr::new((Set_::add(k.clone(), acc1.clone()), acc2.clone()))
        } else {
            LrcPtr::new((acc1, Set_::add(k, acc2)))
        }
    }
    pub fn partitionAux<T: PartialOrd + Clone + 'static>(
        f: Func1<T, bool>,
        s: Set_::Set<T>,
        acc_: Set_::Set<T>,
        acc__1: Set_::Set<T>,
    ) -> LrcPtr<(Set_::Set<T>, Set_::Set<T>)> {
        let acc: LrcPtr<(Set_::Set<T>, Set_::Set<T>)> = LrcPtr::new((acc_, acc__1));
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Set_::partition1(f.clone(), t.Key.clone(), acc.0.clone(), acc.1.clone())
                } else {
                    let acc_3: LrcPtr<(Set_::Set<T>, Set_::Set<T>)> = Set_::partitionAux(
                        f.clone(),
                        t.Right.clone(),
                        acc.0.clone(),
                        acc.1.clone(),
                    );
                    let acc_6: LrcPtr<(Set_::Set<T>, Set_::Set<T>)> = Set_::partition1(
                        f.clone(),
                        t.Key.clone(),
                        acc_3.0.clone(),
                        acc_3.1.clone(),
                    );
                    Set_::partitionAux(f.clone(), t.Left.clone(), acc_6.0.clone(), acc_6.1.clone())
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn partition<a: PartialOrd + Clone + 'static>(
        f: Func1<a, bool>,
        s: Set_::Set<a>,
    ) -> LrcPtr<(Set_::Set<a>, Set_::Set<a>)> {
        Set_::partitionAux(f, s, Set_::empty(), Set_::empty())
    }
    pub fn minimumElementAux<T: Clone + 'static>(s: Set_::Set<T>, n: T) -> T {
        let s: MutCell<Set_::Set<T>> = MutCell::new(s);
        let n: MutCell<T> = MutCell::new(n);
        '_minimumElementAux: loop {
            break '_minimumElementAux ({
                let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                        if t.Height == 1_i32 {
                            t.Key.clone()
                        } else {
                            let s_temp: Set_::Set<T> = t.Left.clone();
                            let n_temp: T = t.Key.clone();
                            s.set(s_temp);
                            n.set(n_temp);
                            continue '_minimumElementAux;
                        }
                    }
                    _ => n.get().clone(),
                }
            });
        }
    }
    pub fn minimumElementOpt<T: Clone + 'static>(s: Set_::Set<T>) -> Option<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Some(t.Key.clone())
                } else {
                    Some(Set_::minimumElementAux(t.Left.clone(), t.Key.clone()))
                }
            }
            _ => None::<T>,
        }
    }
    pub fn maximumElementAux<T: Clone + 'static>(s: Set_::Set<T>, n: T) -> T {
        let s: MutCell<Set_::Set<T>> = MutCell::new(s);
        let n: MutCell<T> = MutCell::new(n);
        '_maximumElementAux: loop {
            break '_maximumElementAux ({
                let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                        if t.Height == 1_i32 {
                            t.Key.clone()
                        } else {
                            let s_temp: Set_::Set<T> = t.Right.clone();
                            let n_temp: T = t.Key.clone();
                            s.set(s_temp);
                            n.set(n_temp);
                            continue '_maximumElementAux;
                        }
                    }
                    _ => n.get().clone(),
                }
            });
        }
    }
    pub fn maximumElementOpt<T: Clone + 'static>(s: Set_::Set<T>) -> Option<T> {
        let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> = s.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Some(t.Key.clone())
                } else {
                    Some(Set_::maximumElementAux(t.Right.clone(), t.Key.clone()))
                }
            }
            _ => None::<T>,
        }
    }
    pub fn minElement<a: Clone + 'static>(s: Set_::Set<a>) -> a {
        let matchValue: Option<a> = Set_::minimumElementOpt(s);
        match &matchValue {
            None => panic!(
                "{}",
                append(SR::setContainsNoElements(), string(" (Parameter \'s\')")),
            ),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn maxElement<a: Clone + 'static>(s: Set_::Set<a>) -> a {
        let matchValue: Option<a> = Set_::maximumElementOpt(s);
        match &matchValue {
            None => panic!(
                "{}",
                append(SR::setContainsNoElements(), string(" (Parameter \'s\')")),
            ),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct SetIterator_1<T: PartialOrd + Clone + 'static> {
        pub stack: MutCell<List<Set_::Set<T>>>,
        pub started: MutCell<bool>,
    }
    impl<T: PartialOrd + Clone + 'static> core::fmt::Display for Set_::SetIterator_1<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    pub fn collapseLHS<T: Clone + 'static>(stack: List<Set_::Set<T>>) -> List<Set_::Set<T>> {
        let stack: MutCell<List<Set_::Set<T>>> = MutCell::new(stack.clone());
        '_collapseLHS: loop {
            break '_collapseLHS (if !isEmpty_1(stack.get().clone()) {
                let rest: List<Set_::Set<T>> = tail(stack.get().clone());
                let matchValue: Option<LrcPtr<Set_::SetTree_1<T>>> =
                    (head(stack.get().clone())).root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Set_::SetTree_1<T>> = matchValue_0_0.clone();
                        if t.Height == 1_i32 {
                            stack.get().clone()
                        } else {
                            let stack_temp: List<Set_::Set<T>> = ofArrayWithTail(
                                new_array(&[
                                    t.Left.clone(),
                                    Set_::mkSetTreeLeaf(t.Key.clone()),
                                    t.Right.clone(),
                                ]),
                                rest.clone(),
                            );
                            stack.set(stack_temp);
                            continue '_collapseLHS;
                        }
                    }
                    _ => {
                        let stack_temp: List<Set_::Set<T>> = rest;
                        stack.set(stack_temp);
                        continue '_collapseLHS;
                    }
                }
            } else {
                empty_1::<Set_::Set<T>>()
            });
        }
    }
    pub fn mkIterator<a: PartialOrd + Clone + 'static>(
        s: Set_::Set<a>,
    ) -> LrcPtr<Set_::SetIterator_1<a>> {
        LrcPtr::new(Set_::SetIterator_1::<a> {
            stack: MutCell::new(Set_::collapseLHS(singleton_1(s))),
            started: MutCell::new(false),
        })
    }
    pub fn notStarted<a: Clone + 'static>() -> a {
        panic!("{}", SR::enumerationNotStarted(),)
    }
    pub fn alreadyFinished<a: Clone + 'static>() -> a {
        panic!("{}", SR::enumerationAlreadyFinished(),)
    }
    pub fn current<T: PartialOrd + Clone + 'static>(i: LrcPtr<Set_::SetIterator_1<T>>) -> T {
        if i.started.get().clone() {
            let matchValue: List<Set_::Set<T>> = i.stack.get().clone();
            if !isEmpty_1(matchValue.clone()) {
                if (head(matchValue.clone())).root.is_some() {
                    let k: LrcPtr<Set_::SetTree_1<T>> =
                        getValue((head(matchValue.clone())).root.clone());
                    k.Key.clone()
                } else {
                    Set_::alreadyFinished()
                }
            } else {
                Set_::alreadyFinished()
            }
        } else {
            Set_::notStarted()
        }
    }
    pub fn unexpectedStackForMoveNext<a: Clone + 'static>() -> a {
        panic!(
            "{}",
            string("Please report error: Set iterator, unexpected stack for moveNext"),
        )
    }
    pub fn unexpectedstateInSetTreeCompareStacks<a: Clone + 'static>() -> a {
        panic!("{}", string("unexpected state in SetTree.compareStacks"),)
    }
    pub fn moveNext<T: PartialOrd + Clone + 'static>(i: LrcPtr<Set_::SetIterator_1<T>>) -> bool {
        if i.started.get().clone() {
            let matchValue: List<Set_::Set<T>> = i.stack.get().clone();
            if !isEmpty_1(matchValue.clone()) {
                if (head(matchValue.clone())).root.is_some() {
                    let t: LrcPtr<Set_::SetTree_1<T>> =
                        getValue((head(matchValue.clone())).root.clone());
                    if t.Height == 1_i32 {
                        i.stack.set(Set_::collapseLHS(tail(matchValue.clone())));
                        !isEmpty_1(i.stack.get().clone())
                    } else {
                        Set_::unexpectedStackForMoveNext()
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            i.started.set(true);
            !isEmpty_1(i.stack.get().clone())
        }
    }
    pub fn choose<a: Clone + 'static>(s: Set_::Set<a>) -> a {
        Set_::minElement(s)
    }
    pub fn copyToArray<a: Clone + 'static>(s: Set_::Set<a>, arr: Array<a>, i: i32) {
        let j: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(i));
        Set_::iterate(
            Func1::new({
                let arr = arr.clone();
                let j = j.clone();
                move |x: a| {
                    arr.get_mut()[j.get().clone() as usize] = x;
                    j.set(j.get().clone() + 1_i32)
                }
            }),
            s,
        )
    }
    pub fn toArray<T: Clone + 'static>(s: Set_::Set<T>) -> Array<T> {
        let res: Array<T> = new_with_capacity::<T>(Set_::count(s.clone()));
        Set_::iterate(
            Func1::new({
                let res = res.clone();
                move |x: T| add_1(res.clone(), x)
            }),
            s,
        );
        res.clone()
    }
    pub fn toList<T: Clone + 'static>(s: Set_::Set<T>) -> List<T> {
        Set_::foldBack(
            Func2::new(move |k: T, acc: List<T>| cons(k, acc)),
            s,
            empty_1::<T>(),
        )
    }
    pub fn ofArray<a: PartialOrd + Clone + 'static>(xs: Array<a>) -> Set_::Set<a> {
        fold_2(
            Func2::new(move |acc: Set_::Set<a>, k: a| Set_::add(k, acc)),
            Set_::empty(),
            xs,
        )
    }
    pub fn ofList<a: PartialOrd + Clone + 'static>(xs: List<a>) -> Set_::Set<a> {
        fold_3(
            Func2::new(move |acc: Set_::Set<a>, k: a| Set_::add(k, acc)),
            Set_::empty(),
            xs,
        )
    }
    pub fn ofSeq<a: PartialOrd + Clone + 'static>(
        xs: LrcPtr<dyn IEnumerable_1<a>>,
    ) -> Set_::Set<a> {
        fold_1(
            Func2::new(move |acc: Set_::Set<a>, k: a| Set_::add(k, acc)),
            Set_::empty(),
            xs,
        )
    }
    pub fn toSeq<T: PartialOrd + Clone + 'static>(s: Set_::Set<T>) -> LrcPtr<dyn IEnumerable_1<T>> {
        delay(Func0::new({
            let s = s.clone();
            move || {
                unfold(
                    Func1::new(move |i: LrcPtr<Set_::SetIterator_1<T>>| {
                        if Set_::moveNext(i.clone()) {
                            Some(LrcPtr::new((Set_::current(i.clone()), i)))
                        } else {
                            None::<LrcPtr<(T, LrcPtr<Set_::SetIterator_1<T>>)>>
                        }
                    }),
                    Set_::mkIterator(s.clone()),
                )
            }
        }))
    }
    pub fn compareTo<T: PartialOrd + Clone + 'static>(s1: Set_::Set<T>, s2: Set_::Set<T>) -> i32 {
        compareWith(
            Func2::new(move |e: T, e_1: T| compare(e, e_1)),
            Set_::toSeq(s1),
            Set_::toSeq(s2),
        )
    }
    pub fn equals<T: PartialOrd + Clone + 'static>(s1: Set_::Set<T>, s2: Set_::Set<T>) -> bool {
        Set_::compareTo(s1, s2) == 0_i32
    }
}
