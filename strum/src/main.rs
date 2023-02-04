use std::str::FromStr;
fn main() {
    strum_display();
    strum_enumstring();
}

// 1. If there is a to_string property, this value will be used. There can only be one per variant.
// 2. Of the various serialize properties, the value with the longest length is chosen. If that behavior isn’t desired, you should use to_string.
// 3. The name of the variant will be used if there are no serialize or to_string attributes.
fn strum_display() {
    // 1.
    let keeper = Player::GoalKeeper { height: 180 };
    assert_eq!(String::from("GOALKEEPER"), format!("{}", keeper));

    // 2.
    let forward = Player::Forward;
    assert_eq!(String::from("forward"), format!("{}", forward));

    // 3. case 1
    let catcher = Player::Catcher;
    assert_eq!(String::from("Catcher"), format!("{}", catcher));

    // 3. case 2
    let pitcher = Player::Pitcher {
        limit_number_of_pitch: 100,
    };
    assert_eq!(String::from("Pitcher"), format!("{}", pitcher));

    // assert_eq!(String::from("Orange"), format!("{}", orange)); => Compilation Error!  Fruit::Orange doesn't implement trait Display.
}
use strum_macros::{Display, EnumString};
#[derive(Display)]
enum Player {
    #[strum(serialize = "gk", to_string = "GOALKEEPER")]
    GoalKeeper {
        height: u8,
    },
    #[strum(serialize = "FW", serialize = "forward")]
    Forward,
    Catcher,
    Pitcher {
        limit_number_of_pitch: u32,
    },
}

fn strum_enumstring() {
    // let grape = Player::from_str("Catcher"); Compilation Error!  Player doesn't implement trait FromStr

    let orange1 = Fruit::from_str("or");
    let orange2 = Fruit::from_str("o");
    let orange3 = Fruit::from_str("Orange");
    let orange4 = Fruit::from_str("orange");

    assert_eq!(Ok(Fruit::Orange), orange1);
    assert_eq!(Ok(Fruit::Orange), orange2);
    assert!(orange3.is_err()); // => this result in parse error because strum_macros override the implementation of from_str
    assert!(orange4.is_err());

    let grape = Fruit::from_str("Grape");
    assert_eq!(Ok(Fruit::Grape), grape);
}
#[derive(EnumString, PartialEq, Debug)]
enum Fruit {
    Grape,
    #[strum(serialize = "or", serialize = "o")]
    Orange,
}
