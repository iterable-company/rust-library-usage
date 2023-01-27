pub mod stuff;
use stuff::{Stuff, Name};

fn main() {
    let stuff1 = Stuff::new(
        Name::new("Taro".to_string(), "YAMADA".to_string()),
        50
    );
    let stuff2 = Stuff::new(
        Name::new("Taro".to_string(), "OKAMOTO".to_string()),
        50
    );
    assert!(stuff1 > stuff2);
}
