pub mod Option_ {
    use super::*;
    use crate::Native_::Func0;
    use crate::Native_::Func1;
    use crate::Native_::Func2;
    use crate::Native_::Func3;
    use crate::String_::string;
    pub fn bind<T: Clone + 'static, U: Clone + 'static>(
        binder: Func1<T, Option<U>>,
        opt: Option<T>,
    ) -> Option<U> {
        match &opt {
            None => None::<U>,
            Some(opt_0_0) => binder(opt_0_0.clone()),
        }
    }
    pub fn contains<T: Eq + core::hash::Hash + Clone + 'static>(value: T, opt: Option<T>) -> bool {
        match &opt {
            None => false,
            Some(opt_0_0) => opt_0_0.clone() == value,
        }
    }
    pub fn count<T: Clone + 'static>(opt: Option<T>) -> i32 {
        match &opt {
            None => 0_i32,
            _ => 1_i32,
        }
    }
    pub fn defaultArg<T: Clone + 'static>(opt: Option<T>, defaultValue_1: T) -> T {
        match &opt {
            None => defaultValue_1,
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn defaultValue<T: Clone + 'static>(defaultValue_1: T, opt: Option<T>) -> T {
        match &opt {
            None => defaultValue_1,
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn defaultWith<T: Clone + 'static>(defThunk: Func0<T>, opt: Option<T>) -> T {
        match &opt {
            None => defThunk(),
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn exists<T: Clone + 'static>(predicate: Func1<T, bool>, opt: Option<T>) -> bool {
        match &opt {
            None => false,
            Some(opt_0_0) => predicate(opt_0_0.clone()),
        }
    }
    pub fn filter<T: Clone + 'static>(predicate: Func1<T, bool>, opt: Option<T>) -> Option<T> {
        match &opt {
            None => None::<T>,
            Some(opt_0_0) => {
                if predicate(opt_0_0.clone()) {
                    opt.clone()
                } else {
                    None::<T>
                }
            }
        }
    }
    pub fn flatten<T: Clone + 'static>(opt: Option<Option<T>>) -> Option<T> {
        match &opt {
            None => None::<T>,
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn fold<S: Clone + 'static, T: Clone + 'static>(
        folder: Func2<S, T, S>,
        state: S,
        opt: Option<T>,
    ) -> S {
        match &opt {
            None => state.clone(),
            Some(opt_0_0) => folder(state.clone(), opt_0_0.clone()),
        }
    }
    pub fn foldBack<T: Clone + 'static, S: Clone + 'static>(
        folder: Func2<T, S, S>,
        opt: Option<T>,
        state: S,
    ) -> S {
        match &opt {
            None => state.clone(),
            Some(opt_0_0) => folder(opt_0_0.clone(), state.clone()),
        }
    }
    pub fn forAll<T: Clone + 'static>(predicate: Func1<T, bool>, opt: Option<T>) -> bool {
        match &opt {
            None => true,
            Some(opt_0_0) => predicate(opt_0_0.clone()),
        }
    }
    pub fn getValue<T: Clone + 'static>(opt: Option<T>) -> T {
        match &opt {
            None => panic!("{}", string("Option has no value"),),
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn iterate<T: Clone + 'static>(action: Func1<T, ()>, opt: Option<T>) {
        match &opt {
            None => (),
            Some(opt_0_0) => action(opt_0_0.clone()),
        };
    }
    pub fn map<T: Clone + 'static, U: Clone + 'static>(
        mapping: Func1<T, U>,
        opt: Option<T>,
    ) -> Option<U> {
        match &opt {
            None => None::<U>,
            Some(opt_0_0) => Some(mapping(opt_0_0.clone())),
        }
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone + 'static>(
        mapping: Func2<T1, T2, U>,
        opt1: Option<T1>,
        opt2: Option<T2>,
    ) -> Option<U> {
        match &opt1 {
            Some(opt1_0_0) => {
                let x: T1 = opt1_0_0.clone();
                match &opt2 {
                    Some(opt2_0_0) => Some(mapping(x, opt2_0_0.clone())),
                    _ => None::<U>,
                }
            }
            _ => None::<U>,
        }
    }
    pub fn map3<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        U: Clone + 'static,
    >(
        mapping: Func3<T1, T2, T3, U>,
        opt1: Option<T1>,
        opt2: Option<T2>,
        opt3: Option<T3>,
    ) -> Option<U> {
        match &opt1 {
            Some(opt1_0_0) => {
                let x: T1 = opt1_0_0.clone();
                match &opt2 {
                    Some(opt2_0_0) => {
                        let y: T2 = opt2_0_0.clone();
                        match &opt3 {
                            Some(opt3_0_0) => Some(mapping(x, y, opt3_0_0.clone())),
                            _ => None::<U>,
                        }
                    }
                    _ => None::<U>,
                }
            }
            _ => None::<U>,
        }
    }
    pub fn orElse<T: Clone + 'static>(ifNone: Option<T>, opt: Option<T>) -> Option<T> {
        match &opt {
            None => ifNone,
            _ => opt.clone(),
        }
    }
    pub fn orElseWith<T: Clone + 'static>(
        ifNoneThunk: Func0<Option<T>>,
        opt: Option<T>,
    ) -> Option<T> {
        match &opt {
            None => ifNoneThunk(),
            _ => opt.clone(),
        }
    }
}
