#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod stuff {
    use derive_new::new;
    pub struct Stuff {
        name: Name,
        age: u32,
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Stuff {
        #[inline]
        fn cmp(&self, other: &Stuff) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.name, &other.name) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.age, &other.age)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Stuff {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Stuff,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.name, &other.name) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.age, &other.age)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Stuff {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Stuff {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Name>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Stuff {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Stuff {
        #[inline]
        fn eq(&self, other: &Stuff) -> bool {
            self.name == other.name && self.age == other.age
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Stuff {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Stuff",
                "name",
                &self.name,
                "age",
                &&self.age,
            )
        }
    }
    impl Stuff {
        ///Constructs a new `Stuff`.
        pub fn new(name: Name, age: u32) -> Self {
            Stuff { name: name, age: age }
        }
    }
    pub struct Name {
        first: String,
        second: String,
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Name {
        #[inline]
        fn cmp(&self, other: &Name) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.first, &other.first) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.second, &other.second)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Name {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Name,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.first, &other.first) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.second, &other.second)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Name {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Name {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Name {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Name {
        #[inline]
        fn eq(&self, other: &Name) -> bool {
            self.first == other.first && self.second == other.second
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Name {
        #[inline]
        fn clone(&self) -> Name {
            Name {
                first: ::core::clone::Clone::clone(&self.first),
                second: ::core::clone::Clone::clone(&self.second),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Name",
                "first",
                &self.first,
                "second",
                &&self.second,
            )
        }
    }
    impl Name {
        ///Constructs a new `Name`.
        pub fn new(first: String, second: String) -> Self {
            Name {
                first: first,
                second: second,
            }
        }
    }
    pub enum Position {
        Pitcher,
        Catcher,
        FirstBaseMan,
        SecondBaseMan,
        ThirdBaseMan,
        ShortStop,
        RightFielder,
        CenterFielder,
        LeftFielder,
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Position {
        #[inline]
        fn cmp(&self, other: &Position) -> ::core::cmp::Ordering {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Position {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Position,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Position {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Position {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Position {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Position {
        #[inline]
        fn eq(&self, other: &Position) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Position {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Position::Pitcher => "Pitcher",
                    Position::Catcher => "Catcher",
                    Position::FirstBaseMan => "FirstBaseMan",
                    Position::SecondBaseMan => "SecondBaseMan",
                    Position::ThirdBaseMan => "ThirdBaseMan",
                    Position::ShortStop => "ShortStop",
                    Position::RightFielder => "RightFielder",
                    Position::CenterFielder => "CenterFielder",
                    Position::LeftFielder => "LeftFielder",
                },
            )
        }
    }
}
use stuff::{Name, Stuff};
use crate::stuff::Position;
fn main() {
    let mut stuffs = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 50),
            Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 40),
            Stuff::new(Name::new("Jiro".to_string(), "YAMADA".to_string()), 50),
        ]),
    );
    stuffs.sort_by(|l, r| l.cmp(r));
    match (
        &stuffs,
        &<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Stuff::new(Name::new("Jiro".to_string(), "YAMADA".to_string()), 50),
                Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 40),
                Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 50),
            ]),
        ),
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
    let mut names = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            Name::new("Taro".to_string(), "YAMADA".to_string()),
            Name::new("Taro".to_string(), "OKAMOTO".to_string()),
            Name::new("Jiro".to_string(), "YAMADA".to_string()),
            Name::new("Jiro".to_string(), "SAKAGAMI".to_string()),
            Name::new("Saburo".to_string(), "YAMADA".to_string()),
        ]),
    );
    names.sort_by(|l, r| l.cmp(r));
    match (
        &names,
        &<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Name::new("Jiro".to_string(), "SAKAGAMI".to_string()),
                Name::new("Jiro".to_string(), "YAMADA".to_string()),
                Name::new("Saburo".to_string(), "YAMADA".to_string()),
                Name::new("Taro".to_string(), "OKAMOTO".to_string()),
                Name::new("Taro".to_string(), "YAMADA".to_string()),
            ]),
        ),
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
    let name1 = Name::new("Jiro".to_string(), "SAKAGAMI".to_string());
    let name2 = Name::new("Jiro".to_string(), "YAMADA".to_string());
    match (&name1.clone().max(name2.clone()), &name2) {
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
    match (&name1.clone().min(name2.clone()), &name1) {
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
    match (
        &Name::new("Jiro".to_string(), "ASADA".to_string())
            .clamp(name1.clone(), name2.clone()),
        &name1,
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
    match (
        &Name::new("Jiro".to_string(), "TAMIYA".to_string())
            .clamp(name1.clone(), name2.clone()),
        &Name::new("Jiro".to_string(), "TAMIYA".to_string()),
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
    match (
        &Name::new("Jiro".to_string(), "YAMAGAMI".to_string())
            .clamp(name1.clone(), name2.clone()),
        &name2,
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
    let mut position_vec = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            Position::ThirdBaseMan,
            Position::Catcher,
            Position::FirstBaseMan,
            Position::LeftFielder,
            Position::RightFielder,
        ]),
    );
    position_vec.sort();
    match (
        &position_vec,
        &<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Position::Catcher,
                Position::FirstBaseMan,
                Position::ThirdBaseMan,
                Position::RightFielder,
                Position::LeftFielder,
            ]),
        ),
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
