fn main() {
    diff_with_();
    zip_longest_();
}

use itertools::{diff_with, Diff, Itertools};
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

fn zip_longest_() {
    // case: same length
    assert_eq!(
        zip_longest__(
            vec![1, 2, 3],
            vec!["one".to_string(), "two".to_string(), "three".to_string()]
        ),
        vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (3, "three".to_string())
        ]
    );
    // case: left is longger than right
    assert_eq!(
        zip_longest__(
            vec![1, 2, 3, 4],
            vec!["one".to_string(), "two".to_string(), "three".to_string()]
        ),
        vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (3, "three".to_string()),
            (4, "".to_string())
        ]
    );
    // case: left is shorter than right
    assert_eq!(
        zip_longest__(
            vec![1, 2],
            vec!["one".to_string(), "two".to_string(), "three".to_string()]
        ),
        vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (0, "three".to_string()),
        ]
    );
}

use itertools::EitherOrBoth::*;
fn zip_longest__(nums: Vec<i32>, strings: Vec<String>) -> Vec<(i32, String)> {
    nums.into_iter()
        .zip_longest(strings.into_iter())
        .filter_map(|elem| match elem {
            Both(num, string) => Some((num, string)),
            Left(num) => Some((num, "".to_string())),
            Right(string) => Some((0, string)),
        })
        .collect()
}
