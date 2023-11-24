use a_macro::{ConvertTo, EnumConvertFrom};
use derive_new::new;

fn main() {
    let row = Row {
        int: 1,
        op_int: Some(2),
        kind: Kind::Other,
        op_kind: Some(Kind::One),
        sub: SubRow {
            str: "str3".to_string(),
        },
        op_sub: Some(SubRow {
            str: "str4".to_string(),
        }),
        int2: 10,
        _exclude_field: 11.0,
    };
    assert_eq!(
        Domain::from(row),
        Domain {
            int: 1,
            op_int: Some(2),
            kind: Kind::Other,
            op_kind: Some(Kind::One),
            sub: SubDomain {
                str: "str3".to_string(),
            },
            op_sub: Some(SubDomain {
                str: "str4".to_string(),
            }),
            int2: 11,
        }
    );


    assert_eq!(KindTo1::from(KindFrom::One), KindTo1::One,);
    assert_eq!(KindTo2::from(KindFrom::One), KindTo2::One,);
    assert_eq!(KindTo1::from(KindFrom::Two), KindTo1::Two,);
    assert_eq!(KindTo2::from(KindFrom::Two), KindTo2::Two,);
    assert_eq!(KindTo1::from(KindFrom::Three), KindTo1::Three,);
    assert_eq!(KindTo2::from(KindFrom::Three), KindTo2::Three,);
    assert_eq!(KindTo1::from(KindFrom::Other), KindTo1::Other,);
    assert_eq!(KindTo2::from(KindFrom::Other), KindTo2::Other,);
}

#[derive(PartialEq, Debug)]
enum Kind {
    One,
    Other,
}

struct SubRow {
    pub str: String,
}

#[derive(PartialEq, Debug)]
struct SubDomain {
    pub str: String,
}

impl From<SubRow> for SubDomain {
    fn from(row: SubRow) -> Self {
        Self { str: row.str }
    }
}

fn add_one(value: i32) -> i32 {
    value + 1
}

#[derive(ConvertTo)]
#[target(Domain)]
struct Row {
    pub int: i32,
    pub op_int: Option<i32>,
    pub kind: Kind,
    pub op_kind: Option<Kind>,
    pub sub: SubRow,
    pub op_sub: Option<SubRow>,
    #[convert_with(add_one)]
    pub int2: i32,
    #[exclude_convert]
    pub _exclude_field: f64,
}

#[allow(clippy::too_many_arguments)]
#[derive(PartialEq, Debug, new)]
struct Domain {
    pub int: i32,
    pub op_int: Option<i32>,
    pub kind: Kind,
    pub op_kind: Option<Kind>,
    pub sub: SubDomain,
    pub op_sub: Option<SubDomain>,
    pub int2: i32,
}

#[derive(EnumConvertFrom, PartialEq)]
#[enum_target(KindTo1, KindTo2)]
enum KindFrom {
    One,
    Two,
    Three,
    Other,
}

#[derive(Debug, PartialEq)]
enum KindTo1 {
    One,
    Two,
    Three,
    Other,
}

#[derive(Debug, PartialEq)]
enum KindTo2 {
    One,
    Two,
    Three,
    Other,
}
