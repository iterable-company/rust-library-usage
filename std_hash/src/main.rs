pub mod stuff;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use stuff::Stuff;
use std::assert_ne;

fn main() {
    
    let stuff = Stuff::new(
        "Taro YAMADA".to_string(),
        50,
        "Kyoto".to_string(),
    );
    let same_stuff = Stuff::new(
        "Taro YAMADA".to_string(),
        50,
        "Kyoto".to_string(),
    );
    assert_eq!({
        let mut hasher = DefaultHasher::new();
        stuff.hash(&mut hasher);
        hasher.finish()
    }, {
        let mut hasher = DefaultHasher::new();
        same_stuff.hash(&mut hasher);
        hasher.finish()
    });

    let different_stuff = Stuff::new(
        "Jiro YAMADA".to_string(),
        50,
        "Kyoto".to_string(),
    );
    assert_ne!({
        let mut hasher = DefaultHasher::new();
        stuff.hash(&mut hasher);
        hasher.finish()
    }, {
        let mut hasher = DefaultHasher::new();
        different_stuff.hash(&mut hasher);
        hasher.finish()
    })
}
