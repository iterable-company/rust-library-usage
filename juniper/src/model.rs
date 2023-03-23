use derive_new::new;
use juniper::GraphQLInputObject;

#[derive(PartialEq, Eq, Hash, Clone, GraphQLEnum)]
pub enum SportsKind {
    Football,
    Baseball,
}

#[derive(new, Eq, PartialEq, Hash, Clone, GraphQLObject)]
#[graphql(description = "team")]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub kind: SportsKind,
}

#[derive(new, Clone, Debug, GraphQLObject)]
#[graphql(description = "player")]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub position: String,
}

impl Player {
    pub fn from_with_id(new_player: NewPlayer, id: i32) -> Self {
        Self {
            id: id,
            name: new_player.name,
            age: new_player.age,
            position: new_player.position,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "new player")]
pub struct NewPlayer {
    pub name: String,
    pub age: i32,
    pub position: String,
}
