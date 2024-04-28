pub mod Result_ {
    use super::*;
    use crate::List_::empty;
    use crate::List_::singleton;
    use crate::List_::List;
    use crate::NativeArray_::new_array;
    use crate::NativeArray_::new_empty;
    use crate::NativeArray_::Array;
    use crate::Native_::Func1;
    use crate::Native_::Func2;
    pub fn map<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        mapping: Func1<a, b>,
        result: Result<a, c>,
    ) -> Result<b, c> {
        match &result {
            Err(result_1_0) => Err::<b, c>(result_1_0.clone()),
            Ok(result_0_0) => Ok::<b, c>(mapping(result_0_0.clone())),
        }
    }
    pub fn mapError<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        mapping: Func1<a, b>,
        result: Result<c, a>,
    ) -> Result<c, b> {
        match &result {
            Err(result_1_0) => Err::<c, b>(mapping(result_1_0.clone())),
            Ok(result_0_0) => Ok::<c, b>(result_0_0.clone()),
        }
    }
    pub fn bind<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        binder: Func1<a, Result<b, c>>,
        result: Result<a, c>,
    ) -> Result<b, c> {
        match &result {
            Err(result_1_0) => Err::<b, c>(result_1_0.clone()),
            Ok(result_0_0) => binder(result_0_0.clone()),
        }
    }
    pub fn isOk<a: Clone + 'static, b: Clone + 'static>(result: Result<a, b>) -> bool {
        match &result {
            Ok(result_0_0) => true,
            _ => false,
        }
    }
    pub fn isError<a: Clone + 'static, b: Clone + 'static>(result: Result<a, b>) -> bool {
        match &result {
            Ok(result_0_0) => false,
            _ => true,
        }
    }
    pub fn contains<a: Eq + core::hash::Hash + Clone + 'static, b: Clone + 'static>(
        value: a,
        result: Result<a, b>,
    ) -> bool {
        match &result {
            Ok(result_0_0) => result_0_0.clone() == value,
            _ => false,
        }
    }
    pub fn count<a: Clone + 'static, b: Clone + 'static>(result: Result<a, b>) -> i32 {
        match &result {
            Ok(result_0_0) => 1_i32,
            _ => 0_i32,
        }
    }
    pub fn defaultValue<a: Clone + 'static, b: Clone + 'static>(
        defaultValue_1: a,
        result: Result<a, b>,
    ) -> a {
        match &result {
            Ok(result_0_0) => result_0_0.clone(),
            _ => defaultValue_1,
        }
    }
    pub fn defaultWith<a: Clone + 'static, b: Clone + 'static>(
        defThunk: Func1<a, b>,
        result: Result<b, a>,
    ) -> b {
        match &result {
            Ok(result_0_0) => result_0_0.clone(),
            Err(result_1_0) => defThunk(result_1_0.clone()),
        }
    }
    pub fn exists<a: Clone + 'static, b: Clone + 'static>(
        predicate: Func1<a, bool>,
        result: Result<a, b>,
    ) -> bool {
        match &result {
            Ok(result_0_0) => predicate(result_0_0.clone()),
            _ => false,
        }
    }
    pub fn fold<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        folder: Func2<a, b, a>,
        state: a,
        result: Result<b, c>,
    ) -> a {
        match &result {
            Ok(result_0_0) => folder(state.clone(), result_0_0.clone()),
            _ => state.clone(),
        }
    }
    pub fn foldBack<a: Clone + 'static, b: Clone + 'static, c: Clone + 'static>(
        folder: Func2<a, b, b>,
        result: Result<a, c>,
        state: b,
    ) -> b {
        match &result {
            Ok(result_0_0) => folder(result_0_0.clone(), state.clone()),
            _ => state.clone(),
        }
    }
    pub fn forAll<a: Clone + 'static, b: Clone + 'static>(
        predicate: Func1<a, bool>,
        result: Result<a, b>,
    ) -> bool {
        match &result {
            Ok(result_0_0) => predicate(result_0_0.clone()),
            _ => true,
        }
    }
    pub fn iterate<a: Clone + 'static, b: Clone + 'static>(
        action: Func1<a, ()>,
        result: Result<a, b>,
    ) {
        match &result {
            Ok(result_0_0) => action(result_0_0.clone()),
            _ => (),
        };
    }
    pub fn toArray<a: Clone + 'static, b: Clone + 'static>(result: Result<a, b>) -> Array<a> {
        match &result {
            Ok(result_0_0) => new_array(&[result_0_0.clone()]),
            _ => new_empty::<a>(),
        }
    }
    pub fn toList<a: Clone + 'static, b: Clone + 'static>(result: Result<a, b>) -> List<a> {
        match &result {
            Ok(result_0_0) => singleton(result_0_0.clone()),
            _ => empty::<a>(),
        }
    }
    pub fn toOption<a: Clone + 'static, b: Clone + 'static>(result: Result<a, b>) -> Option<a> {
        match &result {
            Ok(result_0_0) => Some(result_0_0.clone()),
            _ => None::<a>,
        }
    }
    pub fn toValueOption<a: Clone + 'static, b: Clone + 'static>(
        result: Result<a, b>,
    ) -> Option<a> {
        match &result {
            Ok(result_0_0) => Some(result_0_0.clone()),
            _ => None::<a>,
        }
    }
}
