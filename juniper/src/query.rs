use std::sync::{Arc, Mutex};

use crate::model::{Player, Team};

pub struct Query;

use indexmap::IndexMap;
use juniper::FieldResult;
use once_cell::sync::Lazy;

graphql_object!(Query: Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>> |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field player(&executor, id: i32) -> FieldResult<Player> {
        Ok(executor
            .context()
            .lock()?
            .values()
            .flat_map(|v| v.into_iter())
            .find(|player| player.id == id)
            .ok_or("not found player having id = {}")?
            .to_owned()
        )
    }
});
