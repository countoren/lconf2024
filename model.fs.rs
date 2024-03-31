#![allow(dead_code,)]
#![allow(non_camel_case_types,)]
#![allow(non_snake_case,)]
#![allow(non_upper_case_globals,)]
#![allow(unreachable_code,)]
#![allow(unused_attributes,)]
#![allow(unused_imports,)]
#![allow(unused_macros,)]
#![allow(unused_parens,)]
#![allow(unused_variables,)]
mod module_2b58b {
    pub mod Main {
        use super::*;
        use fable_library_rust::Native_::Any;
        use fable_library_rust::String_::sprintf;
        use fable_library_rust::String_::string;
        pub fn helloEcoSystem<a: Clone + 'static>(subEcoSystem: a) -> string {
            sprintf!("Hello Sub Eco System {} from Parent Eco system",
                     &subEcoSystem)
        }
    }
}
pub use module_2b58b::*;
