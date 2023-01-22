#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod stuff {
    use derive_getters::Dissolve;
    use derive_new::new;
    pub struct Stuff {
        name: String,
        age: u8,
        address: String,
    }
    impl Stuff {
        pub fn dissolve(self) -> (String, u8, String) {
            (self.name, self.age, self.address)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Stuff {
        #[inline]
        fn clone(&self) -> Stuff {
            Stuff {
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                address: ::core::clone::Clone::clone(&self.address),
            }
        }
    }
    impl Stuff {
        ///Constructs a new `Stuff`.
        pub fn new(name: String, age: u8, address: String) -> Self {
            Stuff {
                name: name,
                age: age,
                address: address,
            }
        }
    }
}
use derive_getters::Getters;
use stuff::Stuff;
fn main() {
    let point = Point { x: 3, y: 4 };
    match (&3, &*point.x()) {
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
    match (&4, &*point.y()) {
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
    let stuff = Stuff::new(String::from("Taro YAMADA"), 50, String::from("Kyoto"));
    let (n, ag, ad) = stuff.clone().dissolve();
    match (&"Taro YAMADA".to_string(), &n) {
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
    match (&50, &ag) {
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
    match (&"Kyoto".to_string(), &ad) {
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
}
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn x(&self) -> &i32 {
        &self.x
    }
    pub fn y(&self) -> &i32 {
        &self.y
    }
}
