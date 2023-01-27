use derive_new::new;
#[derive(PartialOrd, PartialEq, new)]
pub struct Stuff {
    name: Name,
    age: u32,
}

#[derive(PartialOrd, PartialEq, Debug, new)]
pub struct Name{
    first: String,
    second: String,
}
