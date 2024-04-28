pub mod Global_ {
    use super::*;
    pub mod SR {
        use super::*;
        use crate::Native_::OnceInit;
        use crate::String_::string;
        pub fn arrayWasEmpty() -> string {
            static arrayWasEmpty: OnceInit<string> = OnceInit::new();
            arrayWasEmpty
                .get_or_init(|| string("The input array was empty."))
                .clone()
        }
        pub fn arrayIndexOutOfBounds() -> string {
            static arrayIndexOutOfBounds: OnceInit<string> = OnceInit::new();
            arrayIndexOutOfBounds
                .get_or_init(|| string("The index was outside the range of elements in the array."))
                .clone()
        }
        pub fn arraysHadDifferentLengths() -> string {
            static arraysHadDifferentLengths: OnceInit<string> = OnceInit::new();
            arraysHadDifferentLengths
                .get_or_init(|| string("The arrays have different lengths."))
                .clone()
        }
        pub fn enumerationAlreadyFinished() -> string {
            static enumerationAlreadyFinished: OnceInit<string> = OnceInit::new();
            enumerationAlreadyFinished
                .get_or_init(|| string("Enumeration already finished."))
                .clone()
        }
        pub fn enumerationNotStarted() -> string {
            static enumerationNotStarted: OnceInit<string> = OnceInit::new();
            enumerationNotStarted
                .get_or_init(|| string("Enumeration has not started. Call MoveNext."))
                .clone()
        }
        pub fn indexOutOfBounds() -> string {
            static indexOutOfBounds: OnceInit<string> = OnceInit::new();
            indexOutOfBounds
                .get_or_init(|| string("The index was outside the range of elements in the list."))
                .clone()
        }
        pub fn inputListWasEmpty() -> string {
            static inputListWasEmpty: OnceInit<string> = OnceInit::new();
            inputListWasEmpty
                .get_or_init(|| string("The input list was empty."))
                .clone()
        }
        pub fn inputMustBeNonNegative() -> string {
            static inputMustBeNonNegative: OnceInit<string> = OnceInit::new();
            inputMustBeNonNegative
                .get_or_init(|| string("The input must be non-negative."))
                .clone()
        }
        pub fn inputMustBePositive() -> string {
            static inputMustBePositive: OnceInit<string> = OnceInit::new();
            inputMustBePositive
                .get_or_init(|| string("The input must be positive."))
                .clone()
        }
        pub fn inputSequenceEmpty() -> string {
            static inputSequenceEmpty: OnceInit<string> = OnceInit::new();
            inputSequenceEmpty
                .get_or_init(|| string("The input sequence was empty."))
                .clone()
        }
        pub fn inputSequenceTooLong() -> string {
            static inputSequenceTooLong: OnceInit<string> = OnceInit::new();
            inputSequenceTooLong
                .get_or_init(|| string("The input sequence contains more than one element."))
                .clone()
        }
        pub fn keyNotFound() -> string {
            static keyNotFound: OnceInit<string> = OnceInit::new();
            keyNotFound
                .get_or_init(|| string("The item, key, or index was not found in the collection."))
                .clone()
        }
        pub fn keyNotFoundAlt() -> string {
            static keyNotFoundAlt: OnceInit<string> = OnceInit::new();
            keyNotFoundAlt
                .get_or_init(|| {
                    string("An index satisfying the predicate was not found in the collection.")
                })
                .clone()
        }
        pub fn listsHadDifferentLengths() -> string {
            static listsHadDifferentLengths: OnceInit<string> = OnceInit::new();
            listsHadDifferentLengths
                .get_or_init(|| string("The lists had different lengths."))
                .clone()
        }
        pub fn mapCannotBeMutated() -> string {
            static mapCannotBeMutated: OnceInit<string> = OnceInit::new();
            mapCannotBeMutated
                .get_or_init(|| string("Map values cannot be mutated."))
                .clone()
        }
        pub fn notAPermutation() -> string {
            static notAPermutation: OnceInit<string> = OnceInit::new();
            notAPermutation
                .get_or_init(|| string("The function did not compute a permutation."))
                .clone()
        }
        pub fn notEnoughElements() -> string {
            static notEnoughElements: OnceInit<string> = OnceInit::new();
            notEnoughElements
                .get_or_init(|| {
                    string("The input sequence has an insufficient number of elements.")
                })
                .clone()
        }
        pub fn outOfRange() -> string {
            static outOfRange: OnceInit<string> = OnceInit::new();
            outOfRange
                .get_or_init(|| string("The index is outside the legal range."))
                .clone()
        }
        pub fn resetNotSupported() -> string {
            static resetNotSupported: OnceInit<string> = OnceInit::new();
            resetNotSupported
                .get_or_init(|| string("Reset is not supported on this enumerator."))
                .clone()
        }
        pub fn setContainsNoElements() -> string {
            static setContainsNoElements: OnceInit<string> = OnceInit::new();
            setContainsNoElements
                .get_or_init(|| string("Set contains no elements."))
                .clone()
        }
    }
}
