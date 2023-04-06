use std::sync::{Arc, Mutex};

use crate::model::{Player, Team};

pub struct Query(pub(crate) &'static Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>>);

use indexmap::IndexMap;
use juniper::FieldResult;
use once_cell::sync::Lazy;

#[juniper::graphql_object]
impl Query {
    fn player(&self, id: i32) -> FieldResult<Player> {
        Ok(self
            .0
            .lock()?
            .values()
            .flat_map(|v| v.into_iter())
            .find(|player| player.id == id)
            .ok_or("not found player having id = {}")?
            .to_owned())
    }
}
