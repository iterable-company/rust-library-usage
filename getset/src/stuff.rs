use derive_new::new;
use getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};
#[derive(Clone, new, CopyGetters, Getters, MutGetters, Setters)]
pub struct Stuff {
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    // get_copyはCopyトレイトを実装しているものにしかつけられない
    age: u8,
    #[getset(get = "pub")]
    birth_year: u16,
    #[getset(get_mut = "pub", get = "pub")]
    address: String,
    #[getset(set = "pub", get = "pub")]
    division: String,
    #[getset(get)]
    rank: String,
}
