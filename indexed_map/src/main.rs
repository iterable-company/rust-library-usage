fn main() {
    retain_insertion_order();
    sort_by_key();
    sort_by();
    swap_indices();
}

use indexmap::IndexMap;
use std::collections::HashMap;
fn retain_insertion_order() {
    let mut i_map = IndexMap::new();
    let mut map = HashMap::new();

    i_map.insert("USA", "USD");
    map.insert("USA", "USD");

    i_map.insert("Japan", "JPY");
    map.insert("Japan", "JPY");

    i_map.insert("Switzerland", "CHF");
    map.insert("Switzerland", "CHF");

    let mut i_map_key = i_map.keys();
    assert_eq!(i_map_key.next().unwrap().to_owned(), "USA");
    assert_eq!(i_map_key.next().unwrap().to_owned(), "Japan");
    assert_eq!(i_map_key.next().unwrap().to_owned(), "Switzerland");

    let mut map_key = map.keys();
    // assert_eq!(map_key.next().unwrap().to_owned(), "USA");       assertion error! not retain order of insertion
    // assert_eq!(map_key.next().unwrap().to_owned(), "Japan");
    // assert_eq!(map_key.next().unwrap().to_owned(), "Switzerland");
}

fn sort_by_key() {
    let mut i_map = IndexMap::new();

    i_map.insert("USA", "USA");
    i_map.insert("Japan", "JPY");
    i_map.insert("Switzerland", "CHF");
    i_map.sort_keys();

    let mut i_map_iter = i_map.iter();
    assert_eq!(
        i_map_iter
            .next()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .unwrap(),
        ("Japan", "JPY")
    );
    assert_eq!(
        i_map_iter
            .next()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .unwrap(),
        ("Switzerland", "CHF")
    );
    assert_eq!(
        i_map_iter
            .next()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .unwrap(),
        ("USA", "USA")
    );
}

fn sort_by() {
    let mut i_map = IndexMap::new();

    i_map.insert(1, 5);
    i_map.insert(2, 3);
    i_map.insert(3, 1);
    i_map.insert(4, 2);
    i_map.insert(5, 0);
    i_map.insert(6, 1);

    i_map.sort_by(|lk, lv, rk, rv| (*lk + *lv).cmp(&(*rk + *rv)));

    let mut i_map_key = i_map.keys();
    assert_eq!(i_map_key.next().unwrap().to_owned(), 3);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 2);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 5);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 1);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 4);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 6);
}

fn swap_indices() {
    let mut i_map = IndexMap::new();

    i_map.insert(1, 5);
    i_map.insert(2, 3);
    i_map.insert(3, 1);
    i_map.insert(4, 2);
    i_map.insert(5, 0);
    i_map.insert(6, 1);

    i_map.swap_indices(1, 4);
    let mut i_map_key = i_map.keys();
    assert_eq!(i_map_key.next().unwrap().to_owned(), 1);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 5);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 3);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 4);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 2);
    assert_eq!(i_map_key.next().unwrap().to_owned(), 6);
}
