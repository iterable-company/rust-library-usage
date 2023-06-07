use derive_new::new;
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, new)]
pub struct Stuff {
    name: Name,
    age: u32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, new)]
pub struct Name {
    first: String,
    second: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Position {
    Pitcher,
    Catcher,
    FirstBaseMan,
    SecondBaseMan,
    ThirdBaseMan,
    ShortStop,
    RightFielder,
    CenterFielder,
    LeftFielder,
}
