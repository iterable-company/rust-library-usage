pub mod stuff;
use stuff::Stuff;

fn main() {
    let stuff = Stuff::builder()
        .name(String::from("Someone"))
        .age(11)
        .address(String::from("Somewhere"))
        .build();

    assert_eq!(
        stuff,
        Stuff::new(String::from("Someone"), 11, String::from("Somewhere"))
    )
    // フィールドが足りなくてコンパイルエラーになる
    // let stuff1 = Stuff::builder()
    //     .name(String::from("Someone"))
    //     .age(11)
    //     .build();

    // フィールドが二重に定義されているのでコンパイルエラーになる
    // let stuff2 = Stuff::builder()
    //     .name(String::from("Someone"))
    //     .age(11)
    //     .address(String::from("Somewhere"))
    //     .name(String::from("Someone"))
    //     .build();
}
