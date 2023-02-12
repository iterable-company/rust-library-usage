use std::str::FromStr;
fn main() {
    retain_insertion_order();
    sort_by_key();
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

    i_map.insert("USA", "USD");
    i_map.insert("Japan", "JPY");
    i_map.insert("Switzerland", "CHF");
    i_map.sort_keys();

    let mut i_map_key = i_map.keys();
    assert_eq!(i_map_key.next().unwrap().to_owned(), "Japan");
    assert_eq!(i_map_key.next().unwrap().to_owned(), "Switzerland");
    assert_eq!(i_map_key.next().unwrap().to_owned(), "USA");
}
