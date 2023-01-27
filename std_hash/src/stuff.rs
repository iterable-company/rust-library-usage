use derive_new::new;

#[derive(Hash, new, Debug)]
pub struct Stuff {
    name: String,
    age: u32,
    address: String,
}
