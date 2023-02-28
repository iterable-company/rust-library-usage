use derive_new::new;

#[derive(new, Debug, Copy, PartialEq, Eq, Hash, Clone, async_graphql::Enum)]
pub enum SportsKind {
    Football,
    Baseball,
}

#[derive(new, Debug, PartialEq, Eq, Hash, Clone, async_graphql::SimpleObject)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub kind: SportsKind,
}

#[derive(new, Debug, Clone, async_graphql::SimpleObject)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub age: u32,
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

#[derive(new, Clone, async_graphql::InputObject)]
pub struct NewPlayer {
    pub name: String,
    pub age: u32,
    pub position: String,
}
