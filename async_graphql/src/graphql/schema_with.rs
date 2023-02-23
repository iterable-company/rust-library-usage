use indexmap::IndexMap;

use crate::domain::lib::{Player, Team};

use super::{mutation, query};

pub type Schema =
    async_graphql::Schema<query::Query, mutation::Mutation, async_graphql::EmptySubscription>;

pub fn schema_with(data: &'static IndexMap<Team, Vec<Player>>) -> Schema {
    Schema::build(
        query::Query(data),
        mutation::Mutation(data),
        async_graphql::EmptySubscription,
    )
    .disable_introspection()
    .finish()
}
