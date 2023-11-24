use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::poi::Poi;

mod poi;

fn main() {
    let mut content = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(
            r#""name1","value","name2"
"hoge","123","fuga""#
                .as_bytes(),
        );
    for record in content.deserialize() {
        let record: Hoge = record.unwrap();
        println!("{:?}", record);
    }
    let hoge = Poi { hoge: 1 };
    println!("{:?}", hoge);
}

#[derive(Deserialize, Debug)]
struct Hoge {
    pub name1: String,
    #[serde(flatten)]
    pub fuga: Fuga,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Fuga {
    pub name2: String,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaleRankingRow {
    #[serde(flatten)]
    sku: SaleRankingInfoRow,
    stock_quantity: i32,
    sales_quantity: i32,
    sales_amount: BigDecimal,
    gross_profit_amount: BigDecimal,
}

#[derive(Debug, Clone, Deserialize)]
struct SaleRankingInfoRow {
    sku_id: String,
    sku_customer_id: String,
    sku_name: String,
    sku_photo_url: Option<String>,
}
