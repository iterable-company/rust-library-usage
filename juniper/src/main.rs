pub mod model;
pub mod mutation;
pub mod query;

use std::io;
use std::sync::{Arc, Mutex};

use crate::model::{Player, SportsKind, Team};
use crate::query::Query;
use actix_cors::Cors;
use actix_web::{
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use indexmap::IndexMap;
use juniper::http::GraphQLRequest;
use juniper::EmptySubscription;
use mutation::Mutation;
use once_cell::sync::Lazy;

#[macro_use]
extern crate juniper;

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription>;
fn create_schema(data_store: &'static Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>>) -> Schema {
    Schema::new(
        Query { 0: &data_store },
        Mutation { 0: &data_store },
        EmptySubscription::new(),
    )
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema(&IN_MEMORY_DATA));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
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
