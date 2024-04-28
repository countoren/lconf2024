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
        use fable_library_rust::Native_::LrcPtr;
        use fable_library_rust::String_::string;
        #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq,)]
        pub enum EcoSystem { FSharp, JS, Rust, }
        impl core::fmt::Display for Main::EcoSystem {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq,)]
        pub struct Message {
            pub ecoSystem: LrcPtr<Main::EcoSystem>,
            pub text: string,
        }
        impl core::fmt::Display for Main::Message {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
    }
}
pub use module_2b58b::*;
