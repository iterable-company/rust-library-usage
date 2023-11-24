use std::marker::PhantomData;

use derive_new::new;
use typed_builder::TypedBuilder;
#[derive(TypedBuilder, PartialEq, Debug, new)]
pub struct Stuff {
    name: String,
    age: u8,
    address: String,
}

pub struct Id<T> {
    id: String,
    _phantom: PhantomData<T>,
}

// #[derive(TypedBuilder)]
// pub struct Hoge {
//     hoge_id: Id<Self>,
//     value: i32,
// }
