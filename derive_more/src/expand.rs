#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod point {
    use derive_new::new;
    pub struct Point {
        x: i32,
        y: i32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Point {
        #[inline]
        fn clone(&self) -> Point {
            Point {
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Point {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Point",
                "x",
                &&self.x,
                "y",
                &&self.y,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Point {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Point {
        #[inline]
        fn eq(&self, other: &Point) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    impl Point {
        ///Constructs a new `Point`.
        pub fn new(x: i32, y: i32) -> Self {
            Point { x: x, y: y }
        }
    }
    use std::ops::Add;
    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Point) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    use std::convert::From;
    impl From<(i32, i32)> for Point {
        fn from(value: (i32, i32)) -> Self {
            Self { x: value.0, y: value.1 }
        }
    }
}
pub mod wrapper {
    use crate::point::Point;
    use derive_more::{Add, Display, From, Into};
    use derive_new::new;
    pub struct Point2(Point);
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Point2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Point2 {
        #[inline]
        fn eq(&self, other: &Point2) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(Point)> for Point2 {
        #[inline]
        fn from(original: (Point)) -> Point2 {
            Point2(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<Point2> for (Point) {
        #[inline]
        fn from(original: Point2) -> Self {
            (original.0)
        }
    }
    impl ::core::ops::Add for Point2 {
        type Output = Point2;
        #[inline]
        fn add(self, rhs: Point2) -> Point2 {
            Point2(self.0.add(rhs.0))
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Point2 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Point2", &&self.0)
        }
    }
    impl Point2 {
        ///Constructs a new `Point2`.
        pub fn new(f0: Point) -> Self {
            Point2(f0)
        }
    }
}
use point::Point;
use wrapper::Point2;
fn main() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let p3 = p1 + p2;
    match (&p3, &Point::new(4, 6)) {
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
    let p4 = Point::from((5, 7));
    match (&p4, &Point::new(5, 7)) {
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
    let p5: Point = (2, 3).into();
    match (&p5, &Point::new(2, 3)) {
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
    let p6 = Point2::new((1, 2).into());
    let p7 = Point2::new((3, 5).into());
    let p8 = p6 + p7;
    match (&p8, &Point2::new((4, 7).into())) {
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
    let p9 = Point::new(1, 2);
    let p10 = Point2::from(p9);
    match (&p10, &Point2::new((1, 2).into())) {
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
