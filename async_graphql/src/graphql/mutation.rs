use std::sync::{Arc, Mutex};

use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::domain::lib::{NewPlayer, Player, Team};

pub struct Mutation(pub &'static Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>>);

#[async_graphql::Object]
impl Mutation {
    async fn add_player(
        &self,
        id: i32,
        name: String,
        age: u32,
        position: String,
    ) -> async_graphql::Result<Player> {
        let (team, player) = {
            let map = self
                .0
                .lock()
                .map_err(|_| async_graphql::Error::new("can't obtain lock"))?;
            let team = map.keys().find(|team| team.id == id).unwrap().clone();
            let mut ids: Vec<_> = map
                .values()
                .flat_map(|players| players.into_iter())
                .map(|player| player.id)
                .collect();
            ids.sort();
            let new_id = ids.last().unwrap() + 1;
            let player = Player {
                id: new_id,
                name: name,
                age: age,
                position: position,
            };
            (team, player)
        };
        let mut map = self
            .0
            .lock()
            .map_err(|_| async_graphql::Error::new("can't obtain lock"))?;
        map.get_mut(&team).unwrap().push(player.clone());
        Ok(player)
    }
}
