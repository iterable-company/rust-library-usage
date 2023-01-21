use derive_new::new;
use typed_builder::TypedBuilder;
#[derive(TypedBuilder, PartialEq, Debug, new)]
pub struct Stuff {
    name: String,
    age: u8,
    address: String,
}
