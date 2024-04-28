pub mod Array_ {
    use super::*;
    use crate::Global_::SR;
    use crate::HashMap_::add as add_2;
    use crate::HashMap_::get;
    use crate::HashMap_::new_empty as new_empty_2;
    use crate::HashMap_::set;
    use crate::HashMap_::tryGetValue;
    use crate::HashMap_::HashMap;
    use crate::HashSet_::add as add_1;
    use crate::HashSet_::new_empty as new_empty_1;
    use crate::HashSet_::new_from_array;
    use crate::HashSet_::HashSet;
    use crate::Interfaces_::System::Collections::Generic::IEnumerable_1;
    use crate::NativeArray_::add;
    use crate::NativeArray_::count as count_2;
    use crate::NativeArray_::new_array;
    use crate::NativeArray_::new_copy;
    use crate::NativeArray_::new_empty;
    use crate::NativeArray_::new_init;
    use crate::NativeArray_::new_with_capacity;
    use crate::NativeArray_::Array;
    use crate::Native_::compare;
    use crate::Native_::defaultOf;
    use crate::Native_::getZero;
    use crate::Native_::makeCompare;
    use crate::Native_::Func1;
    use crate::Native_::Func2;
    use crate::Native_::Func3;
    use crate::Native_::LrcPtr;
    use crate::Native_::MutCell;
    use crate::Option_::defaultArg;
    use crate::Option_::getValue;
    use crate::Seq_::toArray;
    use crate::String_::append as append_1;
    use crate::String_::string;
    pub fn tryItem<T: Clone + 'static>(index: i32, source: Array<T>) -> Option<T> {
        if if index < 0_i32 {
            true
        } else {
            index >= count_2(source.clone())
        } {
            None::<T>
        } else {
            Some(source[index].clone())
        }
    }
    pub fn reverse<T: Clone + 'static>(source: Array<T>) -> Array<T> {
        let res: Array<T> = new_copy(source);
        res.get_mut().reverse();
        res
    }
    pub fn fill<T: Clone + 'static>(target: Array<T>, targetIndex: i32, count: i32, value: T) {
        if if targetIndex < 0_i32 {
            true
        } else {
            targetIndex + count > count_2(target.clone())
        } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        {
            let len: i32 = count_2(target.clone());
            for i in targetIndex..=targetIndex + count - 1_i32 {
                target.get_mut()[i as usize] = value.clone();
            }
        }
    }
    pub fn getSubArray<T: Clone + 'static>(
        source: Array<T>,
        startIndex: i32,
        count: i32,
    ) -> Array<T> {
        if if startIndex < 0_i32 {
            true
        } else {
            startIndex + count > count_2(source.clone())
        } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        {
            let res: Array<T> = new_with_capacity::<T>(count);
            for i in 0_i32..=count - 1_i32 {
                add(res.clone(), source[startIndex + i].clone());
            }
            res.clone()
        }
    }
    pub fn exactlyOne<T: Clone + 'static>(source: Array<T>) -> T {
        if count_2(source.clone()) == 1_i32 {
            source[0_i32].clone()
        } else {
            if source.is_empty() {
                panic!(
                    "{}",
                    append_1(SR::inputSequenceEmpty(), string(" (Parameter \'array\')")),
                )
            } else {
                panic!(
                    "{}",
                    append_1(SR::inputSequenceTooLong(), string(" (Parameter \'array\')")),
                )
            }
        }
    }
    pub fn tryExactlyOne<T: Clone + 'static>(source: Array<T>) -> Option<T> {
        if count_2(source.clone()) == 1_i32 {
            Some(source[0_i32].clone())
        } else {
            None::<T>
        }
    }
    pub fn head<T: Clone + 'static>(source: Array<T>) -> T {
        if source.is_empty() {
            panic!(
                "{}",
                append_1(SR::arrayWasEmpty(), string(" (Parameter \'array\')")),
            )
        } else {
            source[0_i32].clone()
        }
    }
    pub fn tryHead<T: Clone + 'static>(source: Array<T>) -> Option<T> {
        if source.is_empty() {
            None::<T>
        } else {
            Some(source[0_i32].clone())
        }
    }
    pub fn last<T: Clone + 'static>(source: Array<T>) -> T {
        let len: i32 = count_2(source.clone());
        if source.is_empty() {
            panic!(
                "{}",
                append_1(SR::arrayWasEmpty(), string(" (Parameter \'array\')")),
            )
        } else {
            source[len - 1_i32].clone()
        }
    }
    pub fn tryLast<T: Clone + 'static>(source: Array<T>) -> Option<T> {
        let len: i32 = count_2(source.clone());
        if source.is_empty() {
            None::<T>
        } else {
            Some(source[len - 1_i32].clone())
        }
    }
    pub fn tail<T: Clone + 'static>(source: Array<T>) -> Array<T> {
        if source.is_empty() {
            panic!(
                "{}",
                append_1(SR::notEnoughElements(), string(" (Parameter \'array\')")),
            );
        }
        Array_::getSubArray(source.clone(), 1_i32, count_2(source) - 1_i32)
    }
    pub fn append<T: Clone + 'static>(source1: Array<T>, source2: Array<T>) -> Array<T> {
        let len1: i32 = count_2(source1.clone());
        let len2: i32 = count_2(source2.clone());
        let res: Array<T> = new_with_capacity::<T>(len1 + len2);
        for i in 0_i32..=len1 - 1_i32 {
            add(res.clone(), source1[i].clone());
        }
        for i_1 in 0_i32..=len2 - 1_i32 {
            add(res.clone(), source2[i_1].clone());
        }
        res.clone()
    }
    pub fn choose<T: Clone + 'static, U: Clone + 'static>(
        chooser: Func1<T, Option<U>>,
        source: Array<T>,
    ) -> Array<U> {
        let res: Array<U> = new_empty::<U>();
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            let matchValue: Option<U> = chooser(source[i].clone());
            match &matchValue {
                None => (),
                Some(matchValue_0_0) => add(res.clone(), matchValue_0_0.clone()),
            }
        }
        res.clone()
    }
    pub fn compareWith<T: Clone + 'static>(
        comparer: Func2<T, T, i32>,
        source1: Array<T>,
        source2: Array<T>,
    ) -> i32 {
        let len1: i32 = count_2(source1.clone());
        let len2: i32 = count_2(source2.clone());
        let len: i32 = if len1 < len2 { len1 } else { len2 };
        let i: MutCell<i32> = MutCell::new(0_i32);
        let res: MutCell<i32> = MutCell::new(0_i32);
        while if res.get().clone() == 0_i32 {
            i.get().clone() < len
        } else {
            false
        } {
            res.set(comparer(
                source1[i.get().clone()].clone(),
                source2[i.get().clone()].clone(),
            ));
            i.set(i.get().clone() + 1_i32)
        }
        if res.get().clone() != 0_i32 {
            res.get().clone()
        } else {
            if len1 > len2 {
                1_i32
            } else {
                if len1 < len2 {
                    -1_i32
                } else {
                    0_i32
                }
            }
        }
    }
    pub fn compareTo<T: PartialOrd + Clone + 'static>(source1: Array<T>, source2: Array<T>) -> i32 {
        let len1: i32 = count_2(source1.clone());
        let len2: i32 = count_2(source2.clone());
        if len1 > len2 {
            1_i32
        } else {
            if len1 < len2 {
                -1_i32
            } else {
                let i: MutCell<i32> = MutCell::new(0_i32);
                let res: MutCell<i32> = MutCell::new(0_i32);
                while if res.get().clone() == 0_i32 {
                    i.get().clone() < len1
                } else {
                    false
                } {
                    res.set(compare(
                        source1[i.get().clone()].clone(),
                        source2[i.get().clone()].clone(),
                    ));
                    i.set(i.get().clone() + 1_i32)
                }
                res.get().clone()
            }
        }
    }
    pub fn equals<T: Eq + core::hash::Hash + Clone + 'static>(
        source1: Array<T>,
        source2: Array<T>,
    ) -> bool {
        let len1: i32 = count_2(source1.clone());
        if len1 == count_2(source2.clone()) {
            let i: MutCell<i32> = MutCell::new(0_i32);
            let res: MutCell<bool> = MutCell::new(true);
            while if res.get().clone() {
                i.get().clone() < len1
            } else {
                false
            } {
                res.set(source1[i.get().clone()].clone() == source2[i.get().clone()].clone());
                i.set(i.get().clone() + 1_i32)
            }
            res.get().clone()
        } else {
            false
        }
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<i32, T, U>,
        source: Array<T>,
    ) -> Array<U> {
        let len: i32 = count_2(source.clone());
        let res: Array<U> = new_with_capacity::<U>(len);
        for i in 0_i32..=len - 1_i32 {
            add(res.clone(), mapping(i, source[i].clone()));
        }
        res.clone()
    }
    pub fn map<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func1<T, U>,
        source: Array<T>,
    ) -> Array<U> {
        let len: i32 = count_2(source.clone());
        let res: Array<U> = new_with_capacity::<U>(len);
        for i in 0_i32..=len - 1_i32 {
            add(res.clone(), mapping(source[i].clone()));
        }
        res.clone()
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone + 'static>(
        mapping: Func3<i32, T1, T2, U>,
        source1: Array<T1>,
        source2: Array<T2>,
    ) -> Array<U> {
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        {
            let len: i32 = count_2(source1.clone());
            let res: Array<U> = new_with_capacity::<U>(len);
            for i in 0_i32..=len - 1_i32 {
                add(
                    res.clone(),
                    mapping(i, source1[i].clone(), source2[i].clone()),
                );
            }
            res.clone()
        }
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<T1, T2, U>,
        source1: Array<T1>,
        source2: Array<T2>,
    ) -> Array<U> {
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        {
            let len: i32 = count_2(source1.clone());
            let res: Array<U> = new_with_capacity::<U>(len);
            for i in 0_i32..=len - 1_i32 {
                add(res.clone(), mapping(source1[i].clone(), source2[i].clone()));
            }
            res.clone()
        }
    }
    pub fn map3<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        U: Clone + 'static,
    >(
        mapping: Func3<T1, T2, T3, U>,
        source1: Array<T1>,
        source2: Array<T2>,
        source3: Array<T3>,
    ) -> Array<U> {
        if if count_2(source1.clone()) != count_2(source2.clone()) {
            true
        } else {
            count_2(source2.clone()) != count_2(source3.clone())
        } {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        {
            let len: i32 = count_2(source1.clone());
            let res: Array<U> = new_with_capacity::<U>(len);
            for i in 0_i32..=len - 1_i32 {
                add(
                    res.clone(),
                    mapping(source1[i].clone(), source2[i].clone(), source3[i].clone()),
                );
            }
            res.clone()
        }
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<State, T, LrcPtr<(U, State)>>,
        state: State,
        source: Array<T>,
    ) -> LrcPtr<(Array<U>, State)> {
        let acc: MutCell<State> = MutCell::new(state);
        let len: i32 = count_2(source.clone());
        let res: Array<U> = new_with_capacity::<U>(len);
        for i in 0_i32..=len - 1_i32 {
            let m: LrcPtr<(U, State)> = mapping(acc.get().clone(), source[i].clone());
            add(res.clone(), m.0.clone());
            acc.set(m.1.clone())
        }
        LrcPtr::new((res.clone(), acc.get().clone()))
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<T, State, LrcPtr<(U, State)>>,
        source: Array<T>,
        state: State,
    ) -> LrcPtr<(Array<U>, State)> {
        let acc: MutCell<State> = MutCell::new(state);
        let len: i32 = count_2(source.clone());
        let res: Array<U> = new_with_capacity::<U>(len);
        for i in (0_i32..=len - 1_i32).rev() {
            let m: LrcPtr<(U, State)> = mapping(source[i].clone(), acc.get().clone());
            add(res.clone(), m.0.clone());
            acc.set(m.1.clone())
        }
        res.get_mut().reverse();
        LrcPtr::new((res.clone(), acc.get().clone()))
    }
    pub fn indexed<T: Clone + 'static>(source: Array<T>) -> Array<LrcPtr<(i32, T)>> {
        let len: i32 = count_2(source.clone());
        let res: Array<LrcPtr<(i32, T)>> = new_with_capacity::<LrcPtr<(i32, T)>>(len);
        for i in 0_i32..=len - 1_i32 {
            add(res.clone(), LrcPtr::new((i, source[i].clone())));
        }
        res.clone()
    }
    pub fn concat<T: Clone + 'static>(sources: Array<Array<T>>) -> Array<T> {
        let len: MutCell<i32> = MutCell::new(0_i32);
        for idx in 0_i32..=count_2(sources.clone()) - 1_i32 {
            let arr: Array<T> = sources[idx].clone();
            len.set(len.get().clone() + count_2(arr))
        }
        {
            let res: Array<T> = new_with_capacity::<T>(len.get().clone());
            for idx_1 in 0_i32..=count_2(sources.clone()) - 1_i32 {
                let arr_1: Array<T> = sources[idx_1].clone();
                for idx_2 in 0_i32..=count_2(arr_1.clone()) - 1_i32 {
                    add(res.clone(), arr_1[idx_2].clone());
                }
            }
            res.clone()
        }
    }
    pub fn collect<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func1<T, Array<U>>,
        source: Array<T>,
    ) -> Array<U> {
        Array_::concat(Array_::map(mapping, source))
    }
    pub fn exists<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> bool {
        let i: MutCell<i32> = MutCell::new(0_i32);
        let res: MutCell<bool> = MutCell::new(false);
        while if i.get().clone() < count_2(source.clone()) {
            !res.get().clone()
        } else {
            false
        } {
            res.set(predicate(source[i.get().clone()].clone()));
            i.set(i.get().clone() + 1_i32)
        }
        res.get().clone()
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone + 'static>(
        predicate: Func2<T1, T2, bool>,
        source1: Array<T1>,
        source2: Array<T2>,
    ) -> bool {
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        {
            let i: MutCell<i32> = MutCell::new(0_i32);
            let res: MutCell<bool> = MutCell::new(false);
            while if i.get().clone() < count_2(source1.clone()) {
                !res.get().clone()
            } else {
                false
            } {
                res.set(predicate(
                    source1[i.get().clone()].clone(),
                    source2[i.get().clone()].clone(),
                ));
                i.set(i.get().clone() + 1_i32)
            }
            res.get().clone()
        }
    }
    pub fn contains<T: Eq + core::hash::Hash + Clone + 'static>(
        value: T,
        source: Array<T>,
    ) -> bool {
        Array_::exists(
            Func1::new({
                let value = value.clone();
                move |x: T| x == value.clone()
            }),
            source,
        )
    }
    pub fn filter<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> Array<T> {
        let res: Array<T> = new_empty::<T>();
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            if predicate(source[i].clone()) {
                add(res.clone(), source[i].clone());
            };
        }
        res.clone()
    }
    pub fn initialize<T: Clone + 'static>(count: i32, initializer: Func1<i32, T>) -> Array<T> {
        if count < 0_i32 {
            panic!(
                "{}",
                append_1(
                    SR::inputMustBeNonNegative(),
                    string(" (Parameter \'count\')")
                ),
            );
        }
        {
            let res: Array<T> = new_with_capacity::<T>(count);
            for i in 0_i32..=count - 1_i32 {
                add(res.clone(), initializer(i));
            }
            res.clone()
        }
    }
    pub fn pairwise<T: Clone + 'static>(source: Array<T>) -> Array<LrcPtr<(T, T)>> {
        if count_2(source.clone()) < 2_i32 {
            let a: Array<LrcPtr<(T, T)>> = new_empty::<LrcPtr<(T, T)>>();
            a
        } else {
            let len: i32 = count_2(source.clone()) - 1_i32;
            let res: Array<LrcPtr<(T, T)>> = new_with_capacity::<LrcPtr<(T, T)>>(len);
            for i in 0_i32..=len - 1_i32 {
                add(
                    res.clone(),
                    LrcPtr::new((source[i].clone(), source[i + 1_i32].clone())),
                );
            }
            res.clone()
        }
    }
    pub fn partition<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        source: Array<T>,
    ) -> LrcPtr<(Array<T>, Array<T>)> {
        let res1: Array<T> = new_empty::<T>();
        let res2: Array<T> = new_empty::<T>();
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            if predicate(source[i].clone()) {
                add(res1.clone(), source[i].clone())
            } else {
                add(res2.clone(), source[i].clone())
            };
        }
        LrcPtr::new((res1.clone(), res2.clone()))
    }
    pub fn reduce<T: Clone + 'static>(reduction: Func2<T, T, T>, source: Array<T>) -> T {
        if source.is_empty() {
            panic!("{}", SR::arrayWasEmpty(),);
        }
        {
            let acc_1: MutCell<T> = MutCell::new(source[0_i32].clone());
            for i_1 in 0_i32..=count_2(source.clone()) - 1_i32 {
                acc_1.set({
                    let x: T = source[i_1].clone();
                    if i_1 == 0_i32 {
                        x.clone()
                    } else {
                        reduction(acc_1.get().clone(), x)
                    }
                });
            }
            acc_1.get().clone()
        }
    }
    pub fn reduceBack<T: Clone + 'static>(reduction: Func2<T, T, T>, source: Array<T>) -> T {
        if source.is_empty() {
            panic!("{}", SR::arrayWasEmpty(),);
        }
        {
            let len: i32 = count_2(source.clone());
            let acc_1: MutCell<T> = MutCell::new(source[len - 1_i32].clone());
            for i_1 in 1_i32..=len {
                acc_1.set({
                    let x: T = source[len - i_1].clone();
                    if i_1 - 1_i32 == 0_i32 {
                        x.clone()
                    } else {
                        reduction(acc_1.get().clone(), x)
                    }
                });
            }
            acc_1.get().clone()
        }
    }
    pub fn replicate<T: Clone + 'static>(count: i32, initial: T) -> Array<T> {
        Array_::initialize(
            count,
            Func1::new({
                let initial = initial.clone();
                move |_arg: i32| initial.clone()
            }),
        )
    }
    pub fn scan<State: Clone + 'static, T: Clone + 'static>(
        folder: Func2<State, T, State>,
        state: State,
        source: Array<T>,
    ) -> Array<State> {
        let len: i32 = count_2(source.clone());
        let res: Array<State> = new_init(&state, len + 1_i32);
        res.get_mut()[0_i32 as usize] = state;
        for i in 0_i32..=len - 1_i32 {
            res.get_mut()[(i + 1_i32) as usize] = folder(res[i].clone(), source[i].clone());
        }
        res.clone()
    }
    pub fn scanBack<T: Clone + 'static, State: Clone + 'static>(
        folder: Func2<T, State, State>,
        source: Array<T>,
        state: State,
    ) -> Array<State> {
        let len: i32 = count_2(source.clone());
        let res: Array<State> = new_init(&state, len + 1_i32);
        res.get_mut()[len as usize] = state;
        for i in (0_i32..=len - 1_i32).rev() {
            res.get_mut()[i as usize] = folder(source[i].clone(), res[i + 1_i32].clone());
        }
        res.clone()
    }
    pub fn skip<T: Clone + 'static>(count: i32, source: Array<T>) -> Array<T> {
        if count > count_2(source.clone()) {
            panic!(
                "{}",
                append_1(SR::outOfRange(), string(" (Parameter \'count\')")),
            );
        }
        {
            let count_1: i32 = if count < 0_i32 { 0_i32 } else { count };
            Array_::getSubArray(source.clone(), count_1, count_2(source) - count_1)
        }
    }
    pub fn skipWhile<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> Array<T> {
        let count: MutCell<i32> = MutCell::new(0_i32);
        while if count.get().clone() < count_2(source.clone()) {
            predicate(source[count.get().clone()].clone())
        } else {
            false
        } {
            count.set(count.get().clone() + 1_i32);
        }
        Array_::getSubArray(
            source.clone(),
            count.get().clone(),
            count_2(source.clone()) - count.get().clone(),
        )
    }
    pub fn take<T: Clone + 'static>(count: i32, source: Array<T>) -> Array<T> {
        if count < 0_i32 {
            panic!(
                "{}",
                append_1(
                    SR::inputMustBeNonNegative(),
                    string(" (Parameter \'count\')")
                ),
            );
        }
        if count > count_2(source.clone()) {
            panic!(
                "{}",
                append_1(SR::notEnoughElements(), string(" (Parameter \'array\')")),
            );
        }
        Array_::getSubArray(source, 0_i32, count)
    }
    pub fn takeWhile<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> Array<T> {
        let count: MutCell<i32> = MutCell::new(0_i32);
        while if count.get().clone() < count_2(source.clone()) {
            predicate(source[count.get().clone()].clone())
        } else {
            false
        } {
            count.set(count.get().clone() + 1_i32);
        }
        Array_::getSubArray(source.clone(), 0_i32, count.get().clone())
    }
    pub fn truncate<T: Clone + 'static>(count: i32, source: Array<T>) -> Array<T> {
        Array_::getSubArray(
            source.clone(),
            0_i32,
            if count < 0_i32 {
                0_i32
            } else {
                if count > count_2(source.clone()) {
                    count_2(source)
                } else {
                    count
                }
            },
        )
    }
    pub fn copyTo<T: Clone + 'static>(
        source: Array<T>,
        sourceIndex: i32,
        target: Array<T>,
        targetIndex: i32,
        count: i32,
    ) {
        let diff: i32 = targetIndex - sourceIndex;
        for i in sourceIndex..=sourceIndex + count - 1_i32 {
            target.get_mut()[(i + diff) as usize] = source[i].clone();
        }
    }
    pub fn tryFind<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> Option<T> {
        fn inner_loop<T: Clone + 'static>(
            i: i32,
            predicate_1: Func1<T, bool>,
            source_1: Array<T>,
        ) -> Option<T> {
            let i: MutCell<i32> = MutCell::new(i);
            let predicate_1 = MutCell::new(predicate_1.clone());
            let source_1: MutCell<Array<T>> = MutCell::new(source_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if i.get().clone() >= count_2(source_1.get().clone()) {
                    None::<T>
                } else {
                    if predicate_1(source_1[i.get().clone()].clone()) {
                        Some(source_1[i.get().clone()].clone())
                    } else {
                        let i_temp: i32 = i.get().clone() + 1_i32;
                        let predicate_1_temp = predicate_1.get().clone();
                        let source_1_temp: Array<T> = source_1.get().clone();
                        i.set(i_temp);
                        predicate_1.set(predicate_1_temp);
                        source_1.set(source_1_temp);
                        continue '_inner_loop;
                    }
                });
            }
        }
        inner_loop(0_i32, predicate, source)
    }
    pub fn find<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> T {
        let matchValue: Option<T> = Array_::tryFind(predicate, source);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndex<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        source: Array<T>,
    ) -> Option<i32> {
        fn inner_loop<T: Clone + 'static>(
            i: i32,
            predicate_1: Func1<T, bool>,
            source_1: Array<T>,
        ) -> Option<i32> {
            let i: MutCell<i32> = MutCell::new(i);
            let predicate_1 = MutCell::new(predicate_1.clone());
            let source_1: MutCell<Array<T>> = MutCell::new(source_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if i.get().clone() >= count_2(source_1.get().clone()) {
                    None::<i32>
                } else {
                    if predicate_1(source_1[i.get().clone()].clone()) {
                        Some(i.get().clone())
                    } else {
                        let i_temp: i32 = i.get().clone() + 1_i32;
                        let predicate_1_temp = predicate_1.get().clone();
                        let source_1_temp: Array<T> = source_1.get().clone();
                        i.set(i_temp);
                        predicate_1.set(predicate_1_temp);
                        source_1.set(source_1_temp);
                        continue '_inner_loop;
                    }
                });
            }
        }
        inner_loop(0_i32, predicate, source)
    }
    pub fn findIndex<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> i32 {
        let matchValue: Option<i32> = Array_::tryFindIndex(predicate, source);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn indexOf<T: Eq + core::hash::Hash + Clone + 'static>(source: Array<T>, item: T) -> i32 {
        let matchValue: Option<i32> = Array_::tryFindIndex(
            Func1::new({
                let item = item.clone();
                move |x: T| x == item.clone()
            }),
            source,
        );
        match &matchValue {
            None => -1_i32,
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindBack<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        source: Array<T>,
    ) -> Option<T> {
        fn inner_loop<T: Clone + 'static>(
            i: i32,
            predicate_1: Func1<T, bool>,
            source_1: Array<T>,
        ) -> Option<T> {
            let i: MutCell<i32> = MutCell::new(i);
            let predicate_1 = MutCell::new(predicate_1.clone());
            let source_1: MutCell<Array<T>> = MutCell::new(source_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if i.get().clone() < 0_i32 {
                    None::<T>
                } else {
                    if predicate_1(source_1[i.get().clone()].clone()) {
                        Some(source_1[i.get().clone()].clone())
                    } else {
                        let i_temp: i32 = i.get().clone() - 1_i32;
                        let predicate_1_temp = predicate_1.get().clone();
                        let source_1_temp: Array<T> = source_1.get().clone();
                        i.set(i_temp);
                        predicate_1.set(predicate_1_temp);
                        source_1.set(source_1_temp);
                        continue '_inner_loop;
                    }
                });
            }
        }
        inner_loop(count_2(source.clone()) - 1_i32, predicate, source)
    }
    pub fn findBack<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> T {
        let matchValue: Option<T> = Array_::tryFindBack(predicate, source);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndexBack<T: Clone + 'static>(
        predicate: Func1<T, bool>,
        source: Array<T>,
    ) -> Option<i32> {
        fn inner_loop<T: Clone + 'static>(
            i: i32,
            predicate_1: Func1<T, bool>,
            source_1: Array<T>,
        ) -> Option<i32> {
            let i: MutCell<i32> = MutCell::new(i);
            let predicate_1 = MutCell::new(predicate_1.clone());
            let source_1: MutCell<Array<T>> = MutCell::new(source_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if i.get().clone() < 0_i32 {
                    None::<i32>
                } else {
                    if predicate_1(source_1[i.get().clone()].clone()) {
                        Some(i.get().clone())
                    } else {
                        let i_temp: i32 = i.get().clone() - 1_i32;
                        let predicate_1_temp = predicate_1.get().clone();
                        let source_1_temp: Array<T> = source_1.get().clone();
                        i.set(i_temp);
                        predicate_1.set(predicate_1_temp);
                        source_1.set(source_1_temp);
                        continue '_inner_loop;
                    }
                });
            }
        }
        inner_loop(count_2(source.clone()) - 1_i32, predicate, source)
    }
    pub fn findIndexBack<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> i32 {
        let matchValue: Option<i32> = Array_::tryFindIndexBack(predicate, source);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn findLastIndex<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> i32 {
        let matchValue: Option<i32> = Array_::tryFindIndexBack(predicate, source);
        match &matchValue {
            None => -1_i32,
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryPick<T: Clone + 'static, U: Clone + 'static>(
        chooser: Func1<T, Option<U>>,
        source: Array<T>,
    ) -> Option<U> {
        fn inner_loop<T: Clone + 'static, U: Clone + 'static>(
            i: i32,
            chooser_1: Func1<T, Option<U>>,
            source_1: Array<T>,
        ) -> Option<U> {
            let i: MutCell<i32> = MutCell::new(i);
            let chooser_1 = MutCell::new(chooser_1.clone());
            let source_1: MutCell<Array<T>> = MutCell::new(source_1.clone());
            '_inner_loop: loop {
                break '_inner_loop (if i.get().clone() >= count_2(source_1.get().clone()) {
                    None::<U>
                } else {
                    let matchValue: Option<U> = chooser_1(source_1[i.get().clone()].clone());
                    if matchValue.is_none() {
                        let i_temp: i32 = i.get().clone() + 1_i32;
                        let chooser_1_temp = chooser_1.get().clone();
                        let source_1_temp: Array<T> = source_1.get().clone();
                        i.set(i_temp);
                        chooser_1.set(chooser_1_temp);
                        source_1.set(source_1_temp);
                        continue '_inner_loop;
                    } else {
                        matchValue
                    }
                });
            }
        }
        inner_loop(0_i32, chooser, source)
    }
    pub fn pick<T: Clone + 'static, U: Clone + 'static>(
        chooser: Func1<T, Option<U>>,
        source: Array<T>,
    ) -> U {
        let matchValue: Option<U> = Array_::tryPick(chooser, source);
        match &matchValue {
            None => panic!("{}", SR::keyNotFoundAlt(),),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn fold<State: Clone + 'static, T: Clone + 'static>(
        folder: Func2<State, T, State>,
        state: State,
        source: Array<T>,
    ) -> State {
        let acc: MutCell<State> = MutCell::new(state);
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            acc.set(folder(acc.get().clone(), source[i].clone()));
        }
        acc.get().clone()
    }
    pub fn foldBack<T: Clone + 'static, State: Clone + 'static>(
        folder: Func2<T, State, State>,
        source: Array<T>,
        state: State,
    ) -> State {
        let acc: MutCell<State> = MutCell::new(state);
        let len: i32 = count_2(source.clone());
        for i in 1_i32..=len {
            acc.set(folder(source[len - i].clone(), acc.get().clone()));
        }
        acc.get().clone()
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone + 'static>(
        folder: Func3<State, T1, T2, State>,
        state: State,
        source1: Array<T1>,
        source2: Array<T2>,
    ) -> State {
        let acc: MutCell<State> = MutCell::new(state);
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        for i in 0_i32..=count_2(source1.clone()) - 1_i32 {
            acc.set(folder(
                acc.get().clone(),
                source1[i].clone(),
                source2[i].clone(),
            ));
        }
        acc.get().clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone + 'static>(
        folder: Func3<T1, T2, State, State>,
        source1: Array<T1>,
        source2: Array<T2>,
        state: State,
    ) -> State {
        let acc: MutCell<State> = MutCell::new(state);
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        {
            let len: i32 = count_2(source1.clone());
            for i in 1_i32..=len {
                acc.set(folder(
                    source1[len - i].clone(),
                    source2[len - i].clone(),
                    acc.get().clone(),
                ));
            }
            acc.get().clone()
        }
    }
    pub fn forAll<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> bool {
        let i: MutCell<i32> = MutCell::new(0_i32);
        let res: MutCell<bool> = MutCell::new(true);
        while if i.get().clone() < count_2(source.clone()) {
            res.get().clone()
        } else {
            false
        } {
            res.set(predicate(source[i.get().clone()].clone()));
            i.set(i.get().clone() + 1_i32)
        }
        res.get().clone()
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone + 'static>(
        predicate: Func2<T1, T2, bool>,
        source1: Array<T1>,
        source2: Array<T2>,
    ) -> bool {
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        {
            let i: MutCell<i32> = MutCell::new(0_i32);
            let res: MutCell<bool> = MutCell::new(true);
            while if i.get().clone() < count_2(source1.clone()) {
                res.get().clone()
            } else {
                false
            } {
                res.set(predicate(
                    source1[i.get().clone()].clone(),
                    source2[i.get().clone()].clone(),
                ));
                i.set(i.get().clone() + 1_i32)
            }
            res.get().clone()
        }
    }
    pub fn iterate<T: Clone + 'static>(action: Func1<T, ()>, source: Array<T>) {
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            action(source[i].clone());
        }
    }
    pub fn iterateIndexed<T: Clone + 'static>(action: Func2<i32, T, ()>, source: Array<T>) {
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            action(i, source[i].clone());
        }
    }
    pub fn iterate2<T: Clone + 'static>(
        action: Func2<T, T, ()>,
        source1: Array<T>,
        source2: Array<T>,
    ) {
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        for i in 0_i32..=count_2(source1.clone()) - 1_i32 {
            action(source1[i].clone(), source2[i].clone());
        }
    }
    pub fn iterateIndexed2<T: Clone + 'static>(
        action: Func3<i32, T, T, ()>,
        source1: Array<T>,
        source2: Array<T>,
    ) {
        if count_2(source1.clone()) != count_2(source2.clone()) {
            panic!("{}", SR::arraysHadDifferentLengths(),);
        }
        for i in 0_i32..=count_2(source1.clone()) - 1_i32 {
            action(i, source1[i].clone(), source2[i].clone());
        }
    }
    pub fn permute<T: Clone + 'static>(indexMap: Func1<i32, i32>, source: Array<T>) -> Array<T> {
        let len: i32 = count_2(source.clone());
        let res: Array<T> = new_copy(source.clone());
        let checkFlags: Array<i32> = new_init(&0_i32, len);
        Array_::iterateIndexed(
            Func2::new({
                let checkFlags = checkFlags.clone();
                let indexMap = indexMap.clone();
                let len = len.clone();
                let res = res.clone();
                move |i: i32, x: T| {
                    let j: i32 = indexMap(i);
                    if if j < 0_i32 { true } else { j >= len } {
                        panic!("{}", SR::notAPermutation(),);
                    }
                    res.get_mut()[j as usize] = x;
                    checkFlags.get_mut()[j as usize] = 1_i32
                }
            }),
            source,
        );
        if !Array_::forAll(Func1::new(move |y: i32| 1_i32 == y), checkFlags.clone()) {
            panic!("{}", SR::notAPermutation(),);
        }
        res.clone()
    }
    pub fn getSlice<T: Clone + 'static>(
        source: Array<T>,
        lower: Option<i32>,
        upper: Option<i32>,
    ) -> Array<T> {
        let patternInput: LrcPtr<(i32, i32)> = {
            let lower_1: Option<i32> = lower;
            let upper_1: Option<i32> = upper;
            let length: i32 = count_2(source.clone());
            LrcPtr::new((
                if lower_1.is_some() {
                    if getValue(lower_1.clone()) >= 0_i32 {
                        let n_1: i32 = getValue(lower_1.clone());
                        n_1
                    } else {
                        0_i32
                    }
                } else {
                    0_i32
                },
                if upper_1.is_some() {
                    if getValue(upper_1.clone()) < 0_i32 + length {
                        let m_1: i32 = getValue(upper_1.clone());
                        m_1
                    } else {
                        0_i32 + length - 1_i32
                    }
                } else {
                    0_i32 + length - 1_i32
                },
            ))
        };
        let start: i32 = patternInput.0.clone();
        Array_::getSubArray(source, start, patternInput.1.clone() - start + 1_i32)
    }
    pub fn setSlice<T: Clone + 'static>(
        target: Array<T>,
        lower: Option<i32>,
        upper: Option<i32>,
        source: Array<T>,
    ) {
        let start: i32 = defaultArg(lower, 0_i32);
        let count: i32 = defaultArg(upper, count_2(target.clone()) - 1_i32) - start + 1_i32;
        for i in 0_i32..=count - 1_i32 {
            target.get_mut()[(start + i) as usize] = source[i].clone();
        }
    }
    pub fn sortInPlaceWith<T: Clone + 'static>(comparer: Func2<T, T, i32>, source: Array<T>) {
        source.get_mut().sort_by(makeCompare(Func2::new({
            let comparer = comparer.clone();
            move |delegateArg: T, delegateArg_1: T| comparer(delegateArg, delegateArg_1)
        })));
    }
    pub fn sortInPlace<T: PartialOrd + Clone + 'static>(source: Array<T>) {
        Array_::sortInPlaceWith(Func2::new(move |e: T, e_1: T| compare(e, e_1)), source);
    }
    pub fn sortInPlaceBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        source: Array<T>,
    ) {
        Array_::sortInPlaceWith(
            Func2::new({
                let projection = projection.clone();
                move |x: T, y: T| compare(projection(x), projection(y))
            }),
            source,
        );
    }
    pub fn sort<T: PartialOrd + Clone + 'static>(source: Array<T>) -> Array<T> {
        let res: Array<T> = new_copy(source);
        Array_::sortInPlace(res.clone());
        res
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        source: Array<T>,
    ) -> Array<T> {
        let res: Array<T> = new_copy(source);
        Array_::sortInPlaceBy(projection, res.clone());
        res
    }
    pub fn sortWith<T: Clone + 'static>(comparer: Func2<T, T, i32>, source: Array<T>) -> Array<T> {
        let res: Array<T> = new_copy(source);
        Array_::sortInPlaceWith(comparer, res.clone());
        res
    }
    pub fn sortDescending<T: PartialOrd + Clone + 'static>(source: Array<T>) -> Array<T> {
        Array_::sortWith(Func2::new(move |x: T, y: T| compare(x, y) * -1_i32), source)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        source: Array<T>,
    ) -> Array<T> {
        Array_::sortWith(
            Func2::new({
                let projection = projection.clone();
                move |x: T, y: T| compare(projection(x), projection(y)) * -1_i32
            }),
            source,
        )
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone + 'static>(
        xs: Array<T1>,
        ys: Array<T2>,
    ) -> Array<LrcPtr<(T1, T2)>> {
        let len1: i32 = count_2(xs.clone());
        let len2: i32 = count_2(ys.clone());
        let res: Array<LrcPtr<(T1, T2)>> = new_with_capacity::<LrcPtr<(T1, T2)>>(len1 * len2);
        for i in 0_i32..=len1 - 1_i32 {
            for j in 0_i32..=len2 - 1_i32 {
                add(res.clone(), LrcPtr::new((xs[i].clone(), ys[j].clone())));
            }
        }
        res.clone()
    }
    pub fn unfold<State: Clone + 'static, T: Clone + 'static>(
        generator: Func1<State, Option<LrcPtr<(T, State)>>>,
        state: State,
    ) -> Array<T> {
        fn inner_loop<State: Clone + 'static, T: Clone + 'static>(
            generator_1: Func1<State, Option<LrcPtr<(T, State)>>>,
            state_1: State,
            res: Array<T>,
        ) {
            let generator_1 = MutCell::new(generator_1.clone());
            let state_1: MutCell<State> = MutCell::new(state_1);
            let res: MutCell<Array<T>> = MutCell::new(res.clone());
            '_inner_loop: loop {
                break '_inner_loop ({
                    let matchValue: Option<LrcPtr<(T, State)>> = generator_1(state_1.get().clone());
                    match &matchValue {
                        Some(matchValue_0_0) => {
                            let s: State = (matchValue_0_0).1.clone();
                            add(res.get().clone(), (matchValue_0_0).0.clone());
                            {
                                let generator_1_temp = generator_1.get().clone();
                                let state_1_temp: State = s.clone();
                                let res_temp: Array<T> = res.get().clone();
                                generator_1.set(generator_1_temp);
                                state_1.set(state_1_temp);
                                res.set(res_temp);
                                continue '_inner_loop;
                            }
                        }
                        _ => (),
                    }
                });
            }
        }
        let res_1: Array<T> = new_empty::<T>();
        inner_loop(generator, state, res_1.clone());
        res_1
    }
    pub fn unzip<T1: Clone + 'static, T2: Clone + 'static>(
        source: Array<LrcPtr<(T1, T2)>>,
    ) -> LrcPtr<(Array<T1>, Array<T2>)> {
        let len: i32 = count_2(source.clone());
        let res1: Array<T1> = new_with_capacity::<T1>(len);
        let res2: Array<T2> = new_with_capacity::<T2>(len);
        Array_::iterateIndexed(
            Func2::new({
                let res1 = res1.clone();
                let res2 = res2.clone();
                move |i: i32, tupledArg: LrcPtr<(T1, T2)>| {
                    add(res1.clone(), tupledArg.0.clone());
                    add(res2.clone(), tupledArg.1.clone())
                }
            }),
            source,
        );
        LrcPtr::new((res1.clone(), res2.clone()))
    }
    pub fn unzip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(
        source: Array<LrcPtr<(T1, T2, T3)>>,
    ) -> LrcPtr<(Array<T1>, Array<T2>, Array<T3>)> {
        let len: i32 = count_2(source.clone());
        let res1: Array<T1> = new_with_capacity::<T1>(len);
        let res2: Array<T2> = new_with_capacity::<T2>(len);
        let res3: Array<T3> = new_with_capacity::<T3>(len);
        Array_::iterateIndexed(
            Func2::new({
                let res1 = res1.clone();
                let res2 = res2.clone();
                let res3 = res3.clone();
                move |i: i32, tupledArg: LrcPtr<(T1, T2, T3)>| {
                    add(res1.clone(), tupledArg.0.clone());
                    add(res2.clone(), tupledArg.1.clone());
                    add(res3.clone(), tupledArg.2.clone())
                }
            }),
            source,
        );
        LrcPtr::new((res1.clone(), res2.clone(), res3.clone()))
    }
    pub fn zip<T1: Clone + 'static, T2: Clone + 'static>(
        source1: Array<T1>,
        source2: Array<T2>,
    ) -> Array<LrcPtr<(T1, T2)>> {
        Array_::map2(
            Func2::new(move |x: T1, y: T2| LrcPtr::new((x, y))),
            source1,
            source2,
        )
    }
    pub fn zip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(
        source1: Array<T1>,
        source2: Array<T2>,
        source3: Array<T3>,
    ) -> Array<LrcPtr<(T1, T2, T3)>> {
        Array_::map3(
            Func3::new(move |x: T1, y: T2, z: T3| LrcPtr::new((x, y, z))),
            source1,
            source2,
            source3,
        )
    }
    pub fn chunkBySize<T: Clone + 'static>(chunkSize: i32, source: Array<T>) -> Array<Array<T>> {
        if chunkSize <= 0_i32 {
            panic!(
                "{}",
                append_1(SR::inputMustBePositive(), string(" (Parameter \'size\')")),
            );
        }
        {
            let len: i32 = count_2(source.clone());
            let chunkCount: i32 = (len - 1_i32) / chunkSize + 1_i32;
            let res: Array<Array<T>> = new_with_capacity::<Array<T>>(chunkCount);
            for i in 0_i32..=chunkCount - 1_i32 {
                let start: i32 = i * chunkSize;
                add(
                    res.clone(),
                    Array_::getSubArray(source.clone(), start, chunkSize.min(len - start)),
                )
            }
            res.clone()
        }
    }
    pub fn splitAt<T: Clone + 'static>(
        index: i32,
        source: Array<T>,
    ) -> LrcPtr<(Array<T>, Array<T>)> {
        if if index < 0_i32 {
            true
        } else {
            index > count_2(source.clone())
        } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        LrcPtr::new((
            Array_::getSubArray(source.clone(), 0_i32, index),
            Array_::getSubArray(source.clone(), index, count_2(source) - index),
        ))
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone + 'static>(source: Array<T>) -> T {
        let acc: MutCell<T> = MutCell::new(getZero());
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            acc.set(acc.get().clone() + source[i].clone());
        }
        acc.get().clone()
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default + Clone + 'static>(
        projection: Func1<T, U>,
        source: Array<T>,
    ) -> U {
        let acc: MutCell<U> = MutCell::new(getZero());
        for i in 0_i32..=count_2(source.clone()) - 1_i32 {
            acc.set(acc.get().clone() + projection(source[i].clone()));
        }
        acc.get().clone()
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        xs: Array<T>,
    ) -> T {
        Array_::reduce(
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
    pub fn max<T: PartialOrd + Clone + 'static>(xs: Array<T>) -> T {
        Array_::reduce(
            Func2::new(move |x: T, y: T| if x.clone() > y.clone() { x } else { y }),
            xs,
        )
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone + 'static>(
        projection: Func1<T, U>,
        xs: Array<T>,
    ) -> T {
        Array_::reduce(
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
    pub fn min<T: PartialOrd + Clone + 'static>(xs: Array<T>) -> T {
        Array_::reduce(
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
        source: Array<T>,
    ) -> T {
        if source.is_empty() {
            panic!(
                "{}",
                append_1(SR::arrayWasEmpty(), string(" (Parameter \'array\')")),
            );
        }
        {
            let total: MutCell<T> = MutCell::new(getZero());
            for i in 0_i32..=count_2(source.clone()) - 1_i32 {
                total.set(total.get().clone() + source[i].clone());
            }
            total.get().clone() / T::from(count_2(source.clone()))
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
        source: Array<T>,
    ) -> U {
        if source.is_empty() {
            panic!(
                "{}",
                append_1(SR::arrayWasEmpty(), string(" (Parameter \'array\')")),
            );
        }
        {
            let total: MutCell<U> = MutCell::new(getZero());
            for i in 0_i32..=count_2(source.clone()) - 1_i32 {
                total.set(total.get().clone() + projection(source[i].clone()));
            }
            total.get().clone() / U::from(count_2(source.clone()))
        }
    }
    pub fn ofOption<T: Clone + 'static>(opt: Option<T>) -> Array<T> {
        match &opt {
            None => new_init(&defaultOf(), 0_i32),
            Some(opt_0_0) => new_init(&opt_0_0, 1_i32),
        }
    }
    pub fn r#where<T: Clone + 'static>(predicate: Func1<T, bool>, source: Array<T>) -> Array<T> {
        Array_::filter(predicate, source)
    }
    pub fn windowed<T: Clone + 'static>(windowSize: i32, source: Array<T>) -> Array<Array<T>> {
        if windowSize <= 0_i32 {
            panic!(
                "{}",
                append_1(SR::inputMustBePositive(), string(" (Parameter \'size\')")),
            );
        }
        {
            let len: i32 = 0_i32.max(count_2(source.clone()) - windowSize + 1_i32);
            let res: Array<Array<T>> = new_with_capacity::<Array<T>>(len);
            for i in 0_i32..=len - 1_i32 {
                add(
                    res.clone(),
                    Array_::getSubArray(source.clone(), i, windowSize),
                );
            }
            res.clone()
        }
    }
    pub fn splitInto<T: Clone + 'static>(chunks: i32, source: Array<T>) -> Array<Array<T>> {
        if chunks <= 0_i32 {
            panic!(
                "{}",
                append_1(SR::inputMustBePositive(), string(" (Parameter \'chunks\')")),
            );
        }
        if source.is_empty() {
            let a: Array<Array<T>> = new_empty::<Array<T>>();
            a
        } else {
            let res: Array<Array<T>> = new_with_capacity::<Array<T>>(chunks);
            let chunks_1: i32 = chunks.min(count_2(source.clone()));
            let minChunkSize: i32 = count_2(source.clone()) / chunks_1;
            let chunksWithExtraItem: i32 = count_2(source.clone()) % chunks_1;
            for i in 0_i32..=chunks_1 - 1_i32 {
                let chunkSize: i32 = if i < chunksWithExtraItem {
                    minChunkSize + 1_i32
                } else {
                    minChunkSize
                };
                add(
                    res.clone(),
                    Array_::getSubArray(
                        source.clone(),
                        i * minChunkSize + chunksWithExtraItem.min(i),
                        chunkSize,
                    ),
                )
            }
            res.clone()
        }
    }
    pub fn transpose<T: Clone + 'static>(arrays: Array<Array<T>>) -> Array<Array<T>> {
        if arrays.is_empty() {
            let a: Array<Array<T>> = new_empty::<Array<T>>();
            a
        } else {
            let len: i32 = count_2(arrays.clone());
            let innerLen: i32 = count_2(arrays[0_i32].clone());
            if !Array_::forAll(
                Func1::new({
                    let innerLen = innerLen.clone();
                    move |a_1: Array<T>| count_2(a_1) == innerLen
                }),
                arrays.clone(),
            ) {
                panic!("{}", SR::arraysHadDifferentLengths(),);
            }
            {
                let res: Array<Array<T>> = new_with_capacity::<Array<T>>(innerLen);
                for i in 0_i32..=innerLen - 1_i32 {
                    let res2: Array<T> = new_with_capacity::<T>(len);
                    for j in 0_i32..=len - 1_i32 {
                        add(res2.clone(), (arrays[j].clone())[i].clone());
                    }
                    add(res.clone(), res2.clone())
                }
                res.clone()
            }
        }
    }
    pub fn distinct<T: Eq + core::hash::Hash + Clone + 'static>(xs: Array<T>) -> Array<T> {
        let hashSet: HashSet<T> = new_empty_1::<T>();
        Array_::filter(
            Func1::new({
                let hashSet = hashSet.clone();
                move |x: T| add_1(hashSet.clone(), x)
            }),
            xs,
        )
    }
    pub fn distinctBy<T: Clone + 'static, Key: Eq + core::hash::Hash + Clone + 'static>(
        projection: Func1<T, Key>,
        xs: Array<T>,
    ) -> Array<T> {
        let hashSet: HashSet<Key> = new_empty_1::<Key>();
        Array_::filter(
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
        xs: Array<T>,
    ) -> Array<T> {
        let hashSet: HashSet<T> = new_from_array(toArray(itemsToExclude));
        Array_::filter(
            Func1::new({
                let hashSet = hashSet.clone();
                move |x: T| add_1(hashSet.clone(), x)
            }),
            xs,
        )
    }
    pub fn countBy<T: Clone + 'static, Key: Eq + core::hash::Hash + Clone + 'static>(
        projection: Func1<T, Key>,
        xs: Array<T>,
    ) -> Array<LrcPtr<(Key, i32)>> {
        let dict: HashMap<Key, i32> = new_empty_2::<Key, i32>();
        let keys: Array<Key> = new_empty::<Key>();
        for idx in 0_i32..=count_2(xs.clone()) - 1_i32 {
            let key: Key = projection(xs[idx].clone());
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
        Array_::map(
            Func1::new({
                let dict = dict.clone();
                move |key_1: Key| LrcPtr::new((key_1.clone(), get(dict.clone(), key_1)))
            }),
            keys.clone(),
        )
    }
    pub fn groupBy<T: Clone + 'static, Key: Eq + core::hash::Hash + Clone + 'static>(
        projection: Func1<T, Key>,
        xs: Array<T>,
    ) -> Array<LrcPtr<(Key, Array<T>)>> {
        let dict: HashMap<Key, Array<T>> = new_empty_2::<Key, Array<T>>();
        let keys: Array<Key> = new_empty::<Key>();
        for idx in 0_i32..=count_2(xs.clone()) - 1_i32 {
            let x: T = xs[idx].clone();
            let key: Key = projection(x.clone());
            let matchValue: LrcPtr<(bool, Array<T>)> = {
                let outArg: MutCell<Array<T>> = MutCell::new(new_empty::<T>());
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
        Array_::map(
            Func1::new({
                let dict = dict.clone();
                move |key_1: Key| {
                    LrcPtr::new((key_1.clone(), {
                        let a_1: Array<T> = get(dict.clone(), key_1);
                        a_1
                    }))
                }
            }),
            keys.clone(),
        )
    }
    pub fn insertAt<T: Clone + 'static>(index: i32, y: T, xs: Array<T>) -> Array<T> {
        let len: i32 = count_2(xs.clone());
        if if index < 0_i32 { true } else { index > len } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        {
            let res: Array<T> = new_with_capacity::<T>(len + 1_i32);
            for i in 0_i32..=index - 1_i32 {
                add(res.clone(), xs[i].clone());
            }
            add(res.clone(), y);
            for i_1 in index..=len - 1_i32 {
                add(res.clone(), xs[i_1].clone());
            }
            res.clone()
        }
    }
    pub fn insertManyAt<T: Clone + 'static>(
        index: i32,
        ys: LrcPtr<dyn IEnumerable_1<T>>,
        xs: Array<T>,
    ) -> Array<T> {
        let len: i32 = count_2(xs.clone());
        if if index < 0_i32 { true } else { index > len } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        {
            let ys_1: Array<T> = toArray(ys);
            let len2: i32 = count_2(ys_1.clone());
            let res: Array<T> = new_with_capacity::<T>(len + len2);
            for i in 0_i32..=index - 1_i32 {
                add(res.clone(), xs[i].clone());
            }
            for i_1 in 0_i32..=len2 - 1_i32 {
                add(res.clone(), ys_1[i_1].clone());
            }
            for i_2 in index..=len - 1_i32 {
                add(res.clone(), xs[i_2].clone());
            }
            res.clone()
        }
    }
    pub fn removeAt<T: Clone + 'static>(index: i32, xs: Array<T>) -> Array<T> {
        if if index < 0_i32 {
            true
        } else {
            index >= count_2(xs.clone())
        } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        {
            let i: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
            let predicate = Func1::new({
                let i = i.clone();
                let index = index.clone();
                move |_arg: T| {
                    i.set(i.get().clone() + 1_i32);
                    i.get().clone() != index
                }
            });
            Array_::filter(predicate, xs)
        }
    }
    pub fn removeManyAt<T: Clone + 'static>(index: i32, count: i32, xs: Array<T>) -> Array<T> {
        let i: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let status: LrcPtr<MutCell<i32>> = LrcPtr::new(MutCell::new(-1_i32));
        let res: Array<T> = {
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
            Array_::filter(predicate, xs)
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
    pub fn updateAt<T: Clone + 'static>(index: i32, y: T, xs: Array<T>) -> Array<T> {
        let len: i32 = count_2(xs.clone());
        if if index < 0_i32 { true } else { index >= len } {
            panic!(
                "{}",
                append_1(SR::indexOutOfBounds(), string(" (Parameter \'index\')")),
            );
        }
        {
            let res: Array<T> = new_with_capacity::<T>(len);
            for i in 0_i32..=len - 1_i32 {
                add(
                    res.clone(),
                    if i == index { y.clone() } else { xs[i].clone() },
                );
            }
            res.clone()
        }
    }
}
