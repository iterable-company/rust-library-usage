use derive_new::new;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Point {
    x: i32,
    y: i32,
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
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
