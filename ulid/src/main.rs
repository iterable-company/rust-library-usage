fn main() {
    partial_ord();
    increment_random();
}

use std::{thread, time};
use ulid::Ulid;
fn partial_ord() {
    let u1 = Ulid::new();
    thread::sleep(time::Duration::from_millis(100));
    let u2 = Ulid::new();

    assert!(u1 < u2);
}

fn increment_random() {
    let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    let result = Ulid::from_string(text).unwrap();

    let timestamp = result.timestamp_ms();
    let random = result.random();

    let incremented = result.increment().unwrap();

    assert_eq!(incremented.timestamp_ms(), timestamp);
    assert_eq!(Ulid::from_parts(timestamp, random + 1), incremented);
}
