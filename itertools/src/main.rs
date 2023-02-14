fn main() {
    diff_with_();
}

use itertools::{diff_with, Diff};
fn diff_with_() {
    match diff_with(vec![1, 2, 3], vec![2, 4, 6], |i, j| 2 * i == *j) {
        None => (),
        _ => panic!("!!!"),
    }

    match diff_with(vec![1, 2, 3], vec![1, 3, 2], |i, j| i == j) {
        Some(Diff::FirstMismatch(idx, i, j)) => {
            println!("i: {:?}, j: {:?}", i, j);
            assert_eq!(idx, 1)
        }
        _ => panic!("!!!"),
    }

    match diff_with(vec![1, 2, 3, 4, 5], vec![2, 4], |i, j| 2 * i == *j) {
        Some(Diff::Shorter(idx, i)) => {
            println!("i: {:?}", i);
            assert_eq!(idx, 2)
        }
        _ => panic!("!!!"),
    }

    match diff_with(vec![1, 2], vec![2, 4, 6, 8, 10], |i, j| 2 * i == *j) {
        Some(Diff::Longer(idx, j)) => {
            println!("j: {:?}", j);
            assert_eq!(idx, 2)
        }
        _ => panic!("!!!"),
    }
}
