use derive_getters::Dissolve;
use derive_new::new;
#[derive(Dissolve, Clone, new)]
pub struct Stuff {
    name: String,
    age: u8,
    address: String,
}
