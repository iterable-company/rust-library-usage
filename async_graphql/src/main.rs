use axum::Extension;

pub mod domain;
pub mod graphql;

use domain::lib::{Player, SportsKind, Team};
use graphql::{handler::graphql_handler, schema_with::schema_with};
use indexmap::IndexMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let graphql_schema = schema_with(&IN_MEMORY_DATA);

    let app = axum::Router::new()
        .route("/graphql", axum::routing::post(graphql_handler))
        .layer(Extension(graphql_schema));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

use lazy_static::lazy_static;
lazy_static! {
    static ref IN_MEMORY_DATA: IndexMap<Team, Vec<Player>> = vec![
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
    .collect();
}
