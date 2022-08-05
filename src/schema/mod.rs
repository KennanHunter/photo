mod mutation;
mod query;

use juniper::{EmptySubscription, FieldResult, GraphQLObject, RootNode};

use mutation::Mutation;
use query::Query;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}
