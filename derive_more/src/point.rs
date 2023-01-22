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

use anyhow::anyhow;
use regex::Regex;
use std::str::FromStr;
impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\(([0-9]+),([0-9]+)\)")?;
        let str = s
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>();
        let caps = re
            .captures(&str)
            .ok_or(anyhow!("can't convert into Point from {}", str))?;
        let x = caps.get(1).map_or("0", |m| m.as_str());
        let y = caps.get(2).map_or("0", |m| m.as_str());
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

use std::fmt::Display;
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
