#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod stuff {
    use derive_new::new;
    pub struct Stuff {
        name: String,
        age: u32,
        address: String,
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Stuff {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.name, state);
            ::core::hash::Hash::hash(&self.age, state);
            ::core::hash::Hash::hash(&self.address, state)
        }
    }
    impl Stuff {
        ///Constructs a new `Stuff`.
        pub fn new(name: String, age: u32, address: String) -> Self {
            Stuff {
                name: name,
                age: age,
                address: address,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Stuff {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Stuff",
                "name",
                &&self.name,
                "age",
                &&self.age,
                "address",
                &&self.address,
            )
        }
    }
}
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use stuff::Stuff;
use std::assert_ne;
fn main() {
    let stuff = Stuff::new("Taro YAMADA".to_string(), 50, "Kyoto".to_string());
    let same_stuff = Stuff::new("Taro YAMADA".to_string(), 50, "Kyoto".to_string());
    match (
        &{
            let mut hasher = DefaultHasher::new();
            stuff.hash(&mut hasher);
            hasher.finish()
        },
        &{
            let mut hasher = DefaultHasher::new();
            same_stuff.hash(&mut hasher);
            hasher.finish()
        },
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let different_stuff = Stuff::new("Jiro YAMADA".to_string(), 50, "Kyoto".to_string());
    match (
        &{
            let mut hasher = DefaultHasher::new();
            stuff.hash(&mut hasher);
            hasher.finish()
        },
        &{
            let mut hasher = DefaultHasher::new();
            different_stuff.hash(&mut hasher);
            hasher.finish()
        },
    ) {
        (left_val, right_val) => {
            if *left_val == *right_val {
                let kind = ::core::panicking::AssertKind::Ne;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    }
}
