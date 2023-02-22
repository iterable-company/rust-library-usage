pub mod stuff;

use derive_getters::Getters;
use stuff::Stuff;

fn main() {
    getters();
    dissolve();
}

fn getters() {
    let point = Point { x: 3, y: 4 };
    assert_eq!(3, *point.x());
    assert_eq!(4, *point.y());
}

fn dissolve() {
    let stuff = Stuff::new(String::from("Taro YAMADA"), 50, String::from("Kyoto"));
    // プロパティが pub ではないので、以下のコードはコンパイルエラーになる
    //
    // let stuff = Stuff {
    //     name: String::from("Taro YAMADA"),
    //     age: 50,
    //     address: String::from("Kyoto"),
    // };
    //
    // プロパティがpubだと以下のようにして値を展開することができる。
    // let Stuff { name, age, address } = stuff;

    // フィールドがpubでなくても、以下のように展開できる
    let (n, ag, ad) = stuff.clone().dissolve();
    assert_eq!("Taro YAMADA".to_string(), n);
    assert_eq!(50, ag);
    assert_eq!("Kyoto".to_string(), ad);
}

#[derive(Getters)]
struct Point {
    x: i32,
    y: i32,
}
