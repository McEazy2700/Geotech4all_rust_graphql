use async_graphql::*;

use super::users::mutations::UsersMutations;


pub struct Query;

#[Object]
impl Query {
    async fn version(&self) -> String {
        String::from("0.0.1")
    }
}

#[derive(MergedObject, Default)]
pub struct Mutation(UsersMutations);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::new(Query, Mutation::default(), EmptySubscription)
}
