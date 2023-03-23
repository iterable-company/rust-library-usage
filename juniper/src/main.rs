pub mod model;
pub mod mutation;
pub mod query;

use std::sync::{Arc, Mutex};

use crate::model::{Player, SportsKind, Team};
use crate::query::Query;
use indexmap::IndexMap;
use juniper::Variables;
use mutation::Mutation;
use once_cell::sync::Lazy;

#[macro_use]
extern crate juniper;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation>;

fn main() {
    // Run the executor.
    let (res, _errors) = juniper::execute(
        "query { player(id: 1){ name } }",
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &IN_MEMORY_DATA,
    )
    .unwrap();

    // Ensure the value matches.
    assert_eq!(
        res.as_object_value()
            .unwrap()
            .get_field_value("player")
            .unwrap()
            .as_object_value()
            .unwrap()
            .get_field_value("name")
            .unwrap()
            .as_string_value()
            .unwrap(),
        "Otani Shohei",
    );
}

static IN_MEMORY_DATA: Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(
        vec![
            (
                Team::new(
                    1,
                    "Angels".to_string(),
                    "Anaheim".to_string(),
                    SportsKind::Baseball,
                ),
                vec![
                    Player::new(1, "Otani Shohei".to_string(), 28, "2-way".to_string()),
                    Player::new(2, "Mike Trout".to_string(), 31, "outfielder".to_string()),
                ],
            ),
            (
                Team::new(
                    2,
                    "Man City".to_string(),
                    "Manchester".to_string(),
                    SportsKind::Baseball,
                ),
                vec![
                    Player::new(3, "Erling Haaland".to_string(), 22, "forward".to_string()),
                    Player::new(4, "Riyad Mahrez".to_string(), 32, "forward".to_string()),
                ],
            ),
        ]
        .into_iter()
        .collect(),
    ))
});
