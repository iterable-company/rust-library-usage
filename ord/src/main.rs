pub mod stuff;
use stuff::{Stuff, Name};

fn main() {
    let mut stuffs = vec![
        Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 50),
        Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 40),
        Stuff::new(Name::new("Jiro".to_string(), "YAMADA".to_string()), 50),
    ];
    stuffs.sort_by(|l, r| l.cmp(r));
    assert_eq!(stuffs, vec![
        Stuff::new(Name::new("Jiro".to_string(), "YAMADA".to_string()), 50),
        Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 40),
        Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 50),
    ]);

    let mut names = vec![
        Name::new("Taro".to_string(), "YAMADA".to_string()),
        Name::new("Taro".to_string(), "OKAMOTO".to_string()),
        Name::new("Jiro".to_string(), "YAMADA".to_string()),
        Name::new("Jiro".to_string(), "SAKAGAMI".to_string()),
        Name::new("Saburo".to_string(), "YAMADA".to_string()),
    ];
    names.sort_by(|l, r| l.cmp(r));
    assert_eq!(names, vec![
        Name::new("Jiro".to_string(), "SAKAGAMI".to_string()),
        Name::new("Jiro".to_string(), "YAMADA".to_string()),
        Name::new("Saburo".to_string(), "YAMADA".to_string()),
        Name::new("Taro".to_string(), "OKAMOTO".to_string()),
        Name::new("Taro".to_string(), "YAMADA".to_string()),
    ])
}
