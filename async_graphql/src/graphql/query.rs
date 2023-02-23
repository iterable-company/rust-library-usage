use indexmap::IndexMap;

use crate::domain::lib::{Player, Team};

pub struct Query(pub &'static IndexMap<Team, Vec<Player>>);

#[async_graphql::Object]
impl Query {
    async fn teams(&self) -> async_graphql::Result<Vec<Team>> {
        Ok(self.0.keys().map(|t| t.clone()).collect::<Vec<Team>>())
    }

    async fn get_player_by_id(&self, id: i32) -> async_graphql::Result<Player> {
        Ok(self
            .0
            .values()
            .flat_map(|players| players.into_iter())
            .find(|player| player.id == id)
            .unwrap()
            .to_owned())
    }
}
