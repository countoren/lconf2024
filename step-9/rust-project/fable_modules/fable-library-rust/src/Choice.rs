pub mod Choice_ {
    use super::*;
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
    pub enum Choice_2<T1: Clone + 'static, T2: Clone + 'static> {
        Choice1Of2(T1),
        Choice2Of2(T2),
    }
    impl<T1: Clone + 'static, T2: Clone + 'static> core::fmt::Display for Choice_::Choice_2<T1, T2> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
    pub enum Choice_3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static> {
        Choice1Of3(T1),
        Choice2Of3(T2),
        Choice3Of3(T3),
    }
    impl<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static> core::fmt::Display
        for Choice_::Choice_3<T1, T2, T3>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
    pub enum Choice_4<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
    > {
        Choice1Of4(T1),
        Choice2Of4(T2),
        Choice3Of4(T3),
        Choice4Of4(T4),
    }
    impl<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, T4: Clone + 'static>
        core::fmt::Display for Choice_::Choice_4<T1, T2, T3, T4>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
    pub enum Choice_5<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
    > {
        Choice1Of5(T1),
        Choice2Of5(T2),
        Choice3Of5(T3),
        Choice4Of5(T4),
        Choice5Of5(T5),
    }
    impl<
            T1: Clone + 'static,
            T2: Clone + 'static,
            T3: Clone + 'static,
            T4: Clone + 'static,
            T5: Clone + 'static,
        > core::fmt::Display for Choice_::Choice_5<T1, T2, T3, T4, T5>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
    pub enum Choice_6<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
        T6: Clone + 'static,
    > {
        Choice1Of6(T1),
        Choice2Of6(T2),
        Choice3Of6(T3),
        Choice4Of6(T4),
        Choice5Of6(T5),
        Choice6Of6(T6),
    }
    impl<
            T1: Clone + 'static,
            T2: Clone + 'static,
            T3: Clone + 'static,
            T4: Clone + 'static,
            T5: Clone + 'static,
            T6: Clone + 'static,
        > core::fmt::Display for Choice_::Choice_6<T1, T2, T3, T4, T5, T6>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
    pub enum Choice_7<
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
        T6: Clone + 'static,
        T7: Clone + 'static,
    > {
        Choice1Of7(T1),
        Choice2Of7(T2),
        Choice3Of7(T3),
        Choice4Of7(T4),
        Choice5Of7(T5),
        Choice6Of7(T6),
        Choice7Of7(T7),
    }
    impl<
            T1: Clone + 'static,
            T2: Clone + 'static,
            T3: Clone + 'static,
            T4: Clone + 'static,
            T5: Clone + 'static,
            T6: Clone + 'static,
            T7: Clone + 'static,
        > core::fmt::Display for Choice_::Choice_7<T1, T2, T3, T4, T5, T6, T7>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", core::any::type_name::<Self>())
        }
    }
}
