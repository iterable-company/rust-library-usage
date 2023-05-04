fn main() {
    option_take();
    mem_take();
}

fn option_take() {
    let mut op1 = Some(1);
    let op2 = op1.take();
    assert_eq!(op1, None);
    assert_eq!(op2, Some(1));
}

#[derive(new, Default, PartialEq, Debug)]
struct Human {
    name: String,
    age: u32,
}

use std::mem::take;

use derive_new::new;
fn mem_take() {
    let mut v1 = vec![1, 2];
    let v2 = take(&mut v1);

    assert_eq!(v1, Vec::new());
    assert_eq!(v2, vec![1, 2]);

    let mut h1 = Human::new("Otani Shohei".to_string(), 28);
    let h2 = take(&mut h1);

    assert_eq!(h1, Human::new("".to_string(), 0));
    assert_eq!(h2, Human::new("Otani Shohei".to_string(), 28));
}
