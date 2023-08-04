use graphql_client::GraphQLQuery;
use juniper::RootNode;
use tokio::time::Duration;
use juniper::GraphQLObject;

// Required for graphql_client to match
// Hasura uuid type in schema.json.
#[allow(non_camel_case_types)]
pub type uuid = uuid_::Uuid;

pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }
}

pub struct Mutation;

#[juniper::graphql_object]
impl Mutation {
    fn noop() -> bool {
        true
    }
}

// Now, the `RootNode` creation will be valid, as `Query` and `Mutation` types both implement `GraphQLType`.
pub type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription<crate::types::Context>>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user-by-username.graphql"
)]
pub struct UserByUsername;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/user-create.graphql"
)]
pub struct UserCreate;
