use indexmap::IndexMap;

use crate::domain::lib::{NewPlayer, Player, Team};

pub struct Mutation(pub &'static IndexMap<Team, Vec<Player>>);

#[async_graphql::Object]
impl Mutation {
    async fn add_player(&self, _team_id: i32, _player: NewPlayer) -> async_graphql::Result<Player> {
        todo!()
    }
}
