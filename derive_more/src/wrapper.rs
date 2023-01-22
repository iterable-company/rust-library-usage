use crate::point::Point;
use derive_more::{Add, Display, From, Into};
use derive_new::new;

#[derive(PartialEq, From, Into, Add, Debug, new)]
pub struct Point2(Point);
