pub mod List_ {
    use super::*;
    use crate::Array_::chunkBySize as chunkBySize_1;
    use crate::Array_::map as map_1;
    use crate::Array_::pairwise as pairwise_1;
    use crate::Array_::permute as permute_1;
    use crate::Array_::scan as scan_1;
    use crate::Array_::scanBack as scanBack_1;
    use crate::Array_::sortInPlaceWith;
    use crate::Array_::splitInto as splitInto_1;
    use crate::Array_::tryFindBack as tryFindBack_1;
    use crate::Array_::tryFindIndexBack as tryFindIndexBack_1;
    use crate::Array_::windowed as windowed_1;
    use crate::Exception_::finally;
    use crate::Global_::SR;
    use crate::HashMap_::add as add_2;
    use crate::HashMap_::get;
    use crate::HashMap_::new_empty as new_empty_1;
    use crate::HashMap_::set;
    use crate::HashMap_::tryGetValue;
    use crate::HashMap_::HashMap;
    use crate::HashSet_::add as add_1;
    use crate::HashSet_::new_empty;
    use crate::HashSet_::new_from_array;
    use crate::HashSet_::HashSet;
    use crate::Interfaces_::System::Collections::Generic::IEnumerable_1;
    use crate::Interfaces_::System::Collections::Generic::IEnumerator_1;
    use crate::Interfaces_::System::IDisposable;
    use crate::NativeArray_::add;
    use crate::NativeArray_::count as count_2;
    use crate::NativeArray_::new_array;
    use crate::NativeArray_::new_empty as new_empty_2;
    use crate::NativeArray_::new_with_capacity;
    use crate::NativeArray_::Array;
    use crate::Native_::compare;
    use crate::Native_::getZero;
    use crate::Native_::Any;
    use crate::Native_::Func1;
    use crate::Native_::Func2;
    use crate::Native_::Func3;
    use crate::Native_::LrcPtr;
    use crate::Native_::MutCell;
    use crate::Option_::getValue;
    use crate::Seq_::toArray as toArray_1;
    use crate::String_::append as append_1;
    use crate::String_::string;
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Hash, Eq)]
    pub struct Node_1<T: Clone + 'static> {
        pub head: T,
        pub tail: MutCell<List_::List<T>>,
    }
    impl<T: Clone + 'static> core::fmt::Display for List_::Node_1<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Hash, Eq)]
    pub struct List<T: Clone + 'static> {
        pub root: Option<LrcPtr<List_::Node_1<T>>>,
    }
    impl<T: Clone + 'static> core::fmt::Display for List_::List<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    fn mkList<a: Clone + 'static>(root: Option<LrcPtr<List_::Node_1<a>>>) -> List_::List<a> {
        List_::List::<a> { root: root }
    }
    fn appendConsNoTail<T: Clone + 'static>(
        x: T,
        node: Option<LrcPtr<List_::Node_1<T>>>,
    ) -> Option<LrcPtr<List_::Node_1<T>>> {
        let tail_1: Option<LrcPtr<List_::Node_1<T>>> = Some(LrcPtr::new(List_::Node_1::<T> {
            head: x,
            tail: MutCell::new(List_::mkList(None::<LrcPtr<List_::Node_1<T>>>)),
        }));
        {
            let node_1: Option<LrcPtr<List_::Node_1<T>>> = node;
            match &node_1 {
                None => (),
                Some(node_1_0_0) => {
                    let node_1_1: LrcPtr<List_::Node_1<T>> = node_1_0_0.clone();
                    node_1_1.tail.set(List_::mkList(tail_1.clone()))
                }
            }
        }
        tail_1
    }
    pub fn empty<T: Clone + 'static>() -> List_::List<T> {
        List_::mkList(None::<LrcPtr<List_::Node_1<T>>>)
    }
    pub fn cons<T: Clone + 'static>(x: T, xs: List_::List<T>) -> List_::List<T> {
        List_::mkList(Some(LrcPtr::new(List_::Node_1::<T> {
            head: x,
            tail: MutCell::new(xs),
        })))
    }
    pub fn singleton<T: Clone + 'static>(x: T) -> List_::List<T> {
        List_::cons(x, List_::empty())
    }
    pub fn isEmpty<T: Clone + 'static>(xs: List_::List<T>) -> bool {
        xs.root.is_none()
    }
    pub fn head<T: Clone + 'static>(xs: List_::List<T>) -> T {
        let matchValue: Option<LrcPtr<List_::Node_1<T>>> = xs.root.clone();
        match &matchValue {
            None => panic!(
                "{}",
                append_1(SR::inputListWasEmpty(), string(" (Parameter \'list\')")),
            ),
            Some(matchValue_0_0) => (matchValue_0_0).head.clone(),
        }
    }
    pub fn tryHead<T: Clone + 'static>(xs: List_::List<T>) -> Option<T> {
        let matchValue: Option<LrcPtr<List_::Node_1<T>>> = xs.root.clone();
        match &matchValue {
            None => None::<T>,
            Some(matchValue_0_0) => Some((matchValue_0_0).head.clone()),
        }
    }
    pub fn tail<T: Clone + 'static>(xs: List_::List<T>) -> List_::List<T> {
        let matchValue: Option<LrcPtr<List_::Node_1<T>>> = xs.root.clone();
        match &matchValue {
            None => panic!(
                "{}",
                append_1(SR::inputListWasEmpty(), string(" (Parameter \'list\')")),
            ),
            Some(matchValue_0_0) => {
                let node: LrcPtr<List_::Node_1<T>> = matchValue_0_0.clone();
                node.tail.get().clone()
            }
        }
    }
    pub fn length<T: Clone + 'static>(xs: List_::List<T>) -> i32 {
        fn inner_loop<T: Clone + 'static>(i: i32, xs_1: List_::List<T>) -> i32 {
            let i: MutCell<i32> = MutCell::new(i);
            let xs_1: MutCell<List_::List<T>> = MutCell::new(xs_1);
            '_inner_loop: loop {
                break '_inner_loop ({
                    let matchValue: Option<LrcPtr<List_::Node_1<T>>> = xs_1.root.clone();
                    match &matchValue {
                        Some(matchValue_0_0) => {
                            let node: LrcPtr<List_::Node_1<T>> = matchValue_0_0.clone();
                            {
                                let i_temp: i32 = i.get().clone() + 1_i32;
                                let xs_1_temp: List_::List<T> = node.tail.get().clone();
                                i.set(i_temp);
                                xs_1.set(xs_1_temp);
                                continue '_inner_loop;
                            }
                        }
                        _ => i.get().clone(),
                    }
                });
            }
        }
        inner_loop(0_i32, xs)
    }
    pub fn tryLast<T: Clone + 'static>(xs: List_::List<T>) -> Option<T> {
        let xs: MutCell<List_::List<T>> = MutCell::new(xs);
        '_tryLast: loop {
            break '_tryLast ({
                let matchValue: Option<LrcPtr<List_::Node_1<T>>> = xs.root.clone();
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let node: LrcPtr<List_::Node_1<T>> = matchValue_0_0.clone();
                        if List_::isEmpty(node.tail.get().clone()) {
                            Some(node.head.clone())
                        } else {
                            let xs_temp: List_::List<T> = node.tail.get().clone();
                            xs.set(xs_temp);
                            continue '_tryLast;
                        }
                    }
                    _ => None::<T>,
                }
            });
        }
    }
    pub fn last<T: Clone + 'static>(xs: List_::List<T>) -> T {
        let matchValue: Option<T> = List_::tryLast(xs);
        match &matchValue {
            None => panic!("{}", SR::inputListWasEmpty(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn ofOption<T: Clone + 'static>(opt: Option<T>) -> List_::List<T> {
        match &opt {
            None => List_::empty(),
            Some(opt_0_0) => List_::singleton(opt_0_0.clone()),
        }
    }
    pub fn ofSeq<T: Clone + 'static>(xs: LrcPtr<dyn IEnumerable_1<T>>) -> List_::List<T> {
        let root: MutCell<Option<LrcPtr<List_::Node_1<T>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<T>>>);
        let node: MutCell<Option<LrcPtr<List_::Node_1<T>>>> = MutCell::new(root.get().clone());
        {
            let enumerator: LrcPtr<dyn IEnumerator_1<T>> =
                IEnumerable_1::GetEnumerator(xs.as_ref());
            {
                finally(|| {
                    if ((&enumerator) as &dyn Any).is::<LrcPtr<dyn IDisposable>>() {
                        (&enumerator).Dispose();
                    }
                });
                while IEnumerator_1::MoveNext(enumerator.as_ref()) {
                    let x: T = IEnumerator_1::get_Current(enumerator.as_ref());
                    node.set(List_::appendConsNoTail(x, node.get().clone()));
                    if root.get().clone().is_none() {
                        root.set(node.get().clone());
                    }
                }
            }
        }
        List_::mkList(root.get().clone())
    }
    pub fn toArray<T: Clone + 'static>(xs: List_::List<T>) -> Array<T> {
        let res: Array<T> = new_with_capacity::<T>(List_::length(xs.clone()));
        let xs_1: MutCell<List_::List<T>> = MutCell::new(xs);
        while !List_::isEmpty(xs_1.get().clone()) {
            add(res.clone(), List_::head(xs_1.get().clone()));
            xs_1.set(List_::tail(xs_1.get().clone()))
        }
        res.clone()
    }
    pub fn fold<State: Clone + 'static, T: Clone + 'static>(
        folder: Func2<State, T, State>,
        state: State,
        xs: List_::List<T>,
    ) -> State {
        let acc: MutCell<State> = MutCell::new(state);
        let xs_1: MutCell<List_::List<T>> = MutCell::new(xs);
        while !List_::isEmpty(xs_1.get().clone()) {
            acc.set(folder(acc.get().clone(), List_::head(xs_1.get().clone())));
            xs_1.set(List_::tail(xs_1.get().clone()))
        }
        acc.get().clone()
    }
    pub fn reverse<T: Clone + 'static>(xs: List_::List<T>) -> List_::List<T> {
        List_::fold(
            Func2::new(move |acc: List_::List<T>, x: T| List_::cons(x, acc)),
            List_::empty(),
            xs,
        )
    }
    pub fn foldBack<T: Clone + 'static, State: Clone + 'static>(
        folder: Func2<T, State, State>,
        xs: List_::List<T>,
        state: State,
    ) -> State {
        List_::fold(
            Func2::new({
                let folder = folder.clone();
                move |acc: State, x: T| folder(x, acc)
            }),
            state,
            List_::reverse(xs),
        )
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone + 'static>(
        folder: Func3<State, T1, T2, State>,
        state: State,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) -> State {
        let acc: MutCell<State> = MutCell::new(state);
        let xs_1: MutCell<List_::List<T1>> = MutCell::new(xs);
        let ys_1: MutCell<List_::List<T2>> = MutCell::new(ys);
        while if !List_::isEmpty(xs_1.get().clone()) {
            !List_::isEmpty(ys_1.get().clone())
        } else {
            false
        } {
            acc.set(folder(
                acc.get().clone(),
                List_::head(xs_1.get().clone()),
                List_::head(ys_1.get().clone()),
            ));
            xs_1.set(List_::tail(xs_1.get().clone()));
            ys_1.set(List_::tail(ys_1.get().clone()))
        }
        acc.get().clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone + 'static>(
        folder: Func3<T1, T2, State, State>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
        state: State,
    ) -> State {
        List_::fold2(
            Func3::new({
                let folder = folder.clone();
                move |acc: State, x: T1, y: T2| folder(x, y, acc)
            }),
            state,
            List_::reverse(xs),
            List_::reverse(ys),
        )
    }
    pub fn forAll<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> bool {
        let predicate = MutCell::new(predicate.clone());
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        '_forAll: loop {
            break '_forAll (if List_::isEmpty(xs.get().clone()) {
                true
            } else {
                if predicate(List_::head(xs.get().clone())) {
                    let predicate_temp = predicate.get().clone();
                    let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                    predicate.set(predicate_temp);
                    xs.set(xs_temp);
                    continue '_forAll;
                } else {
                    false
                }
            });
        }
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone + 'static>(
        predicate: Func2<T1, T2, bool>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) -> bool {
        let predicate = MutCell::new(predicate.clone());
        let xs: MutCell<List_::List<T1>> = MutCell::new(xs.clone());
        let ys: MutCell<List_::List<T2>> = MutCell::new(ys.clone());
        '_forAll2: loop {
            break '_forAll2 ({
                let matchValue: bool = List_::isEmpty(xs.get().clone());
                let matchValue_1: bool = List_::isEmpty(ys.get().clone());
                if matchValue {
                    if matchValue_1 {
                        true
                    } else {
                        panic!(
                            "{}",
                            append_1(
                                SR::listsHadDifferentLengths(),
                                string(" (Parameter \'list2\')")
                            ),
                        )
                    }
                } else {
                    if matchValue_1 {
                        panic!(
                            "{}",
                            append_1(
                                SR::listsHadDifferentLengths(),
                                string(" (Parameter \'list2\')")
                            ),
                        )
                    } else {
                        if predicate(List_::head(xs.get().clone()), List_::head(ys.get().clone())) {
                            let predicate_temp = predicate.get().clone();
                            let xs_temp: List_::List<T1> = List_::tail(xs.get().clone());
                            let ys_temp: List_::List<T2> = List_::tail(ys.get().clone());
                            predicate.set(predicate_temp);
                            xs.set(xs_temp);
                            ys.set(ys_temp);
                            continue '_forAll2;
                        } else {
                            false
                        }
                    }
                }
            });
        }
    }
    pub fn unfold<State: Clone + 'static, T: Clone + 'static>(
        gen: Func1<State, Option<LrcPtr<(T, State)>>>,
        state: State,
    ) -> List_::List<T> {
        let root: MutCell<Option<LrcPtr<List_::Node_1<T>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<T>>>);
        let node: MutCell<Option<LrcPtr<List_::Node_1<T>>>> = MutCell::new(root.get().clone());
        let acc: MutCell<Option<LrcPtr<(T, State)>>> = MutCell::new(gen(state));
        while acc.get().clone().is_some() {
            let patternInput: LrcPtr<(T, State)> = getValue(acc.get().clone());
            node.set(List_::appendConsNoTail(
                patternInput.0.clone(),
                node.get().clone(),
            ));
            if root.get().clone().is_none() {
                root.set(node.get().clone());
            }
            acc.set(gen(patternInput.1.clone()))
        }
        List_::mkList(root.get().clone())
    }
    pub fn iterate<T: Clone + 'static>(action: Func1<T, ()>, xs: List_::List<T>) {
        List_::fold(
            Func2::new({
                let action = action.clone();
                move |unitVar: (), x: T| action(x)
            }),
            (),
            xs,
        );
    }
    pub fn iterate2<T1: Clone + 'static, T2: Clone + 'static>(
        action: Func2<T1, T2, ()>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) {
        List_::fold2(
            Func3::new({
                let action = action.clone();
                move |unitVar: (), x: T1, y: T2| action(x, y)
            }),
            (),
            xs,
            ys,
        );
    }
    pub fn iterateIndexed<T: Clone + 'static>(action: Func2<i32, T, ()>, xs: List_::List<T>) {
        let value: i32 = List_::fold(
            Func2::new({
                let action = action.clone();
                move |i: i32, x: T| {
                    action(i, x);
                    i + 1_i32
                }
            }),
            0_i32,
            xs,
        );
        ()
    }
    pub fn iterateIndexed2<T1: Clone + 'static, T2: Clone + 'static>(
        action: Func3<i32, T1, T2, ()>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) {
        let value: i32 = List_::fold2(
            Func3::new({
                let action = action.clone();
                move |i: i32, x: T1, y: T2| {
                    action(i, x, y);
                    i + 1_i32
                }
            }),
            0_i32,
            xs,
            ys,
        );
        ()
    }
    pub fn ofArrayWithTail<T: Clone + 'static>(
        xs: Array<T>,
        tail_1: List_::List<T>,
    ) -> List_::List<T> {
        let res: MutCell<List_::List<T>> = MutCell::new(tail_1);
        let len: i32 = count_2(xs.clone());
        for i in (0_i32..=len - 1_i32).rev() {
            res.set(List_::cons(xs[i].clone(), res.get().clone()));
        }
        res.get().clone()
    }
    pub fn ofArray<T: Clone + 'static>(xs: Array<T>) -> List_::List<T> {
        List_::ofArrayWithTail(xs, List_::empty())
    }
    pub fn append<T: Clone + 'static>(xs: List_::List<T>, ys: List_::List<T>) -> List_::List<T> {
        List_::fold(
            Func2::new(move |acc: List_::List<T>, x: T| List_::cons(x, acc)),
            ys,
            List_::reverse(xs),
        )
    }
    pub fn choose<T: Clone + 'static, U: Clone + 'static>(
        chooser: Func1<T, Option<U>>,
        xs: List_::List<T>,
    ) -> List_::List<U> {
        let root: MutCell<Option<LrcPtr<List_::Node_1<U>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<U>>>);
        let node: MutCell<Option<LrcPtr<List_::Node_1<U>>>> = MutCell::new(root.get().clone());
        let xs_1: MutCell<List_::List<T>> = MutCell::new(xs);
        while !List_::isEmpty(xs_1.get().clone()) {
            {
                let matchValue: Option<U> = chooser(List_::head(xs_1.get().clone()));
                match &matchValue {
                    None => (),
                    Some(matchValue_0_0) => {
                        let x: U = matchValue_0_0.clone();
                        node.set(List_::appendConsNoTail(x, node.get().clone()));
                        if root.get().clone().is_none() {
                            root.set(node.get().clone());
                        }
                    }
                }
            }
            xs_1.set(List_::tail(xs_1.get().clone()))
        }
        List_::mkList(root.get().clone())
    }
    pub fn concat<T: Clone + 'static>(sources: List_::List<List_::List<T>>) -> List_::List<T> {
        let root: MutCell<Option<LrcPtr<List_::Node_1<T>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<T>>>);
        let node: MutCell<Option<LrcPtr<List_::Node_1<T>>>> = MutCell::new(root.get().clone());
        let xs: MutCell<List_::List<List_::List<T>>> = MutCell::new(sources);
        let ys: MutCell<List_::List<T>> = MutCell::new(List_::empty());
        while !List_::isEmpty(xs.get().clone()) {
            ys.set(List_::head(xs.get().clone()));
            while !List_::isEmpty(ys.get().clone()) {
                node.set({
                    let node_1: Option<LrcPtr<List_::Node_1<T>>> = node.get().clone();
                    List_::appendConsNoTail(List_::head(ys.get().clone()), node_1)
                });
                if root.get().clone().is_none() {
                    root.set(node.get().clone());
                }
                ys.set(List_::tail(ys.get().clone()))
            }
            xs.set(List_::tail(xs.get().clone()))
        }
        List_::mkList(root.get().clone())
    }
    pub fn compareWith<T: Clone + 'static>(
        comparer: Func2<T, T, i32>,
        xs: List_::List<T>,
        ys: List_::List<T>,
    ) -> i32 {
        let comparer = MutCell::new(comparer.clone());
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        let ys: MutCell<List_::List<T>> = MutCell::new(ys.clone());
        '_compareWith: loop {
            break '_compareWith ({
                let matchValue: bool = List_::isEmpty(xs.get().clone());
                let matchValue_1: bool = List_::isEmpty(ys.get().clone());
                if matchValue {
                    if matchValue_1 {
                        0_i32
                    } else {
                        -1_i32
                    }
                } else {
                    if matchValue_1 {
                        1_i32
                    } else {
                        let c: i32 =
                            comparer(List_::head(xs.get().clone()), List_::head(ys.get().clone()));
                        if c == 0_i32 {
                            let comparer_temp = comparer.get().clone();
                            let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                            let ys_temp: List_::List<T> = List_::tail(ys.get().clone());
                            comparer.set(comparer_temp);
                            xs.set(xs_temp);
                            ys.set(ys_temp);
                            continue '_compareWith;
                        } else {
                            c
                        }
                    }
                }
            });
        }
    }
    pub fn compareTo<T: PartialOrd + Clone + 'static>(
        xs: List_::List<T>,
        ys: List_::List<T>,
    ) -> i32 {
        List_::compareWith(Func2::new(move |e: T, e_1: T| compare(e, e_1)), xs, ys)
    }
    pub fn equals<T: Eq + core::hash::Hash + Clone + 'static>(
        xs: List_::List<T>,
        ys: List_::List<T>,
    ) -> bool {
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        let ys: MutCell<List_::List<T>> = MutCell::new(ys.clone());
        '_equals: loop {
            break '_equals ({
                let matchValue: bool = List_::isEmpty(xs.get().clone());
                let matchValue_1: bool = List_::isEmpty(ys.get().clone());
                if matchValue {
                    if matchValue_1 {
                        true
                    } else {
                        false
                    }
                } else {
                    if matchValue_1 {
                        false
                    } else {
                        if List_::head(xs.get().clone()) == List_::head(ys.get().clone()) {
                            let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                            let ys_temp: List_::List<T> = List_::tail(ys.get().clone());
                            xs.set(xs_temp);
                            ys.set(ys_temp);
                            continue '_equals;
                        } else {
                            false
                        }
                    }
                }
            });
        }
    }
    pub fn exists<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> bool {
        let predicate = MutCell::new(predicate.clone());
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        '_exists: loop {
            break '_exists (if List_::isEmpty(xs.get().clone()) {
                false
            } else {
                if predicate(List_::head(xs.get().clone())) {
                    true
                } else {
                    let predicate_temp = predicate.get().clone();
                    let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                    predicate.set(predicate_temp);
                    xs.set(xs_temp);
                    continue '_exists;
                }
            });
        }
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone + 'static>(
        predicate: Func2<T1, T2, bool>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) -> bool {
        let predicate = MutCell::new(predicate.clone());
        let xs: MutCell<List_::List<T1>> = MutCell::new(xs.clone());
        let ys: MutCell<List_::List<T2>> = MutCell::new(ys.clone());
        '_exists2: loop {
            break '_exists2 ({
                let matchValue: bool = List_::isEmpty(xs.get().clone());
                let matchValue_1: bool = List_::isEmpty(ys.get().clone());
                if matchValue {
                    if matchValue_1 {
                        false
                    } else {
                        panic!(
                            "{}",
                            append_1(
                                SR::listsHadDifferentLengths(),
                                string(" (Parameter \'list2\')")
                            ),
                        )
                    }
                } else {
                    if matchValue_1 {
                        panic!(
                            "{}",
                            append_1(
                                SR::listsHadDifferentLengths(),
                                string(" (Parameter \'list2\')")
                            ),
                        )
                    } else {
                        if predicate(List_::head(xs.get().clone()), List_::head(ys.get().clone())) {
                            true
                        } else {
                            let predicate_temp = predicate.get().clone();
                            let xs_temp: List_::List<T1> = List_::tail(xs.get().clone());
                            let ys_temp: List_::List<T2> = List_::tail(ys.get().clone());
                            predicate.set(predicate_temp);
                            xs.set(xs_temp);
                            ys.set(ys_temp);
                            continue '_exists2;
                        }
                    }
                }
            });
        }
    }
    pub fn contains<T: Eq + core::hash::Hash + Clone + 'static>(
        value: T,
        xs: List_::List<T>,
    ) -> bool {
        List_::exists(
            Func1::new({
                let value = value.clone();
                move |x: T| x == value.clone()
            }),
            xs,
        )
    }
    pub fn filter<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        List_::choose(
            Func1::new({
                let predicate = predicate.clone();
                move |x: T| {
                    if predicate(x.clone()) {
                        Some(x)
                    } else {
                        None::<T>
                    }
                }
            }),
            xs,
        )
    }
    pub fn map<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func1<T, U>,
        xs: List_::List<T>,
    ) -> List_::List<U> {
        List_::unfold(
            Func1::new({
                let mapping = mapping.clone();
                move |xs_1: List_::List<T>| {
                    if List_::isEmpty(xs_1.clone()) {
                        None::<LrcPtr<(U, List_::List<T>)>>
                    } else {
                        Some(LrcPtr::new((
                            mapping(List_::head(xs_1.clone())),
                            List_::tail(xs_1),
                        )))
                    }
                }
            }),
            xs,
        )
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<i32, T, U>,
        xs: List_::List<T>,
    ) -> List_::List<U> {
        List_::unfold(
            Func1::new({
                let mapping = mapping.clone();
                move |tupledArg: LrcPtr<(i32, List_::List<T>)>| {
                    let i: i32 = tupledArg.0.clone();
                    let xs_1: List_::List<T> = tupledArg.1.clone();
                    if List_::isEmpty(xs_1.clone()) {
                        None::<LrcPtr<(U, LrcPtr<(i32, List_::List<T>)>)>>
                    } else {
                        Some(LrcPtr::new((
                            mapping(i, List_::head(xs_1.clone())),
                            LrcPtr::new((i + 1_i32, List_::tail(xs_1))),
                        )))
                    }
                }
            }),
            LrcPtr::new((0_i32, xs)),
        )
    }
    pub fn collect<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func1<T, List_::List<U>>,
        xs: List_::List<T>,
    ) -> List_::List<U> {
        let root: MutCell<Option<LrcPtr<List_::Node_1<U>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<U>>>);
        let node: MutCell<Option<LrcPtr<List_::Node_1<U>>>> = MutCell::new(root.get().clone());
        let xs_1: MutCell<List_::List<T>> = MutCell::new(xs);
        let ys: MutCell<List_::List<U>> = MutCell::new(List_::empty());
        while !List_::isEmpty(xs_1.get().clone()) {
            ys.set(mapping(List_::head(xs_1.get().clone())));
            while !List_::isEmpty(ys.get().clone()) {
                node.set({
                    let node_1: Option<LrcPtr<List_::Node_1<U>>> = node.get().clone();
                    List_::appendConsNoTail(List_::head(ys.get().clone()), node_1)
                });
                if root.get().clone().is_none() {
                    root.set(node.get().clone());
                }
                ys.set(List_::tail(ys.get().clone()))
            }
            xs_1.set(List_::tail(xs_1.get().clone()))
        }
        List_::mkList(root.get().clone())
    }
    pub fn indexed<T: Clone + 'static>(xs: List_::List<T>) -> List_::List<LrcPtr<(i32, T)>> {
        List_::mapIndexed(Func2::new(move |i: i32, x: T| LrcPtr::new((i, x))), xs)
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<T1, T2, U>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) -> List_::List<U> {
        List_::unfold(
            Func1::new({
                let mapping = mapping.clone();
                move |tupledArg: LrcPtr<(List_::List<T1>, List_::List<T2>)>| {
                    let xs_1: List_::List<T1> = tupledArg.0.clone();
                    let ys_1: List_::List<T2> = tupledArg.1.clone();
                    if if List_::isEmpty(xs_1.clone()) {
                        true
                    } else {
                        List_::isEmpty(ys_1.clone())
                    } {
                        None::<LrcPtr<(U, LrcPtr<(List_::List<T1>, List_::List<T2>)>)>>
                    } else {
                        Some(LrcPtr::new((
                            mapping(List_::head(xs_1.clone()), List_::head(ys_1.clone())),
                            LrcPtr::new((List_::tail(xs_1), List_::tail(ys_1))),
                        )))
                    }
                }
            }),
            LrcPtr::new((xs, ys)),
        )
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone + 'static>(
        mapping: Func3<i32, T1, T2, U>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) -> List_::List<U> {
        List_::unfold(
            Func1::new({
                let mapping = mapping.clone();
                move |tupledArg: LrcPtr<(i32, List_::List<T1>, List_::List<T2>)>| {
                    let i: i32 = tupledArg.0.clone();
                    let xs_1: List_::List<T1> = tupledArg.1.clone();
                    let ys_1: List_::List<T2> = tupledArg.2.clone();
                    if if List_::isEmpty(xs_1.clone()) {
                        true
                    } else {
                        List_::isEmpty(ys_1.clone())
                    } {
                        None::<LrcPtr<(U, LrcPtr<(i32, List_::List<T1>, List_::List<T2>)>)>>
                    } else {
                        Some(LrcPtr::new((
                            mapping(i, List_::head(xs_1.clone()), List_::head(ys_1.clone())),
                            LrcPtr::new((i + 1_i32, List_::tail(xs_1), List_::tail(ys_1))),
                        )))
                    }
                }
            }),
            LrcPtr::new((0_i32, xs, ys)),
        )
    }
    pub fn map3<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        U: Clone + 'static,
    >(
        mapping: Func3<T1, T2, T3, U>,
        xs: List_::List<T1>,
        ys: List_::List<T2>,
        zs: List_::List<T3>,
    ) -> List_::List<U> {
        List_::unfold(
            Func1::new({
                let mapping = mapping.clone();
                move |tupledArg: LrcPtr<(List_::List<T1>, List_::List<T2>, List_::List<T3>)>| {
                    let xs_1: List_::List<T1> = tupledArg.0.clone();
                    let ys_1: List_::List<T2> = tupledArg.1.clone();
                    let zs_1: List_::List<T3> = tupledArg.2.clone();
                    if if if List_::isEmpty(xs_1.clone()) {
                        true
                    } else {
                        List_::isEmpty(ys_1.clone())
                    } {
                        true
                    } else {
                        List_::isEmpty(zs_1.clone())
                    } {
                        None::<
                            LrcPtr<(
                                U,
                                LrcPtr<(List_::List<T1>, List_::List<T2>, List_::List<T3>)>,
                            )>,
                        >
                    } else {
                        Some(LrcPtr::new((
                            mapping(
                                List_::head(xs_1.clone()),
                                List_::head(ys_1.clone()),
                                List_::head(zs_1.clone()),
                            ),
                            LrcPtr::new((List_::tail(xs_1), List_::tail(ys_1), List_::tail(zs_1))),
                        )))
                    }
                }
            }),
            LrcPtr::new((xs, ys, zs)),
        )
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<State, T, LrcPtr<(U, State)>>,
        state: State,
        xs: List_::List<T>,
    ) -> LrcPtr<(List_::List<U>, State)> {
        let acc: LrcPtr<MutCell<State>> = LrcPtr::new(MutCell::new(state));
        let gen = Func1::new({
            let acc = acc.clone();
            let mapping = mapping.clone();
            move |xs_1: List_::List<T>| {
                if List_::isEmpty(xs_1.clone()) {
                    None::<LrcPtr<(U, List_::List<T>)>>
                } else {
                    let m: LrcPtr<(U, State)> =
                        mapping(acc.get().clone(), List_::head(xs_1.clone()));
                    acc.set(m.1.clone());
                    Some(LrcPtr::new((m.0.clone(), List_::tail(xs_1))))
                }
            }
        });
        LrcPtr::new((List_::unfold(gen, xs), acc.get().clone()))
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<T, State, LrcPtr<(U, State)>>,
        xs: List_::List<T>,
        state: State,
    ) -> LrcPtr<(List_::List<U>, State)> {
        let ys: LrcPtr<MutCell<List_::List<U>>> = LrcPtr::new(MutCell::new(List_::empty()));
        let folder = Func2::new({
            let mapping = mapping.clone();
            let ys = ys.clone();
            move |acc: State, x: T| {
                let m: LrcPtr<(U, State)> = mapping(x, acc);
                ys.set(List_::cons(m.0.clone(), ys.get().clone()));
                m.1.clone()
            }
        });
        let st: State = List_::fold(folder, state, List_::reverse(xs));
        LrcPtr::new((ys.get().clone(), st))
    }
    pub fn tryPick<T: Clone + 'static, U: Clone + 'static>(
        chooser: Func1<T, Option<U>>,
        xs: List_::List<T>,
    ) -> Option<U> {
        fn inner_loop<T: Clone + 'static, U: Clone + 'static>(
            chooser_1: Func1<T, Option<U>>,
            xs_1: List_::List<T>,
        ) -> Option<U> {
            let chooser_1 = MutCell::new(chooser_1.clone());
            let xs_1: MutCell<List_::List<T>> = MutCell::new(xs_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if List_::isEmpty(xs_1.get().clone()) {
                    None::<U>
                } else {
                    let matchValue: Option<U> = chooser_1(List_::head(xs_1.get().clone()));
                    match &matchValue {
                        None => {
                            let chooser_1_temp = chooser_1.get().clone();
                            let xs_1_temp: List_::List<T> = List_::tail(xs_1.get().clone());
                            chooser_1.set(chooser_1_temp);
                            xs_1.set(xs_1_temp);
                            continue '_inner_loop;
                        }
                        _ => matchValue.clone(),
                    }
                });
            }
        }
        inner_loop(chooser, xs)
    }
    pub fn pick<T: Clone + 'static, U: Clone + 'static>(
        chooser: Func1<T, Option<U>>,
        xs: List_::List<T>,
    ) -> U {
        let matchValue: Option<U> = List_::tryPick(chooser, xs);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFind<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> Option<T> {
        List_::tryPick(
            Func1::new({
                let predicate = predicate.clone();
                move |x: T| {
                    if predicate(x.clone()) {
                        Some(x)
                    } else {
                        None::<T>
                    }
                }
            }),
            xs,
        )
    }
    pub fn find<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> T {
        let matchValue: Option<T> = List_::tryFind(predicate, xs);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindBack<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> Option<T> {
        tryFindBack_1(predicate, List_::toArray(xs))
    }
    pub fn findBack<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> T {
        let matchValue: Option<T> = List_::tryFindBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndex<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> Option<i32> {
        fn inner_loop<T: Clone + 'static>(
            i: i32,
            predicate_1: Func1<T, bool>,
            xs_1: List_::List<T>,
        ) -> Option<i32> {
            let i: MutCell<i32> = MutCell::new(i);
            let predicate_1 = MutCell::new(predicate_1.clone());
            let xs_1: MutCell<List_::List<T>> = MutCell::new(xs_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if List_::isEmpty(xs_1.get().clone()) {
                    None::<i32>
                } else {
                    if predicate_1(List_::head(xs_1.get().clone())) {
                        Some(i.get().clone())
                    } else {
                        let i_temp: i32 = i.get().clone() + 1_i32;
                        let predicate_1_temp = predicate_1.get().clone();
                        let xs_1_temp: List_::List<T> = List_::tail(xs_1.get().clone());
                        i.set(i_temp);
                        predicate_1.set(predicate_1_temp);
                        xs_1.set(xs_1_temp);
                        continue '_inner_loop;
                    }
                });
            }
        }
        inner_loop(0_i32, predicate, xs)
    }
    pub fn findIndex<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> i32 {
        let matchValue: Option<i32> = List_::tryFindIndex(predicate, xs);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndexBack<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> Option<i32> {
        tryFindIndexBack_1(predicate, List_::toArray(xs))
    }
    pub fn findIndexBack<T: Clone + 'static>(predicate: Func1<T, bool>, xs: List_::List<T>) -> i32 {
        let matchValue: Option<i32> = List_::tryFindIndexBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryItem<T: Clone + 'static>(index: i32, xs: List_::List<T>) -> Option<T> {
        fn inner_loop<T: Clone + 'static>(i: i32, xs_1: List_::List<T>) -> Option<T> {
            let i: MutCell<i32> = MutCell::new(i);
            let xs_1: MutCell<List_::List<T>> = MutCell::new(xs_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if List_::isEmpty(xs_1.get().clone()) {
                    None::<T>
                } else {
                    if i.get().clone() == 0_i32 {
                        Some(List_::head(xs_1.get().clone()))
                    } else {
                        if i.get().clone() < 0_i32 {
                            None::<T>
                        } else {
                            let i_temp: i32 = i.get().clone() - 1_i32;
                            let xs_1_temp: List_::List<T> = List_::tail(xs_1.get().clone());
                            i.set(i_temp);
                            xs_1.set(xs_1_temp);
                            continue '_inner_loop;
                        }
                    }
                });
            }
        }
        inner_loop(index, xs)
    }
    pub fn item<T: Clone + 'static>(index: i32, xs: List_::List<T>) -> T {
        let matchValue: Option<T> = List_::tryItem(index, xs);
        match &matchValue {
            Some(matchValue_0_0) => matchValue_0_0.clone(),
            _ => panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            ),
        }
    }
    pub fn initialize<T: Clone + 'static>(
        count: i32,
        initializer: Func1<i32, T>,
    ) -> List_::List<T> {
        List_::unfold(
            Func1::new({
                let count = count.clone();
                let initializer = initializer.clone();
                move |i: i32| {
                    if i < count {
                        Some(LrcPtr::new((initializer(i), i + 1_i32)))
                    } else {
                        None::<LrcPtr<(T, i32)>>
                    }
                }
            }),
            0_i32,
        )
    }
    pub fn pairwise<T: Clone + 'static>(xs: List_::List<T>) -> List_::List<LrcPtr<(T, T)>> {
        List_::ofArray(pairwise_1(List_::toArray(xs)))
    }
    pub fn partition<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> LrcPtr<(List_::List<T>, List_::List<T>)> {
        let root1: MutCell<Option<LrcPtr<List_::Node_1<T>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<T>>>);
        let root2: MutCell<Option<LrcPtr<List_::Node_1<T>>>> =
            MutCell::new(None::<LrcPtr<List_::Node_1<T>>>);
        let node1: MutCell<Option<LrcPtr<List_::Node_1<T>>>> = MutCell::new(root1.get().clone());
        let node2: MutCell<Option<LrcPtr<List_::Node_1<T>>>> = MutCell::new(root2.get().clone());
        let xs_1: MutCell<List_::List<T>> = MutCell::new(xs);
        while !List_::isEmpty(xs_1.get().clone()) {
            let x: T = List_::head(xs_1.get().clone());
            if predicate(x.clone()) {
                node1.set(List_::appendConsNoTail(x.clone(), node1.get().clone()));
                if root1.get().clone().is_none() {
                    root1.set(node1.get().clone());
                }
            } else {
                node2.set(List_::appendConsNoTail(x, node2.get().clone()));
                if root2.get().clone().is_none() {
                    root2.set(node2.get().clone());
                }
            }
            xs_1.set(List_::tail(xs_1.get().clone()))
        }
        LrcPtr::new((
            List_::mkList(root1.get().clone()),
            List_::mkList(root2.get().clone()),
        ))
    }
    pub fn reduce<T: Clone + 'static>(reduction: Func2<T, T, T>, xs: List_::List<T>) -> T {
        if List_::isEmpty(xs.clone()) {
            panic!("{}", SR::inputListWasEmpty(),);
        }
        List_::fold(reduction, List_::head(xs.clone()), List_::tail(xs))
    }
    pub fn reduceBack<T: Clone + 'static>(reduction: Func2<T, T, T>, xs: List_::List<T>) -> T {
        if List_::isEmpty(xs.clone()) {
            panic!("{}", SR::inputListWasEmpty(),);
        }
        List_::foldBack(reduction, List_::tail(xs.clone()), List_::head(xs))
    }
    pub fn replicate<T: Clone + 'static>(count: i32, initial: T) -> List_::List<T> {
        List_::initialize(
            count,
            Func1::new({
                let initial = initial.clone();
                move |_arg: i32| initial.clone()
            }),
        )
    }
    pub fn unzip<a: Clone + 'static, b: Clone + 'static>(
        xs: List_::List<LrcPtr<(a, b)>>,
    ) -> LrcPtr<(List_::List<a>, List_::List<b>)> {
        List_::foldBack(
            Func2::new(
                move |tupledArg: LrcPtr<(a, b)>,
                      tupledArg_1: LrcPtr<(List_::List<a>, List_::List<b>)>| {
                    LrcPtr::new((
                        List_::cons(tupledArg.0.clone(), tupledArg_1.0.clone()),
                        List_::cons(tupledArg.1.clone(), tupledArg_1.1.clone()),
                    ))
                },
            ),
            xs,
            LrcPtr::new((List_::empty(), List_::empty())),
        )
    }
    pub fn unzip3<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        xs: List_::List<LrcPtr<(a, b, c)>>,
    ) -> LrcPtr<(List_::List<a>, List_::List<b>, List_::List<c>)> {
        List_::foldBack(Func2::new(move
                                       |tupledArg: LrcPtr<(a, b, c)>,
                                        tupledArg_1:
                                            LrcPtr<(List_::List<a>,
                                                    List_::List<b>,
                                                    List_::List<c>)>|
                                       LrcPtr::new((List_::cons(tupledArg.0.clone(),
                                                                tupledArg_1.0.clone()),
                                                    List_::cons(tupledArg.1.clone(),
                                                                tupledArg_1.1.clone()),
                                                    List_::cons(tupledArg.2.clone(),
                                                                tupledArg_1.2.clone())))),
                        xs,
                        LrcPtr::new((List_::empty(), List_::empty(),
                                     List_::empty())))
    }
    pub fn zip<a: Clone + 'static, b: Clone + 'static>(
        xs: List_::List<a>,
        ys: List_::List<b>,
    ) -> List_::List<LrcPtr<(a, b)>> {
        List_::map2(Func2::new(move |x: a, y: b| LrcPtr::new((x, y))), xs, ys)
    }
    pub fn zip3<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        xs: List_::List<a>,
        ys: List_::List<b>,
        zs: List_::List<c>,
    ) -> List_::List<LrcPtr<(a, b, c)>> {
        List_::map3(
            Func3::new(move |x: a, y: b, z: c| LrcPtr::new((x, y, z))),
            xs,
            ys,
            zs,
        )
    }
    pub fn sortWith<T: Clone + 'static>(
        comparer: Func2<T, T, i32>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let arr: Array<T> = List_::toArray(xs);
        sortInPlaceWith(comparer, arr.clone());
        List_::ofArray(arr)
    }
    pub fn sort<T: PartialOrd + Clone + 'static>(xs: List_::List<T>) -> List_::List<T> {
        List_::sortWith(Func2::new(move |e: T, e_1: T| compare(e, e_1)), xs)
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        List_::sortWith(
            Func2::new({
                let projection = projection.clone();
                move |x: T, y: T| compare(projection(x), projection(y))
            }),
            xs,
        )
    }
    pub fn sortDescending<T: PartialOrd + Clone + 'static>(xs: List_::List<T>) -> List_::List<T> {
        List_::sortWith(Func2::new(move |x: T, y: T| compare(x, y) * -1_i32), xs)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        List_::sortWith(
            Func2::new({
                let projection = projection.clone();
                move |x: T, y: T| compare(projection(x), projection(y)) * -1_i32
            }),
            xs,
        )
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone + 'static>(xs: List_::List<T>) -> T {
        List_::fold(Func2::new(move |acc: T, x: T| acc + x), getZero(), xs)
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default + Clone + 'static>(
        projection: Func1<T, U>,
        xs: List_::List<T>,
    ) -> U {
        List_::fold(
            Func2::new({
                let projection = projection.clone();
                move |acc: U, x: T| acc + projection(x)
            }),
            getZero(),
            xs,
        )
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        xs: List_::List<T>,
    ) -> T {
        List_::reduce(
            Func2::new({
                let projection = projection.clone();
                move |x: T, y: T| {
                    if projection(x.clone()) > projection(y.clone()) {
                        x
                    } else {
                        y
                    }
                }
            }),
            xs,
        )
    }
    pub fn max<T: PartialOrd + Clone + 'static>(xs: List_::List<T>) -> T {
        List_::reduce(
            Func2::new(move |x: T, y: T| if x.clone() > y.clone() { x } else { y }),
            xs,
        )
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        xs: List_::List<T>,
    ) -> T {
        List_::reduce(
            Func2::new({
                let projection = projection.clone();
                move |x: T, y: T| {
                    if projection(x.clone()) < projection(y.clone()) {
                        x
                    } else {
                        y
                    }
                }
            }),
            xs,
        )
    }
    pub fn min<T: PartialOrd + Clone + 'static>(xs: List_::List<T>) -> T {
        List_::reduce(
            Func2::new(move |x: T, y: T| if x.clone() < y.clone() { x } else { y }),
            xs,
        )
    }
    pub fn average<
        T: core::ops::Add<Output = T>
            + core::ops::Div<Output = T>
            + From<i32>
            + Default
            + Clone
            + 'static,
    >(
        xs: List_::List<T>,
    ) -> T {
        if List_::isEmpty(xs.clone()) {
            panic!("{}", SR::inputListWasEmpty(),);
        }
        {
            let count: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(0_i32));
            let folder = Func2::new({
                let count = count.clone();
                move |acc: T, x: T| {
                    count.set(count.get().clone() + 1_i32);
                    acc + x
                }
            });
            List_::fold(folder, getZero(), xs) / T::from(count.get().clone())
        }
    }
    pub fn averageBy<
        T: Clone + 'static,
        U: core::ops::Add<Output = U>
            + core::ops::Div<Output = U>
            + From<i32>
            + Default
            + Clone
            + 'static,
    >(
        projection: Func1<T, U>,
        xs: List_::List<T>,
    ) -> U {
        if List_::isEmpty(xs.clone()) {
            panic!("{}", SR::inputListWasEmpty(),);
        }
        {
            let count: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(0_i32));
            let folder = Func2::new({
                let count = count.clone();
                let projection = projection.clone();
                move |acc: U, x: T| {
                    count.set(count.get().clone() + 1_i32);
                    acc + projection(x)
                }
            });
            List_::fold(folder, getZero(), xs) / U::from(count.get().clone())
        }
    }
    pub fn permute<T: Clone + 'static>(
        indexMap: Func1<i32, i32>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        List_::ofArray(permute_1(indexMap, List_::toArray(xs)))
    }
    pub fn chunkBySize<T: Clone + 'static>(
        chunkSize: i32,
        xs: List_::List<T>,
    ) -> List_::List<List_::List<T>> {
        List_::ofArray(map_1(
            Func1::new(move |xs_2: Array<T>| List_::ofArray(xs_2)),
            chunkBySize_1(chunkSize, List_::toArray(xs)),
        ))
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone + 'static>(
        xs: List_::List<T1>,
        ys: List_::List<T2>,
    ) -> List_::List<LrcPtr<(T1, T2)>> {
        let root: LrcPtr<MutCell<Option<LrcPtr<List_::Node_1<LrcPtr<(T1, T2)>>>>>> = LrcPtr::new(
            MutCell::new(None::<LrcPtr<List_::Node_1<LrcPtr<(T1, T2)>>>>),
        );
        let node: LrcPtr<MutCell<Option<LrcPtr<List_::Node_1<LrcPtr<(T1, T2)>>>>>> =
            LrcPtr::new(MutCell::new(root.get().clone()));
        List_::iterate(
            Func1::new({
                let node = node.clone();
                let root = root.clone();
                let ys = ys.clone();
                move |x: T1| {
                    List_::iterate(
                        Func1::new({
                            let node = node.clone();
                            let root = root.clone();
                            let x = x.clone();
                            move |y: T2| {
                                node.set(List_::appendConsNoTail(
                                    LrcPtr::new((x.clone(), y)),
                                    node.get().clone(),
                                ));
                                if root.get().clone().is_none() {
                                    root.set(node.get().clone());
                                }
                            }
                        }),
                        ys.clone(),
                    )
                }
            }),
            xs,
        );
        List_::mkList(root.get().clone())
    }
    pub fn scan<State: Clone + 'static, T: Clone + 'static>(
        folder: Func2<State, T, State>,
        state: State,
        xs: List_::List<T>,
    ) -> List_::List<State> {
        List_::ofArray(scan_1(folder, state, List_::toArray(xs)))
    }
    pub fn scanBack<T: Clone + 'static, State: Clone + 'static>(
        folder: Func2<T, State, State>,
        xs: List_::List<T>,
        state: State,
    ) -> List_::List<State> {
        List_::ofArray(scanBack_1(folder, List_::toArray(xs), state))
    }
    pub fn skip<T: Clone + 'static>(count: i32, xs: List_::List<T>) -> List_::List<T> {
        let count: MutCell<i32> = MutCell::new(count);
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        '_skip: loop {
            break '_skip (if count.get().clone() <= 0_i32 {
                xs.get().clone()
            } else {
                if List_::isEmpty(xs.get().clone()) {
                    panic!(
                        "{}",
                        append_1(SR::notEnoughElements(), string(" (Parameter \'list\')")),
                    );
                }
                {
                    let count_temp: i32 = count.get().clone() - 1_i32;
                    let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                    count.set(count_temp);
                    xs.set(xs_temp);
                    continue '_skip;
                }
            });
        }
    }
    pub fn skipSafe<T: Clone + 'static>(count: i32, xs: List_::List<T>) -> List_::List<T> {
        let count: MutCell<i32> = MutCell::new(count);
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        '_skipSafe: loop {
            break '_skipSafe (if if count.get().clone() <= 0_i32 {
                true
            } else {
                List_::isEmpty(xs.get().clone())
            } {
                xs.get().clone()
            } else {
                let count_temp: i32 = count.get().clone() - 1_i32;
                let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                count.set(count_temp);
                xs.set(xs_temp);
                continue '_skipSafe;
            });
        }
    }
    pub fn skipWhile<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let predicate = MutCell::new(predicate.clone());
        let xs: MutCell<List_::List<T>> = MutCell::new(xs.clone());
        '_skipWhile: loop {
            break '_skipWhile (if List_::isEmpty(xs.get().clone()) {
                xs.get().clone()
            } else {
                if !predicate(List_::head(xs.get().clone())) {
                    xs.get().clone()
                } else {
                    let predicate_temp = predicate.get().clone();
                    let xs_temp: List_::List<T> = List_::tail(xs.get().clone());
                    predicate.set(predicate_temp);
                    xs.set(xs_temp);
                    continue '_skipWhile;
                }
            });
        }
    }
    pub fn take<T: Clone + 'static>(count: i32, xs: List_::List<T>) -> List_::List<T> {
        if count < 0_i32 {
            panic!(
                "{}",
                append_1(
                    SR::inputMustBeNonNegative(),
                    string(" (Parameter \'count\')")
                ),
            );
        }
        List_::unfold(
            Func1::new(move |tupledArg: LrcPtr<(i32, List_::List<T>)>| {
                let i: i32 = tupledArg.0.clone();
                let xs_1: List_::List<T> = tupledArg.1.clone();
                if i > 0_i32 {
                    if List_::isEmpty(xs_1.clone()) {
                        panic!(
                            "{}",
                            append_1(SR::notEnoughElements(), string(" (Parameter \'list\')")),
                        );
                    }
                    Some(LrcPtr::new((
                        List_::head(xs_1.clone()),
                        LrcPtr::new((i - 1_i32, List_::tail(xs_1))),
                    )))
                } else {
                    None::<LrcPtr<(T, LrcPtr<(i32, List_::List<T>)>)>>
                }
            }),
            LrcPtr::new((count, xs)),
        )
    }
    pub fn takeWhile<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        List_::unfold(
            Func1::new({
                let predicate = predicate.clone();
                move |xs_1: List_::List<T>| {
                    if if !List_::isEmpty(xs_1.clone()) {
                        predicate(List_::head(xs_1.clone()))
                    } else {
                        false
                    } {
                        Some(LrcPtr::new((List_::head(xs_1.clone()), List_::tail(xs_1))))
                    } else {
                        None::<LrcPtr<(T, List_::List<T>)>>
                    }
                }
            }),
            xs,
        )
    }
    pub fn truncate<T: Clone + 'static>(count: i32, xs: List_::List<T>) -> List_::List<T> {
        List_::unfold(
            Func1::new(move |tupledArg: LrcPtr<(i32, List_::List<T>)>| {
                let i: i32 = tupledArg.0.clone();
                let xs_1: List_::List<T> = tupledArg.1.clone();
                if if i > 0_i32 {
                    !List_::isEmpty(xs_1.clone())
                } else {
                    false
                } {
                    Some(LrcPtr::new((
                        List_::head(xs_1.clone()),
                        LrcPtr::new((i - 1_i32, List_::tail(xs_1))),
                    )))
                } else {
                    None::<LrcPtr<(T, LrcPtr<(i32, List_::List<T>)>)>>
                }
            }),
            LrcPtr::new((count, xs)),
        )
    }
    pub fn getSlice<T: Clone + 'static>(
        lower: Option<i32>,
        upper: Option<i32>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        if lower.is_some() {
            if upper.is_some() {
                let start_1: i32 = getValue(lower.clone());
                let stop_1: i32 = getValue(upper.clone());
                List_::truncate(
                    stop_1 - start_1 + 1_i32,
                    List_::skipSafe(start_1, xs.clone()),
                )
            } else {
                let start: i32 = getValue(lower.clone());
                List_::skipSafe(start, xs.clone())
            }
        } else {
            if upper.is_some() {
                let stop: i32 = getValue(upper.clone());
                List_::truncate(stop + 1_i32, xs.clone())
            } else {
                xs.clone()
            }
        }
    }
    pub fn splitAt<T: Clone + 'static>(
        index: i32,
        xs: List_::List<T>,
    ) -> LrcPtr<(List_::List<T>, List_::List<T>)> {
        if index < 0_i32 {
            panic!(
                "{}",
                append_1(
                    SR::inputMustBeNonNegative(),
                    string(" (Parameter \'index\')")
                ),
            );
        }
        if index > List_::length(xs.clone()) {
            panic!(
                "{}",
                append_1(SR::notEnoughElements(), string(" (Parameter \'index\')")),
            );
        }
        LrcPtr::new((List_::take(index, xs.clone()), List_::skip(index, xs)))
    }
    pub fn exactlyOne<T: Clone + 'static>(xs: List_::List<T>) -> T {
        if List_::isEmpty(xs.clone()) {
            panic!(
                "{}",
                append_1(SR::inputSequenceEmpty(), string(" (Parameter \'list\')")),
            )
        } else {
            if List_::isEmpty(List_::tail(xs.clone())) {
                List_::head(xs)
            } else {
                panic!(
                    "{}",
                    append_1(SR::inputSequenceTooLong(), string(" (Parameter \'list\')")),
                )
            }
        }
    }
    pub fn tryExactlyOne<T: Clone + 'static>(xs: List_::List<T>) -> Option<T> {
        if if !List_::isEmpty(xs.clone()) {
            List_::isEmpty(List_::tail(xs.clone()))
        } else {
            false
        } {
            Some(List_::head(xs))
        } else {
            None::<T>
        }
    }
    pub fn r#where<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        List_::filter(predicate, xs)
    }
    pub fn windowed<T: Clone + 'static>(
        windowSize: i32,
        xs: List_::List<T>,
    ) -> List_::List<List_::List<T>> {
        List_::ofArray(map_1(
            Func1::new(move |xs_2: Array<T>| List_::ofArray(xs_2)),
            windowed_1(windowSize, List_::toArray(xs)),
        ))
    }
    pub fn splitInto<T: Clone + 'static>(
        chunks: i32,
        xs: List_::List<T>,
    ) -> List_::List<List_::List<T>> {
        List_::ofArray(map_1(
            Func1::new(move |xs_2: Array<T>| List_::ofArray(xs_2)),
            splitInto_1(chunks, List_::toArray(xs)),
        ))
    }
    pub fn transpose<T: Clone + 'static>(
        lists: List_::List<List_::List<T>>,
    ) -> List_::List<List_::List<T>> {
        if List_::isEmpty(lists.clone()) {
            List_::empty()
        } else {
            let tRows: List_::List<List_::List<T>> = List_::map(
                Func1::new(move |x: T| List_::singleton(x)),
                List_::head(lists.clone()),
            );
            let nodes: Array<Option<LrcPtr<List_::Node_1<T>>>> = List_::toArray(List_::map(
                Func1::new(move |xs_1: List_::List<T>| xs_1.root.clone()),
                tRows.clone(),
            ));
            {
                let action_1 = Func1::new({
                    let nodes = nodes.clone();
                    move |xs_5: List_::List<T>| {
                        let len: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(0_i32));
                        {
                            let action = Func2::new({
                                let len = len.clone();
                                let nodes = nodes.clone();
                                move |i: i32, x_1: T| {
                                    len.set(len.get().clone() + 1_i32);
                                    nodes.get_mut()[i as usize] =
                                        List_::appendConsNoTail(x_1, nodes[i].clone())
                                }
                            });
                            List_::iterateIndexed(action, xs_5)
                        }
                        if len.get().clone() != count_2(nodes.clone()) {
                            panic!(
                                "{}",
                                append_1(
                                    SR::listsHadDifferentLengths(),
                                    string(" (Parameter \'lists\')")
                                ),
                            );
                        }
                    }
                });
                List_::iterate(action_1, List_::tail(lists))
            }
            tRows
        }
    }
    pub fn distinct<T: Eq + core::hash::Hash + Clone + 'static>(
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let hashSet: HashSet<T> = new_empty::<T>();
        List_::filter(
            Func1::new({
                let hashSet = hashSet.clone();
                move |x: T| add_1(hashSet.clone(), x)
            }),
            xs,
        )
    }
    pub fn distinctBy<T: Clone + 'static, Key: Eq + core::hash::Hash + Clone + 'static>(
        projection: Func1<T, Key>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let hashSet: HashSet<Key> = new_empty::<Key>();
        List_::filter(
            Func1::new({
                let hashSet = hashSet.clone();
                let projection = projection.clone();
                move |x: T| add_1(hashSet.clone(), projection(x))
            }),
            xs,
        )
    }
    pub fn except<T: Eq + core::hash::Hash + Clone + 'static>(
        itemsToExclude: LrcPtr<dyn IEnumerable_1<T>>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let hashSet: HashSet<T> = new_from_array(toArray_1(itemsToExclude));
        List_::filter(
            Func1::new({
                let hashSet = hashSet.clone();
                move |x: T| add_1(hashSet.clone(), x)
            }),
            xs,
        )
    }
    pub fn countBy<T: Clone + 'static, Key: Eq + core::hash::Hash + Clone + 'static>(
        projection: Func1<T, Key>,
        xs: List_::List<T>,
    ) -> List_::List<LrcPtr<(Key, i32)>> {
        let dict: HashMap<Key, i32> = new_empty_1::<Key, i32>();
        let keys: Array<Key> = new_empty_2::<Key>();
        {
            let action = Func1::new({
                let dict = dict.clone();
                let keys = keys.clone();
                let projection = projection.clone();
                move |x: T| {
                    let key: Key = projection(x);
                    let matchValue: LrcPtr<(bool, i32)> = {
                        let outArg: MutCell<i32> = MutCell::new(0_i32);
                        LrcPtr::new((
                            tryGetValue(dict.clone(), key.clone(), &outArg),
                            outArg.get().clone(),
                        ))
                    };
                    if matchValue.0.clone() {
                        set(dict.clone(), key.clone(), matchValue.1.clone() + 1_i32)
                    } else {
                        set(dict.clone(), key.clone(), 1_i32);
                        add(keys.clone(), key)
                    }
                }
            });
            List_::iterate(action, xs)
        }
        List_::ofArray(map_1(
            Func1::new({
                let dict = dict.clone();
                move |key_1: Key| LrcPtr::new((key_1.clone(), get(dict.clone(), key_1)))
            }),
            keys.clone(),
        ))
    }
    pub fn groupBy<T: Clone + 'static, Key: Eq + core::hash::Hash + Clone + 'static>(
        projection: Func1<T, Key>,
        xs: List_::List<T>,
    ) -> List_::List<LrcPtr<(Key, List_::List<T>)>> {
        let dict: HashMap<Key, Array<T>> = new_empty_1::<Key, Array<T>>();
        let keys: Array<Key> = new_empty_2::<Key>();
        {
            let action = Func1::new({
                let dict = dict.clone();
                let keys = keys.clone();
                let projection = projection.clone();
                move |x: T| {
                    let key: Key = projection(x.clone());
                    let matchValue: LrcPtr<(bool, Array<T>)> = {
                        let outArg: MutCell<Array<T>> = MutCell::new(new_empty_2::<T>());
                        LrcPtr::new((
                            tryGetValue(dict.clone(), key.clone(), &outArg),
                            outArg.get().clone(),
                        ))
                    };
                    if matchValue.0.clone() {
                        add(matchValue.1.clone(), x.clone())
                    } else {
                        add_2(dict.clone(), key.clone(), new_array(&[x]));
                        add(keys.clone(), key)
                    }
                }
            });
            List_::iterate(action, xs)
        }
        List_::ofArray(map_1(
            Func1::new({
                let dict = dict.clone();
                move |key_1: Key| {
                    LrcPtr::new((
                        key_1.clone(),
                        List_::ofArray({
                            let a_1: Array<T> = get(dict.clone(), key_1);
                            a_1
                        }),
                    ))
                }
            }),
            keys.clone(),
        ))
    }
    pub fn insertAt<T: Clone + 'static>(index: i32, y: T, xs: List_::List<T>) -> List_::List<T> {
        let i: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let isDone: LrcPtr<MutCell<bool>> = LrcPtr::new(MutCell::new(false));
        let res: List_::List<T> = {
            let folder = Func2::new({
                let i = i.clone();
                let index = index.clone();
                let isDone = isDone.clone();
                let y = y.clone();
                move |acc: List_::List<T>, x: T| {
                    i.set(i.get().clone() + 1_i32);
                    if i.get().clone() == index {
                        isDone.set(true);
                        List_::cons(x.clone(), List_::cons(y.clone(), acc.clone()))
                    } else {
                        List_::cons(x, acc)
                    }
                }
            });
            List_::fold(folder, List_::empty(), xs)
        };
        List_::reverse(if isDone.get().clone() {
            res.clone()
        } else {
            if i.get().clone() + 1_i32 == index {
                List_::cons(y.clone(), res)
            } else {
                panic!(
                    "{}",
                    append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
                )
            }
        })
    }
    pub fn insertManyAt<T: Clone + 'static>(
        index: i32,
        ys: LrcPtr<dyn IEnumerable_1<T>>,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let i: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let isDone: LrcPtr<MutCell<bool>> = LrcPtr::new(MutCell::new(false));
        let ys_1: List_::List<T> = List_::ofSeq(ys);
        let res: List_::List<T> = {
            let folder = Func2::new({
                let i = i.clone();
                let index = index.clone();
                let isDone = isDone.clone();
                let ys_1 = ys_1.clone();
                move |acc: List_::List<T>, x: T| {
                    i.set(i.get().clone() + 1_i32);
                    if i.get().clone() == index {
                        isDone.set(true);
                        List_::cons(x.clone(), List_::append(ys_1.clone(), acc.clone()))
                    } else {
                        List_::cons(x, acc)
                    }
                }
            });
            List_::fold(folder, List_::empty(), xs)
        };
        List_::reverse(if isDone.get().clone() {
            res.clone()
        } else {
            if i.get().clone() + 1_i32 == index {
                List_::append(ys_1.clone(), res)
            } else {
                panic!(
                    "{}",
                    append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
                )
            }
        })
    }
    pub fn removeAt<T: Clone + 'static>(index: i32, xs: List_::List<T>) -> List_::List<T> {
        let i: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let isDone: LrcPtr<MutCell<bool>> = LrcPtr::new(MutCell::new(false));
        let res: List_::List<T> = {
            let predicate = Func1::new({
                let i = i.clone();
                let index = index.clone();
                let isDone = isDone.clone();
                move |_arg: T| {
                    i.set(i.get().clone() + 1_i32);
                    if i.get().clone() == index {
                        isDone.set(true);
                        false
                    } else {
                        true
                    }
                }
            });
            List_::filter(predicate, xs)
        };
        if !isDone.get().clone() {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        res
    }
    pub fn removeManyAt<T: Clone + 'static>(
        index: i32,
        count: i32,
        xs: List_::List<T>,
    ) -> List_::List<T> {
        let i: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let status: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let res: List_::List<T> = {
            let predicate = Func1::new({
                let count = count.clone();
                let i = i.clone();
                let index = index.clone();
                let status = status.clone();
                move |_arg: T| {
                    i.set(i.get().clone() + 1_i32);
                    if i.get().clone() == index {
                        status.set(0_i32);
                        false
                    } else {
                        if i.get().clone() > index {
                            if i.get().clone() < index + count {
                                false
                            } else {
                                status.set(1_i32);
                                true
                            }
                        } else {
                            true
                        }
                    }
                }
            });
            List_::filter(predicate, xs)
        };
        let status_1: i32 = if if status.get().clone() == 0_i32 {
            i.get().clone() + 1_i32 == index + count
        } else {
            false
        } {
            1_i32
        } else {
            status.get().clone()
        };
        if status_1 < 1_i32 {
            panic!(
                "{}",
                append_1(
                    SR::indexOutOfBounds(),
                    append_1(
                        append_1(
                            string(" (Parameter \'"),
                            (if status_1 < 0_i32 {
                                string("index")
                            } else {
                                string("count")
                            })
                        ),
                        string("\')")
                    )
                ),
            );
        }
        res
    }
    pub fn updateAt<T: Clone + 'static>(index: i32, y: T, xs: List_::List<T>) -> List_::List<T> {
        let isDone: LrcPtr<MutCell<bool>> = LrcPtr::new(MutCell::new(false));
        let res: List_::List<T> = {
            let mapping = Func2::new({
                let index = index.clone();
                let isDone = isDone.clone();
                let y = y.clone();
                move |i: i32, x: T| {
                    if i == index {
                        isDone.set(true);
                        y.clone()
                    } else {
                        x
                    }
                }
            });
            List_::mapIndexed(mapping, xs)
        };
        if !isDone.get().clone() {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        res
    }
}
