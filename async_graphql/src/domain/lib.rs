use derive_new::new;

#[derive(new, Copy, PartialEq, Eq, Hash, Clone, async_graphql::Enum)]
pub enum SportsKind {
    Football,
    Baseball,
}

#[derive(new, PartialEq, Eq, Hash, Clone, async_graphql::SimpleObject)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub kind: SportsKind,
}

#[derive(new, Clone, async_graphql::SimpleObject)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub age: u32,
    pub position: String,
}

#[derive(new, Clone, async_graphql::InputObject)]
pub struct NewPlayer {
    pub name: String,
    pub age: u32,
    pub position: String,
}
