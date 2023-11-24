#[macro_use]
extern crate a_macro;

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

#[test]
fn test_convert_row_to_domain() {
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
        _exclude_field: 11.0
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
}
