#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod stuff {
    use derive_new::new;
    use getset;
    use getset::{CopyGetters, Getters, MutGetters, Setters};
    pub struct Stuff {
        #[getset(get = "pub")]
        name: String,
        #[getset(get_copy = "pub")]
        age: u8,
        #[getset(get = "pub")]
        birth_year: u16,
        #[getset(get_mut = "pub", get = "pub")]
        address: String,
        #[getset(set = "pub", get = "pub")]
        division: String,
        #[getset(get)]
        rank: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Stuff {
        #[inline]
        fn clone(&self) -> Stuff {
            Stuff {
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                birth_year: ::core::clone::Clone::clone(&self.birth_year),
                address: ::core::clone::Clone::clone(&self.address),
                division: ::core::clone::Clone::clone(&self.division),
                rank: ::core::clone::Clone::clone(&self.rank),
            }
        }
    }
    impl Stuff {
        ///Constructs a new `Stuff`.
        pub fn new(
            name: String,
            age: u8,
            birth_year: u16,
            address: String,
            division: String,
            rank: String,
        ) -> Self {
            Stuff {
                name: name,
                age: age,
                birth_year: birth_year,
                address: address,
                division: division,
                rank: rank,
            }
        }
    }
    impl Stuff {
        #[inline(always)]
        pub fn age(&self) -> u8 {
            self.age
        }
    }
    impl Stuff {
        #[inline(always)]
        pub fn name(&self) -> &String {
            &self.name
        }
        #[inline(always)]
        pub fn birth_year(&self) -> &u16 {
            &self.birth_year
        }
        #[inline(always)]
        pub fn address(&self) -> &String {
            &self.address
        }
        #[inline(always)]
        pub fn division(&self) -> &String {
            &self.division
        }
        #[inline(always)]
        fn rank(&self) -> &String {
            &self.rank
        }
    }
    impl Stuff {
        #[inline(always)]
        pub fn address_mut(&mut self) -> &mut String {
            &mut self.address
        }
    }
    impl Stuff {
        #[inline(always)]
        pub fn set_division(&mut self, val: String) -> &mut Self {
            self.division = val;
            self
        }
    }
}
use stuff::Stuff;
fn main() {
    let mut stuff = Stuff::new(
        "Taro YAMADA".to_string(),
        50,
        1970,
        "Kyoto".to_string(),
        "R&D".to_string(),
        "A1".to_string(),
    );
    match (&"Taro YAMADA".to_string(), &*stuff.name()) {
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
    match (&50, &stuff.age()) {
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
    match (&1970, &*stuff.birth_year()) {
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
    let address = stuff.address_mut();
    address.push_str(" Kamigyoku");
    match (&"Kyoto Kamigyoku".to_string(), &*stuff.address()) {
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
    stuff.set_division("Sales".to_string());
    match (&"Sales".to_string(), &*stuff.division()) {
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
