pub mod Map_ {
    use super::*;
    use crate::Array_::fold as fold_1;
    use crate::Global_::SR;
    use crate::Interfaces_::System::Collections::Generic::IEnumerable_1;
    use crate::List_::cons;
    use crate::List_::empty as empty_1;
    use crate::List_::fold as fold_2;
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
    use crate::Native_::defaultOf;
    use crate::Native_::Func0;
    use crate::Native_::Func1;
    use crate::Native_::Func2;
    use crate::Native_::Func3;
    use crate::Native_::LrcPtr;
    use crate::Native_::MutCell;
    use crate::Option_::getValue;
    use crate::Seq_::compareWith;
    use crate::Seq_::delay;
    use crate::Seq_::fold as fold_3;
    use crate::Seq_::unfold;
    use crate::String_::string;
    #[derive(Clone, Debug, Default)]
    pub struct MapTree_2<K: Clone + 'static, V: Clone + 'static> {
        pub Height: i32,
        pub Key: K,
        pub Value: V,
        pub Left: Map_::Map<K, V>,
        pub Right: Map_::Map<K, V>,
    }
    impl<K: Clone + 'static, V: Clone + 'static> core::fmt::Display for Map_::MapTree_2<K, V> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Map<K: Clone + 'static, V: Clone + 'static> {
        pub root: Option<LrcPtr<Map_::MapTree_2<K, V>>>,
    }
    impl<K: Clone + 'static, V: Clone + 'static> core::fmt::Display for Map_::Map<K, V> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    fn mkMap<a: Clone + 'static, b: Clone + 'static>(
        root: Option<LrcPtr<Map_::MapTree_2<a, b>>>,
    ) -> Map_::Map<a, b> {
        Map_::Map::<a, b> { root: root }
    }
    pub fn empty<K: Clone + 'static, V: Clone + 'static>() -> Map_::Map<K, V> {
        Map_::Map::<K, V> {
            root: None::<LrcPtr<Map_::MapTree_2<K, V>>>,
        }
    }
    pub fn isEmpty<K: Clone + 'static, V: Clone + 'static>(m: Map_::Map<K, V>) -> bool {
        m.root.is_none()
    }
    pub fn mkMapTreeLeaf<K: Clone + 'static, V: Clone + 'static>(k: K, v: V) -> Map_::Map<K, V> {
        Map_::mkMap(Some(LrcPtr::new(Map_::MapTree_2::<K, V> {
            Height: 1_i32,
            Key: k,
            Value: v,
            Left: Map_::empty(),
            Right: Map_::empty(),
        })))
    }
    pub fn mkMapTreeNode<K: Clone + 'static, V: Clone + 'static>(
        k: K,
        v: V,
        left: Map_::Map<K, V>,
        right: Map_::Map<K, V>,
        h: i32,
    ) -> Map_::Map<K, V> {
        Map_::mkMap(Some(LrcPtr::new(Map_::MapTree_2::<K, V> {
            Height: h,
            Key: k,
            Value: v,
            Left: left,
            Right: right,
        })))
    }
    pub fn singleton<K: Clone + 'static, V: Clone + 'static>(k: K, v: V) -> Map_::Map<K, V> {
        Map_::mkMapTreeLeaf(k, v)
    }
    pub fn sizeAux<K: Clone + 'static, V: Clone + 'static>(acc: i32, m: Map_::Map<K, V>) -> i32 {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    acc + 1_i32
                } else {
                    Map_::sizeAux(Map_::sizeAux(acc + 1_i32, t.Left.clone()), t.Right.clone())
                }
            }
            _ => acc,
        }
    }
    pub fn count<a: Clone + 'static, b: Clone + 'static>(x: Map_::Map<a, b>) -> i32 {
        Map_::sizeAux(0_i32, x)
    }
    pub fn mk<K: Clone + 'static, V: Clone + 'static>(
        l: Map_::Map<K, V>,
        k: K,
        v: V,
        r: Map_::Map<K, V>,
    ) -> Map_::Map<K, V> {
        let hl: i32 = {
            let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = l.root.clone();
            match &matchValue {
                Some(matchValue_0_0) => (matchValue_0_0).Height,
                _ => 0_i32,
            }
        };
        let hr: i32 = {
            let matchValue_1: Option<LrcPtr<Map_::MapTree_2<K, V>>> = r.root.clone();
            match &matchValue_1 {
                Some(matchValue_1_0_0) => (matchValue_1_0_0).Height,
                _ => 0_i32,
            }
        };
        let m_4: i32 = if hl < hr { hr } else { hl };
        if m_4 == 0_i32 {
            Map_::mkMapTreeLeaf(k.clone(), v.clone())
        } else {
            Map_::mkMapTreeNode(k, v, l, r, m_4 + 1_i32)
        }
    }
    pub fn rebalance<K: Clone + 'static, V: Clone + 'static>(
        t1: Map_::Map<K, V>,
        k: K,
        v: V,
        t2: Map_::Map<K, V>,
    ) -> Map_::Map<K, V> {
        let t1h: i32 = {
            let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = t1.root.clone();
            match &matchValue {
                Some(matchValue_0_0) => (matchValue_0_0).Height,
                _ => 0_i32,
            }
        };
        let t2h: i32 = {
            let matchValue_1: Option<LrcPtr<Map_::MapTree_2<K, V>>> = t2.root.clone();
            match &matchValue_1 {
                Some(matchValue_1_0_0) => (matchValue_1_0_0).Height,
                _ => 0_i32,
            }
        };
        if t2h > t1h + 2_i32 {
            let t2_: LrcPtr<Map_::MapTree_2<K, V>> = getValue(t2.root.clone());
            if {
                let matchValue_2: Option<LrcPtr<Map_::MapTree_2<K, V>>> = (t2_.Left).root.clone();
                match &matchValue_2 {
                    Some(matchValue_2_0_0) => (matchValue_2_0_0).Height,
                    _ => 0_i32,
                }
            } > t1h + 1_i32
            {
                let t2l: LrcPtr<Map_::MapTree_2<K, V>> = getValue((t2_.Left).root.clone());
                Map_::mk(
                    Map_::mk(t1.clone(), k.clone(), v.clone(), t2l.Left.clone()),
                    t2l.Key.clone(),
                    t2l.Value.clone(),
                    Map_::mk(
                        t2l.Right.clone(),
                        t2_.Key.clone(),
                        t2_.Value.clone(),
                        t2_.Right.clone(),
                    ),
                )
            } else {
                Map_::mk(
                    Map_::mk(t1.clone(), k.clone(), v.clone(), t2_.Left.clone()),
                    t2_.Key.clone(),
                    t2_.Value.clone(),
                    t2_.Right.clone(),
                )
            }
        } else {
            if t1h > t2h + 2_i32 {
                let t1_: LrcPtr<Map_::MapTree_2<K, V>> = getValue(t1.root.clone());
                if {
                    let matchValue_3: Option<LrcPtr<Map_::MapTree_2<K, V>>> =
                        (t1_.Right).root.clone();
                    match &matchValue_3 {
                        Some(matchValue_3_0_0) => (matchValue_3_0_0).Height,
                        _ => 0_i32,
                    }
                } > t2h + 1_i32
                {
                    let t1r: LrcPtr<Map_::MapTree_2<K, V>> = getValue((t1_.Right).root.clone());
                    Map_::mk(
                        Map_::mk(
                            t1_.Left.clone(),
                            t1_.Key.clone(),
                            t1_.Value.clone(),
                            t1r.Left.clone(),
                        ),
                        t1r.Key.clone(),
                        t1r.Value.clone(),
                        Map_::mk(t1r.Right.clone(), k.clone(), v.clone(), t2.clone()),
                    )
                } else {
                    Map_::mk(
                        t1_.Left.clone(),
                        t1_.Key.clone(),
                        t1_.Value.clone(),
                        Map_::mk(t1_.Right.clone(), k.clone(), v.clone(), t2.clone()),
                    )
                }
            } else {
                Map_::mk(t1, k, v, t2)
            }
        }
    }
    pub fn add<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        v: V,
        m: Map_::Map<K, V>,
    ) -> Map_::Map<K, V> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                let c: i32 = compare(k.clone(), t.Key.clone());
                if t.Height == 1_i32 {
                    if c < 0_i32 {
                        Map_::mkMapTreeNode(k.clone(), v.clone(), Map_::empty(), m.clone(), 2_i32)
                    } else {
                        if c == 0_i32 {
                            Map_::mkMapTreeLeaf(k.clone(), v.clone())
                        } else {
                            Map_::mkMapTreeNode(
                                k.clone(),
                                v.clone(),
                                m.clone(),
                                Map_::empty(),
                                2_i32,
                            )
                        }
                    }
                } else {
                    if c < 0_i32 {
                        Map_::rebalance(
                            Map_::add(k.clone(), v.clone(), t.Left.clone()),
                            t.Key.clone(),
                            t.Value.clone(),
                            t.Right.clone(),
                        )
                    } else {
                        if c == 0_i32 {
                            Map_::mkMapTreeNode(
                                k.clone(),
                                v.clone(),
                                t.Left.clone(),
                                t.Right.clone(),
                                t.Height,
                            )
                        } else {
                            Map_::rebalance(
                                t.Left.clone(),
                                t.Key.clone(),
                                t.Value.clone(),
                                Map_::add(k.clone(), v.clone(), t.Right.clone()),
                            )
                        }
                    }
                }
            }
            _ => Map_::mkMapTreeLeaf(k.clone(), v.clone()),
        }
    }
    pub fn tryGetValue<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        v: &MutCell<V>,
        m: Map_::Map<K, V>,
    ) -> bool {
        let k: MutCell<K> = MutCell::new(k.clone());
        let m: MutCell<Map_::Map<K, V>> = MutCell::new(m);
        '_tryGetValue: loop {
            break '_tryGetValue ({
                let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                        let c: i32 = compare(k.get().clone(), t.Key.clone());
                        if c == 0_i32 {
                            v.set(t.Value.clone());
                            true
                        } else {
                            if t.Height == 1_i32 {
                                false
                            } else {
                                let k_temp: K = k.get().clone();
                                let v_temp: V = v.get().clone();
                                let m_temp: Map_::Map<K, V> = if c < 0_i32 {
                                    t.Left.clone()
                                } else {
                                    t.Right.clone()
                                };
                                k.set(k_temp);
                                v.set(v_temp);
                                m.set(m_temp);
                                continue '_tryGetValue;
                            }
                        }
                    }
                    _ => false,
                }
            });
        }
    }
    pub fn throwKeyNotFound<a: Clone + 'static>() -> a {
        panic!("{}", SR::keyNotFound(),)
    }
    pub fn find<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        m: Map_::Map<K, V>,
    ) -> V {
        let v: MutCell<V> = MutCell::new(defaultOf());
        if Map_::tryGetValue(k, &v, m) {
            v.get().clone()
        } else {
            Map_::throwKeyNotFound()
        }
    }
    pub fn tryFind<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        m: Map_::Map<K, V>,
    ) -> Option<V> {
        let v: MutCell<V> = MutCell::new(defaultOf());
        if Map_::tryGetValue(k, &v, m) {
            Some(v.get().clone())
        } else {
            None::<V>
        }
    }
    pub fn item<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        m: Map_::Map<K, V>,
    ) -> V {
        Map_::find(k, m)
    }
    pub fn partition1<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        f: Func2<a, b, bool>,
        k: a,
        v: b,
        acc1: Map_::Map<a, b>,
        acc2: Map_::Map<a, b>,
    ) -> LrcPtr<(Map_::Map<a, b>, Map_::Map<a, b>)> {
        if f(k.clone(), v.clone()) {
            LrcPtr::new((Map_::add(k.clone(), v.clone(), acc1.clone()), acc2.clone()))
        } else {
            LrcPtr::new((acc1, Map_::add(k, v, acc2)))
        }
    }
    pub fn partitionAux<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        f: Func2<K, V, bool>,
        m: Map_::Map<K, V>,
        acc_: Map_::Map<K, V>,
        acc__1: Map_::Map<K, V>,
    ) -> LrcPtr<(Map_::Map<K, V>, Map_::Map<K, V>)> {
        let acc: LrcPtr<(Map_::Map<K, V>, Map_::Map<K, V>)> = LrcPtr::new((acc_, acc__1));
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Map_::partition1(
                        f.clone(),
                        t.Key.clone(),
                        t.Value.clone(),
                        acc.0.clone(),
                        acc.1.clone(),
                    )
                } else {
                    let acc_3: LrcPtr<(Map_::Map<K, V>, Map_::Map<K, V>)> = Map_::partitionAux(
                        f.clone(),
                        t.Right.clone(),
                        acc.0.clone(),
                        acc.1.clone(),
                    );
                    let acc_6: LrcPtr<(Map_::Map<K, V>, Map_::Map<K, V>)> = Map_::partition1(
                        f.clone(),
                        t.Key.clone(),
                        t.Value.clone(),
                        acc_3.0.clone(),
                        acc_3.1.clone(),
                    );
                    Map_::partitionAux(f.clone(), t.Left.clone(), acc_6.0.clone(), acc_6.1.clone())
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn partition<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        f: Func2<a, b, bool>,
        m: Map_::Map<a, b>,
    ) -> LrcPtr<(Map_::Map<a, b>, Map_::Map<a, b>)> {
        Map_::partitionAux(f, m, Map_::empty(), Map_::empty())
    }
    pub fn filter1<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        f: Func2<a, b, bool>,
        k: a,
        v: b,
        acc: Map_::Map<a, b>,
    ) -> Map_::Map<a, b> {
        if f(k.clone(), v.clone()) {
            Map_::add(k, v, acc.clone())
        } else {
            acc
        }
    }
    pub fn filterAux<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        f: Func2<K, V, bool>,
        m: Map_::Map<K, V>,
        acc: Map_::Map<K, V>,
    ) -> Map_::Map<K, V> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Map_::filter1(f.clone(), t.Key.clone(), t.Value.clone(), acc.clone())
                } else {
                    Map_::filterAux(
                        f.clone(),
                        t.Right.clone(),
                        Map_::filter1(
                            f.clone(),
                            t.Key.clone(),
                            t.Value.clone(),
                            Map_::filterAux(f.clone(), t.Left.clone(), acc.clone()),
                        ),
                    )
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn filter<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        f: Func2<a, b, bool>,
        m: Map_::Map<a, b>,
    ) -> Map_::Map<a, b> {
        Map_::filterAux(f, m, Map_::empty())
    }
    pub fn spliceOutSuccessor<K: Clone + 'static, V: Clone + 'static>(
        m: Map_::Map<K, V>,
    ) -> LrcPtr<(K, V, Map_::Map<K, V>)> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    LrcPtr::new((t.Key.clone(), t.Value.clone(), Map_::empty()))
                } else {
                    if Map_::isEmpty(t.Left.clone()) {
                        LrcPtr::new((t.Key.clone(), t.Value.clone(), t.Right.clone()))
                    } else {
                        let patternInput: LrcPtr<(K, V, Map_::Map<K, V>)> =
                            Map_::spliceOutSuccessor(t.Left.clone());
                        LrcPtr::new((
                            patternInput.0.clone(),
                            patternInput.1.clone(),
                            Map_::mk(
                                patternInput.2.clone(),
                                t.Key.clone(),
                                t.Value.clone(),
                                t.Right.clone(),
                            ),
                        ))
                    }
                }
            }
            _ => panic!("{}", string("internal error: Map.spliceOutSuccessor"),),
        }
    }
    pub fn remove<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        m: Map_::Map<K, V>,
    ) -> Map_::Map<K, V> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                let c: i32 = compare(k.clone(), t.Key.clone());
                if t.Height == 1_i32 {
                    if c == 0_i32 {
                        Map_::empty()
                    } else {
                        m
                    }
                } else {
                    if c < 0_i32 {
                        Map_::rebalance(
                            Map_::remove(k.clone(), t.Left.clone()),
                            t.Key.clone(),
                            t.Value.clone(),
                            t.Right.clone(),
                        )
                    } else {
                        if c == 0_i32 {
                            if Map_::isEmpty(t.Left.clone()) {
                                t.Right.clone()
                            } else {
                                if Map_::isEmpty(t.Right.clone()) {
                                    t.Left.clone()
                                } else {
                                    let patternInput: LrcPtr<(K, V, Map_::Map<K, V>)> =
                                        Map_::spliceOutSuccessor(t.Right.clone());
                                    Map_::mk(
                                        t.Left.clone(),
                                        patternInput.0.clone(),
                                        patternInput.1.clone(),
                                        patternInput.2.clone(),
                                    )
                                }
                            }
                        } else {
                            Map_::rebalance(
                                t.Left.clone(),
                                t.Key.clone(),
                                t.Value.clone(),
                                Map_::remove(k.clone(), t.Right.clone()),
                            )
                        }
                    }
                }
            }
            _ => Map_::empty(),
        }
    }
    pub fn change<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        u: Func1<Option<V>, Option<V>>,
        m: Map_::Map<K, V>,
    ) -> Map_::Map<K, V> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    let c: i32 = compare(k.clone(), t.Key.clone());
                    if c < 0_i32 {
                        let matchValue_2: Option<V> = u(None::<V>);
                        match &matchValue_2 {
                            Some(matchValue_2_0_0) => Map_::mkMapTreeNode(
                                k.clone(),
                                matchValue_2_0_0.clone(),
                                Map_::empty(),
                                m.clone(),
                                2_i32,
                            ),
                            _ => m.clone(),
                        }
                    } else {
                        if c == 0_i32 {
                            let matchValue_3: Option<V> = u(Some(t.Value.clone()));
                            match &matchValue_3 {
                                Some(matchValue_3_0_0) => {
                                    Map_::mkMapTreeLeaf(k.clone(), matchValue_3_0_0.clone())
                                }
                                _ => Map_::empty(),
                            }
                        } else {
                            let matchValue_4: Option<V> = u(None::<V>);
                            match &matchValue_4 {
                                Some(matchValue_4_0_0) => Map_::mkMapTreeNode(
                                    k.clone(),
                                    matchValue_4_0_0.clone(),
                                    m.clone(),
                                    Map_::empty(),
                                    2_i32,
                                ),
                                _ => m.clone(),
                            }
                        }
                    }
                } else {
                    let c_1: i32 = compare(k.clone(), t.Key.clone());
                    if c_1 < 0_i32 {
                        Map_::rebalance(
                            Map_::change(k.clone(), u.clone(), t.Left.clone()),
                            t.Key.clone(),
                            t.Value.clone(),
                            t.Right.clone(),
                        )
                    } else {
                        if c_1 == 0_i32 {
                            let matchValue_5: Option<V> = u(Some(t.Value.clone()));
                            match &matchValue_5 {
                                Some(matchValue_5_0_0) => Map_::mkMapTreeNode(
                                    k.clone(),
                                    matchValue_5_0_0.clone(),
                                    t.Left.clone(),
                                    t.Right.clone(),
                                    t.Height,
                                ),
                                _ => {
                                    if Map_::isEmpty(t.Left.clone()) {
                                        t.Right.clone()
                                    } else {
                                        if Map_::isEmpty(t.Right.clone()) {
                                            t.Left.clone()
                                        } else {
                                            let patternInput: LrcPtr<(K, V, Map_::Map<K, V>)> =
                                                Map_::spliceOutSuccessor(t.Right.clone());
                                            Map_::mk(
                                                t.Left.clone(),
                                                patternInput.0.clone(),
                                                patternInput.1.clone(),
                                                patternInput.2.clone(),
                                            )
                                        }
                                    }
                                }
                            }
                        } else {
                            Map_::rebalance(
                                t.Left.clone(),
                                t.Key.clone(),
                                t.Value.clone(),
                                Map_::change(k.clone(), u.clone(), t.Right.clone()),
                            )
                        }
                    }
                }
            }
            _ => {
                let matchValue_1: Option<V> = u(None::<V>);
                match &matchValue_1 {
                    Some(matchValue_1_0_0) => {
                        Map_::mkMapTreeLeaf(k.clone(), matchValue_1_0_0.clone())
                    }
                    _ => m.clone(),
                }
            }
        }
    }
    pub fn containsKey<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        k: K,
        m: Map_::Map<K, V>,
    ) -> bool {
        let k: MutCell<K> = MutCell::new(k.clone());
        let m: MutCell<Map_::Map<K, V>> = MutCell::new(m);
        '_containsKey: loop {
            break '_containsKey ({
                let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                        let c: i32 = compare(k.get().clone(), t.Key.clone());
                        if t.Height == 1_i32 {
                            c == 0_i32
                        } else {
                            if c < 0_i32 {
                                let k_temp: K = k.get().clone();
                                let m_temp: Map_::Map<K, V> = t.Left.clone();
                                k.set(k_temp);
                                m.set(m_temp);
                                continue '_containsKey;
                            } else {
                                if c == 0_i32 {
                                    true
                                } else {
                                    let k_temp: K = k.get().clone();
                                    let m_temp: Map_::Map<K, V> = t.Right.clone();
                                    k.set(k_temp);
                                    m.set(m_temp);
                                    continue '_containsKey;
                                }
                            }
                        }
                    }
                    _ => false,
                }
            });
        }
    }
    pub fn iterate<K: Clone + 'static, V: Clone + 'static>(f: Func2<K, V, ()>, m: Map_::Map<K, V>) {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone(), t.Value.clone())
                } else {
                    Map_::iterate(f.clone(), t.Left.clone());
                    f(t.Key.clone(), t.Value.clone());
                    Map_::iterate(f.clone(), t.Right.clone())
                }
            }
            _ => (),
        }
    }
    pub fn tryPick<K: Clone + 'static, V: Clone + 'static, a: Clone + 'static>(
        f: Func2<K, V, Option<a>>,
        m: Map_::Map<K, V>,
    ) -> Option<a> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone(), t.Value.clone())
                } else {
                    let matchValue_1: Option<a> = Map_::tryPick(f.clone(), t.Left.clone());
                    match &matchValue_1 {
                        None => {
                            let matchValue_2: Option<a> = f(t.Key.clone(), t.Value.clone());
                            match &matchValue_2 {
                                None => Map_::tryPick(f.clone(), t.Right.clone()),
                                _ => matchValue_2.clone(),
                            }
                        }
                        _ => matchValue_1.clone(),
                    }
                }
            }
            _ => None::<a>,
        }
    }
    pub fn pick<K: Clone + 'static, V: Clone + 'static, a: Clone + 'static>(
        chooser: Func2<K, V, Option<a>>,
        m: Map_::Map<K, V>,
    ) -> a {
        let matchValue: Option<a> = Map_::tryPick(chooser, m);
        match &matchValue {
            Some(matchValue_0_0) => matchValue_0_0.clone(),
            _ => Map_::throwKeyNotFound(),
        }
    }
    pub fn findKey<K: Clone + 'static, V: Clone + 'static>(
        predicate: Func2<K, V, bool>,
        m: Map_::Map<K, V>,
    ) -> K {
        Map_::pick(
            Func2::new({
                let predicate = predicate.clone();
                move |k: K, v: V| {
                    if predicate(k.clone(), v) {
                        Some(k)
                    } else {
                        None::<K>
                    }
                }
            }),
            m,
        )
    }
    pub fn tryFindKey<K: Clone + 'static, V: Clone + 'static>(
        predicate: Func2<K, V, bool>,
        m: Map_::Map<K, V>,
    ) -> Option<K> {
        Map_::tryPick(
            Func2::new({
                let predicate = predicate.clone();
                move |k: K, v: V| {
                    if predicate(k.clone(), v) {
                        Some(k)
                    } else {
                        None::<K>
                    }
                }
            }),
            m,
        )
    }
    pub fn exists<K: Clone + 'static, V: Clone + 'static>(
        f: Func2<K, V, bool>,
        m: Map_::Map<K, V>,
    ) -> bool {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone(), t.Value.clone())
                } else {
                    if if Map_::exists(f.clone(), t.Left.clone()) {
                        true
                    } else {
                        f(t.Key.clone(), t.Value.clone())
                    } {
                        true
                    } else {
                        Map_::exists(f.clone(), t.Right.clone())
                    }
                }
            }
            _ => false,
        }
    }
    pub fn forAll<K: Clone + 'static, V: Clone + 'static>(
        f: Func2<K, V, bool>,
        m: Map_::Map<K, V>,
    ) -> bool {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone(), t.Value.clone())
                } else {
                    if if Map_::forAll(f.clone(), t.Left.clone()) {
                        f(t.Key.clone(), t.Value.clone())
                    } else {
                        false
                    } {
                        Map_::forAll(f.clone(), t.Right.clone())
                    } else {
                        false
                    }
                }
            }
            _ => true,
        }
    }
    pub fn mapRange<V: Clone + 'static, R: Clone + 'static, K: Clone + 'static>(
        f: Func1<V, R>,
        m: Map_::Map<K, V>,
    ) -> Map_::Map<K, R> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Map_::mkMapTreeLeaf(t.Key.clone(), f(t.Value.clone()))
                } else {
                    let l2: Map_::Map<K, R> = Map_::mapRange(f.clone(), t.Left.clone());
                    Map_::mkMapTreeNode(
                        t.Key.clone(),
                        f(t.Value.clone()),
                        l2,
                        Map_::mapRange(f.clone(), t.Right.clone()),
                        t.Height,
                    )
                }
            }
            _ => Map_::empty(),
        }
    }
    pub fn map<K: Clone + 'static, V: Clone + 'static, R: Clone + 'static>(
        f: Func2<K, V, R>,
        m: Map_::Map<K, V>,
    ) -> Map_::Map<K, R> {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    Map_::mkMapTreeLeaf(t.Key.clone(), f(t.Key.clone(), t.Value.clone()))
                } else {
                    let l2: Map_::Map<K, R> = Map_::map(f.clone(), t.Left.clone());
                    Map_::mkMapTreeNode(
                        t.Key.clone(),
                        f(t.Key.clone(), t.Value.clone()),
                        l2,
                        Map_::map(f.clone(), t.Right.clone()),
                        t.Height,
                    )
                }
            }
            _ => Map_::empty(),
        }
    }
    pub fn foldBack<K: Clone + 'static, V: Clone + 'static, a: Clone + 'static>(
        f: Func3<K, V, a, a>,
        m: Map_::Map<K, V>,
        x: a,
    ) -> a {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(t.Key.clone(), t.Value.clone(), x.clone())
                } else {
                    Map_::foldBack(
                        f.clone(),
                        t.Left.clone(),
                        f(
                            t.Key.clone(),
                            t.Value.clone(),
                            Map_::foldBack(f.clone(), t.Right.clone(), x.clone()),
                        ),
                    )
                }
            }
            _ => x.clone(),
        }
    }
    pub fn fold<a: Clone + 'static, K: Clone + 'static, V: Clone + 'static>(
        f: Func3<a, K, V, a>,
        x: a,
        m: Map_::Map<K, V>,
    ) -> a {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    f(x.clone(), t.Key.clone(), t.Value.clone())
                } else {
                    Map_::fold(
                        f.clone(),
                        f(
                            Map_::fold(f.clone(), x.clone(), t.Left.clone()),
                            t.Key.clone(),
                            t.Value.clone(),
                        ),
                        t.Right.clone(),
                    )
                }
            }
            _ => x.clone(),
        }
    }
    pub fn foldFromTo<K: PartialOrd + Clone + 'static, V: Clone + 'static, a: Clone + 'static>(
        lo: K,
        hi: K,
        f: Func3<K, V, a, a>,
        m: Map_::Map<K, V>,
        x: a,
    ) -> a {
        let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
        match &matchValue {
            Some(matchValue_0_0) => {
                let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                if t.Height == 1_i32 {
                    if if compare(lo.clone(), t.Key.clone()) <= 0_i32 {
                        compare(t.Key.clone(), hi.clone()) <= 0_i32
                    } else {
                        false
                    } {
                        f(t.Key.clone(), t.Value.clone(), x.clone())
                    } else {
                        x.clone()
                    }
                } else {
                    let cLoKey_1: i32 = compare(lo.clone(), t.Key.clone());
                    let cKeyHi_1: i32 = compare(t.Key.clone(), hi.clone());
                    let x_2: a = if cLoKey_1 < 0_i32 {
                        Map_::foldFromTo(
                            lo.clone(),
                            hi.clone(),
                            f.clone(),
                            t.Left.clone(),
                            x.clone(),
                        )
                    } else {
                        x.clone()
                    };
                    let x_3: a = if if cLoKey_1 <= 0_i32 {
                        cKeyHi_1 <= 0_i32
                    } else {
                        false
                    } {
                        f(t.Key.clone(), t.Value.clone(), x_2.clone())
                    } else {
                        x_2
                    };
                    if cKeyHi_1 < 0_i32 {
                        Map_::foldFromTo(
                            lo.clone(),
                            hi.clone(),
                            f.clone(),
                            t.Right.clone(),
                            x_3.clone(),
                        )
                    } else {
                        x_3
                    }
                }
            }
            _ => x.clone(),
        }
    }
    pub fn foldSection<K: PartialOrd + Clone + 'static, V: Clone + 'static, a: Clone + 'static>(
        lo: K,
        hi: K,
        f: Func3<K, V, a, a>,
        m: Map_::Map<K, V>,
        x: a,
    ) -> a {
        if compare(lo.clone(), hi.clone()) == 1_i32 {
            x.clone()
        } else {
            Map_::foldFromTo(lo, hi, f, m, x)
        }
    }
    pub fn copyToArray<a: Clone + 'static, b: Clone + 'static>(
        m: Map_::Map<a, b>,
        arr: Array<LrcPtr<(a, b)>>,
        i: i32,
    ) {
        let j: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(i));
        Map_::iterate(
            Func2::new({
                let arr = arr.clone();
                let j = j.clone();
                move |k: a, v: b| {
                    arr.get_mut()[j.get().clone() as usize] = LrcPtr::new((k, v));
                    j.set(j.get().clone() + 1_i32)
                }
            }),
            m,
        )
    }
    pub fn keys<K: Clone + 'static, V: Clone + 'static>(m: Map_::Map<K, V>) -> Array<K> {
        let res: Array<K> = new_with_capacity::<K>(Map_::count(m.clone()));
        Map_::iterate(
            Func2::new({
                let res = res.clone();
                move |k: K, v: V| add_1(res.clone(), k)
            }),
            m,
        );
        res.clone()
    }
    pub fn values<K: Clone + 'static, V: Clone + 'static>(m: Map_::Map<K, V>) -> Array<V> {
        let res: Array<V> = new_with_capacity::<V>(Map_::count(m.clone()));
        Map_::iterate(
            Func2::new({
                let res = res.clone();
                move |k: K, v: V| add_1(res.clone(), v)
            }),
            m,
        );
        res.clone()
    }
    pub fn toArray<K: Clone + 'static, V: Clone + 'static>(
        m: Map_::Map<K, V>,
    ) -> Array<LrcPtr<(K, V)>> {
        let res: Array<LrcPtr<(K, V)>> =
            new_with_capacity::<LrcPtr<(K, V)>>(Map_::count(m.clone()));
        Map_::iterate(
            Func2::new({
                let res = res.clone();
                move |k: K, v: V| add_1(res.clone(), LrcPtr::new((k, v)))
            }),
            m,
        );
        res.clone()
    }
    pub fn toList<K: Clone + 'static, V: Clone + 'static>(
        m: Map_::Map<K, V>,
    ) -> List<LrcPtr<(K, V)>> {
        Map_::foldBack(
            Func3::new(move |k: K, v: V, acc: List<LrcPtr<(K, V)>>| cons(LrcPtr::new((k, v)), acc)),
            m,
            empty_1::<LrcPtr<(K, V)>>(),
        )
    }
    pub fn ofArray<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        xs: Array<LrcPtr<(a, b)>>,
    ) -> Map_::Map<a, b> {
        fold_1(
            Func2::new(move |acc: Map_::Map<a, b>, tupledArg: LrcPtr<(a, b)>| {
                Map_::add(tupledArg.0.clone(), tupledArg.1.clone(), acc)
            }),
            Map_::empty(),
            xs,
        )
    }
    pub fn ofList<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        xs: List<LrcPtr<(a, b)>>,
    ) -> Map_::Map<a, b> {
        fold_2(
            Func2::new(move |acc: Map_::Map<a, b>, tupledArg: LrcPtr<(a, b)>| {
                Map_::add(tupledArg.0.clone(), tupledArg.1.clone(), acc)
            }),
            Map_::empty(),
            xs,
        )
    }
    pub fn ofSeq<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        xs: LrcPtr<dyn IEnumerable_1<LrcPtr<(a, b)>>>,
    ) -> Map_::Map<a, b> {
        fold_3(
            Func2::new(move |acc: Map_::Map<a, b>, tupledArg: LrcPtr<(a, b)>| {
                Map_::add(tupledArg.0.clone(), tupledArg.1.clone(), acc)
            }),
            Map_::empty(),
            xs,
        )
    }
    #[derive(Clone, Debug, Default)]
    pub struct MapIterator_2<K: PartialOrd + Clone + 'static, V: Clone + 'static> {
        pub stack: MutCell<List<Map_::Map<K, V>>>,
        pub started: MutCell<bool>,
    }
    impl<K: PartialOrd + Clone + 'static, V: Clone + 'static> core::fmt::Display
        for Map_::MapIterator_2<K, V>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    pub fn collapseLHS<K: Clone + 'static, V: Clone + 'static>(
        stack: List<Map_::Map<K, V>>,
    ) -> List<Map_::Map<K, V>> {
        let stack: MutCell<List<Map_::Map<K, V>>> = MutCell::new(stack.clone());
        '_collapseLHS: loop {
            break '_collapseLHS (if !isEmpty_1(stack.get().clone()) {
                let rest: List<Map_::Map<K, V>> = tail(stack.get().clone());
                let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> =
                    (head(stack.get().clone())).root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                        if t.Height == 1_i32 {
                            stack.get().clone()
                        } else {
                            let stack_temp: List<Map_::Map<K, V>> = ofArrayWithTail(
                                new_array(&[
                                    t.Left.clone(),
                                    Map_::mkMapTreeLeaf(t.Key.clone(), t.Value.clone()),
                                    t.Right.clone(),
                                ]),
                                rest.clone(),
                            );
                            stack.set(stack_temp);
                            continue '_collapseLHS;
                        }
                    }
                    _ => {
                        let stack_temp: List<Map_::Map<K, V>> = rest;
                        stack.set(stack_temp);
                        continue '_collapseLHS;
                    }
                }
            } else {
                empty_1::<Map_::Map<K, V>>()
            });
        }
    }
    pub fn mkIterator<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        m: Map_::Map<a, b>,
    ) -> LrcPtr<Map_::MapIterator_2<a, b>> {
        LrcPtr::new(Map_::MapIterator_2::<a, b> {
            stack: MutCell::new(Map_::collapseLHS(singleton_1(m))),
            started: MutCell::new(false),
        })
    }
    pub fn notStarted<a: Clone + 'static>() -> a {
        panic!("{}", SR::enumerationNotStarted(),)
    }
    pub fn alreadyFinished<a: Clone + 'static>() -> a {
        panic!("{}", SR::enumerationAlreadyFinished(),)
    }
    pub fn unexpectedStackForCurrent<a: Clone + 'static>() -> a {
        panic!(
            "{}",
            string("Please report error: Map iterator, unexpected stack for current"),
        )
    }
    pub fn unexpectedStackForMoveNext<a: Clone + 'static>() -> a {
        panic!(
            "{}",
            string("Please report error: Map iterator, unexpected stack for moveNext"),
        )
    }
    pub fn current<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        i: LrcPtr<Map_::MapIterator_2<a, b>>,
    ) -> LrcPtr<(a, b)> {
        if i.started.get().clone() {
            let matchValue: List<Map_::Map<a, b>> = i.stack.get().clone();
            if !isEmpty_1(matchValue.clone()) {
                if (head(matchValue.clone())).root.is_some() {
                    let t: LrcPtr<Map_::MapTree_2<a, b>> =
                        getValue((head(matchValue.clone())).root.clone());
                    if t.Height == 1_i32 {
                        LrcPtr::new((t.Key.clone(), t.Value.clone()))
                    } else {
                        Map_::unexpectedStackForCurrent()
                    }
                } else {
                    Map_::alreadyFinished()
                }
            } else {
                Map_::alreadyFinished()
            }
        } else {
            Map_::notStarted()
        }
    }
    pub fn moveNext<a: PartialOrd + Clone + 'static, b: Clone + 'static>(
        i: LrcPtr<Map_::MapIterator_2<a, b>>,
    ) -> bool {
        if i.started.get().clone() {
            let matchValue: List<Map_::Map<a, b>> = i.stack.get().clone();
            if !isEmpty_1(matchValue.clone()) {
                if (head(matchValue.clone())).root.is_some() {
                    let t: LrcPtr<Map_::MapTree_2<a, b>> =
                        getValue((head(matchValue.clone())).root.clone());
                    if t.Height == 1_i32 {
                        i.stack.set(Map_::collapseLHS(tail(matchValue.clone())));
                        !isEmpty_1(i.stack.get().clone())
                    } else {
                        Map_::unexpectedStackForMoveNext()
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
    pub fn toSeq<K: PartialOrd + Clone + 'static, V: Clone + 'static>(
        m: Map_::Map<K, V>,
    ) -> LrcPtr<dyn IEnumerable_1<LrcPtr<(K, V)>>> {
        delay(Func0::new({
            let m = m.clone();
            move || {
                unfold(
                    Func1::new(move |i: LrcPtr<Map_::MapIterator_2<K, V>>| {
                        if Map_::moveNext(i.clone()) {
                            Some(LrcPtr::new((Map_::current(i.clone()), i)))
                        } else {
                            None::<LrcPtr<(LrcPtr<(K, V)>, LrcPtr<Map_::MapIterator_2<K, V>>)>>
                        }
                    }),
                    Map_::mkIterator(m.clone()),
                )
            }
        }))
    }
    pub fn compareTo<K: PartialOrd + Clone + 'static, V: PartialOrd + Clone + 'static>(
        m1: Map_::Map<K, V>,
        m2: Map_::Map<K, V>,
    ) -> i32 {
        compareWith(
            Func2::new(move |e: LrcPtr<(K, V)>, e_1: LrcPtr<(K, V)>| compare(e, e_1)),
            Map_::toSeq(m1),
            Map_::toSeq(m2),
        )
    }
    pub fn equals<K: PartialOrd + Clone + 'static, V: PartialOrd + Clone + 'static>(
        m1: Map_::Map<K, V>,
        m2: Map_::Map<K, V>,
    ) -> bool {
        Map_::compareTo(m1, m2) == 0_i32
    }
    pub fn minKeyValue<K: Clone + 'static, V: Clone + 'static>(
        m: Map_::Map<K, V>,
    ) -> LrcPtr<(K, V)> {
        let m: MutCell<Map_::Map<K, V>> = MutCell::new(m);
        '_minKeyValue: loop {
            break '_minKeyValue ({
                let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                        if if t.Height == 1_i32 {
                            true
                        } else {
                            Map_::isEmpty(t.Left.clone())
                        } {
                            LrcPtr::new((t.Key.clone(), t.Value.clone()))
                        } else {
                            let m_temp: Map_::Map<K, V> = t.Left.clone();
                            m.set(m_temp);
                            continue '_minKeyValue;
                        }
                    }
                    _ => Map_::throwKeyNotFound(),
                }
            });
        }
    }
    pub fn maxKeyValue<K: Clone + 'static, V: Clone + 'static>(
        m: Map_::Map<K, V>,
    ) -> LrcPtr<(K, V)> {
        let m: MutCell<Map_::Map<K, V>> = MutCell::new(m);
        '_maxKeyValue: loop {
            break '_maxKeyValue ({
                let matchValue: Option<LrcPtr<Map_::MapTree_2<K, V>>> = m.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let t: LrcPtr<Map_::MapTree_2<K, V>> = matchValue_0_0.clone();
                        if if t.Height == 1_i32 {
                            true
                        } else {
                            Map_::isEmpty(t.Right.clone())
                        } {
                            LrcPtr::new((t.Key.clone(), t.Value.clone()))
                        } else {
                            let m_temp: Map_::Map<K, V> = t.Right.clone();
                            m.set(m_temp);
                            continue '_maxKeyValue;
                        }
                    }
                    _ => Map_::throwKeyNotFound(),
                }
            });
        }
    }
}
