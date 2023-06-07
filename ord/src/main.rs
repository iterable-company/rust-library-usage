pub mod stuff;
use stuff::{Name, Stuff};

use crate::stuff::Position;

fn main() {
    let mut stuffs = vec![
        Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 50),
        Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 40),
        Stuff::new(Name::new("Jiro".to_string(), "YAMADA".to_string()), 50),
    ];
    stuffs.sort_by(|l, r| l.cmp(r));
    assert_eq!(
        stuffs,
        vec![
            Stuff::new(Name::new("Jiro".to_string(), "YAMADA".to_string()), 50),
            Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 40),
            Stuff::new(Name::new("Taro".to_string(), "YAMADA".to_string()), 50),
        ]
    );

    let mut names = vec![
        Name::new("Taro".to_string(), "YAMADA".to_string()),
        Name::new("Taro".to_string(), "OKAMOTO".to_string()),
        Name::new("Jiro".to_string(), "YAMADA".to_string()),
        Name::new("Jiro".to_string(), "SAKAGAMI".to_string()),
        Name::new("Saburo".to_string(), "YAMADA".to_string()),
    ];
    names.sort_by(|l, r| l.cmp(r));
    assert_eq!(
        names,
        vec![
            Name::new("Jiro".to_string(), "SAKAGAMI".to_string()),
            Name::new("Jiro".to_string(), "YAMADA".to_string()),
            Name::new("Saburo".to_string(), "YAMADA".to_string()),
            Name::new("Taro".to_string(), "OKAMOTO".to_string()),
            Name::new("Taro".to_string(), "YAMADA".to_string()),
        ]
    );

    let name1 = Name::new("Jiro".to_string(), "SAKAGAMI".to_string());
    let name2 = Name::new("Jiro".to_string(), "YAMADA".to_string());

    assert_eq!(name1.clone().max(name2.clone()), name2);
    assert_eq!(name1.clone().min(name2.clone()), name1);

    assert_eq!(
        Name::new("Jiro".to_string(), "ASADA".to_string()).clamp(name1.clone(), name2.clone()),
        name1
    ); // self < arg1 < arg2
    assert_eq!(
        Name::new("Jiro".to_string(), "TAMIYA".to_string()).clamp(name1.clone(), name2.clone()),
        Name::new("Jiro".to_string(), "TAMIYA".to_string())
    ); // arg1 < self < arg2
    assert_eq!(
        Name::new("Jiro".to_string(), "YAMAGAMI".to_string()).clamp(name1.clone(), name2.clone()),
        name2
    ); // arg1 < arg2 < self

    let mut position_vec = vec![
        Position::ThirdBaseMan,
        Position::Catcher,
        Position::FirstBaseMan,
        Position::LeftFielder,
        Position::RightFielder,
    ];
    position_vec.sort();
    assert_eq!(
        position_vec,
        vec![
            Position::Catcher,
            Position::FirstBaseMan,
            Position::ThirdBaseMan,
            Position::RightFielder,
            Position::LeftFielder,
        ]
    );
}
