use crate::point::Point;
use derive_more::{Add, Display, From, FromStr, Into};
use derive_new::new;

#[derive(PartialEq, From, Into, Add, Display, FromStr, Debug, new)]
pub struct Point2(Point);
