use std::sync::{Arc, Mutex};

use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::domain::lib::{Player, Team};

pub struct Query(pub &'static Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>>);

#[async_graphql::Object]
impl Query {
    async fn teams(&self) -> async_graphql::Result<Vec<Team>> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .keys()
            .map(|t| t.clone())
            .collect::<Vec<Team>>())
    }

    async fn get_player_by_id(&self, id: i32) -> async_graphql::Result<Player> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .values()
            .flat_map(|players| players.into_iter())
            .find(|player| player.id == id)
            .unwrap()
            .to_owned())
    }
}
