#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::str::FromStr;
fn main() {
    strum_display();
    strum_enumstring();
}
fn strum_display() {
    let keeper = Player::GoalKeeper { height: 180 };
    match (
        &String::from("GOALKEEPER"),
        &{
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&keeper)],
                ),
            );
            res
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
    let forward = Player::Forward;
    match (
        &String::from("forward"),
        &{
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&forward)],
                ),
            );
            res
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
    let catcher = Player::Catcher;
    match (
        &String::from("Catcher"),
        &{
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&catcher)],
                ),
            );
            res
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
    let pitcher = Player::Pitcher {
        limit_number_of_pitch: 100,
    };
    match (
        &String::from("Pitcher"),
        &{
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&pitcher)],
                ),
            );
            res
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
}
use strum_macros::{Display, EnumString};
enum Player {
    #[strum(serialize = "gk", to_string = "GOALKEEPER")]
    GoalKeeper { height: u8 },
    #[strum(serialize = "FW", serialize = "forward")]
    Forward,
    Catcher,
    Pitcher { limit_number_of_pitch: u32 },
}
impl ::core::fmt::Display for Player {
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
        match *self {
            Player::GoalKeeper { .. } => f.pad("GOALKEEPER"),
            Player::Forward => f.pad("forward"),
            Player::Catcher => f.pad("Catcher"),
            Player::Pitcher { .. } => f.pad("Pitcher"),
        }
    }
}
fn strum_enumstring() {
    let orange1 = Fruit::from_str("or");
    let orange2 = Fruit::from_str("o");
    let orange3 = Fruit::from_str("Orange");
    let orange4 = Fruit::from_str("orange");
    match (&Ok(Fruit::Orange), &orange1) {
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
    match (&Ok(Fruit::Orange), &orange2) {
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
    if !orange3.is_err() {
        ::core::panicking::panic("assertion failed: orange3.is_err()")
    }
    if !orange4.is_err() {
        ::core::panicking::panic("assertion failed: orange4.is_err()")
    }
    let grape = Fruit::from_str("Grape");
    match (&Ok(Fruit::Grape), &grape) {
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
enum Fruit {
    Grape,
    #[strum(serialize = "or", serialize = "o")]
    Orange,
}
#[allow(clippy::use_self)]
impl ::core::str::FromStr for Fruit {
    type Err = ::strum::ParseError;
    fn from_str(
        s: &str,
    ) -> ::core::result::Result<Fruit, <Self as ::core::str::FromStr>::Err> {
        ::core::result::Result::Ok(
            match s {
                "Grape" => Fruit::Grape,
                "or" => Fruit::Orange,
                "o" => Fruit::Orange,
                _ => {
                    return ::core::result::Result::Err(
                        ::strum::ParseError::VariantNotFound,
                    );
                }
            },
        )
    }
}
#[allow(clippy::use_self)]
impl ::core::convert::TryFrom<&str> for Fruit {
    type Error = ::strum::ParseError;
    fn try_from(
        s: &str,
    ) -> ::core::result::Result<Fruit, <Self as ::core::convert::TryFrom<&str>>::Error> {
        ::core::str::FromStr::from_str(s)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Fruit {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Fruit {
    #[inline]
    fn eq(&self, other: &Fruit) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Fruit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Fruit::Grape => ::core::fmt::Formatter::write_str(f, "Grape"),
            Fruit::Orange => ::core::fmt::Formatter::write_str(f, "Orange"),
        }
    }
}
