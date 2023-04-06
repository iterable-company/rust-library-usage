use std::sync::{Arc, Mutex};

use crate::model::{NewPlayer, Player, Team};

pub struct Mutation(pub(crate) &'static Lazy<Arc<Mutex<IndexMap<Team, Vec<Player>>>>>);

use indexmap::IndexMap;
use juniper::FieldResult;
use once_cell::sync::Lazy;

#[juniper::graphql_object]
impl Mutation {
    fn addPlayer(&self, team_id: i32, new_player: NewPlayer) -> FieldResult<Player> {
        let (team, player): (Team, Player) = {
            let map = self.0.lock().map_err(|_| "can't obtain lock")?;
            let team = map
                .keys()
                .find(|team| team.id == team_id)
                .ok_or(format!("not found team: {}", team_id))?;
            let mut ids: Vec<_> = map
                .values()
                .flat_map(|players| players.into_iter())
                .map(|player| player.id)
                .collect();
            ids.sort();
            let new_id = ids.last().unwrap() + 1;
            let player = Player::from_with_id(new_player, new_id);
            (team.clone(), player)
        };
        let mut map = self.0.lock().map_err(|_| "can't obtain lock")?;
        map.get_mut(&team).unwrap().push(player.clone());
        Ok(player)
    }
}
