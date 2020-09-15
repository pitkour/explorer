use crate::graphql::context::Context;
use crate::Schema;
use juniper_rocket::GraphQLRequest;
use rocket::response::content::Html;
use rocket::State;

#[get("/")]
pub fn root_handler() -> &'static str {
    "Hello, world!"
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    request: GraphQLRequest,
    schema: State<Schema>,
    context: State<Context>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    request: GraphQLRequest,
    schema: State<Schema>,
    context: State<Context>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[get("/graphiql")]
pub fn graphiql_handler() -> Html<String> {
    juniper_rocket::graphiql_source("http://api.local.test:8000/graphql")
}

#[get("/playground")]
pub fn playground_handler() -> Html<String> {
    juniper_rocket::playground_source("http://api.local.test:8000/graphql")
}
