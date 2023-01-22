pub mod point;
pub mod wrapper;

use point::Point;
use wrapper::Point2;

fn main() {
    // Point
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let p3 = p1 + p2;
    assert_eq!(p3, Point::new(4, 6));

    let p4 = Point::from((5, 7));
    assert_eq!(p4, Point::new(5, 7));
    let p5: Point = (2, 3).into();
    assert_eq!(p5, Point::new(2, 3));

    // wrapper
    let p6 = Point2::new((1, 2).into());
    let p7 = Point2::new((3, 5).into());
    let p8 = p6 + p7;
    assert_eq!(p8, Point2::new((4, 7).into()));
    let p9 = Point::new(1, 2);
    let p10 = Point2::from(p9);
    assert_eq!(p10, Point2::new((1, 2).into()));
    let p11: Point = p10.into();
    assert_eq!(p11, Point::new(1, 2));
}
