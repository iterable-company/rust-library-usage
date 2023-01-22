use crate::point::Point;
use derive_more::{Add, From, FromStr, Into};
use derive_new::new;

#[derive(PartialEq, From, Into, Add, FromStr, Debug, new)]
pub struct Point2(Point);
