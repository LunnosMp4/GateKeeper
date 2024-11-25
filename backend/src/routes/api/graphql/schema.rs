use async_graphql::{Schema, Object, Context, Result};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, _ctx: &Context<'_>) -> &str {
        "Hello, world!"
    }
}

pub type MySchema = Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription).finish()
}