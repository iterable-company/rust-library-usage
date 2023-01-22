pub mod stuff;
use stuff::Stuff;

fn main() {
    let mut stuff = Stuff::new(
        "Taro YAMADA".to_string(),
        50,
        1970,
        "Kyoto".to_string(),
        "R&D".to_string(),
        "A1".to_string(),
    );
    // stuff.rank() => pub設定ではないので、コンパイルエラーとなる
    assert_eq!("Taro YAMADA".to_string(), *stuff.name());
    assert_eq!(50, stuff.age()); // ここはコピーが渡されるので参照外しが不要
    assert_eq!(1970, *stuff.birth_year()); // ここはただのgetterなので参照外しが必要
    let address = stuff.address_mut();
    address.push_str(" Kamigyoku");
    assert_eq!("Kyoto Kamigyoku".to_string(), *stuff.address());

    stuff.set_division("Sales".to_string());
    assert_eq!("Sales".to_string(), *stuff.division());
}
