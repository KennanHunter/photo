pub mod mutation;
pub mod query;

use juniper::{EmptySubscription, RootNode};

use mutation::Mutation;
use query::Query;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}
