#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use juniper::{EmptyMutation, RootNode};
use rocket::response::content;
use rocket::State;

use crate::models::Team;
use crate::schema::pitkour_teams::dsl::pitkour_teams;

mod database;
mod graphql;

type Schema = RootNode<'static, Team, EmptyMutation<Database>, EmptySubscription<Database>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

fn main() {
    let connection = establish_connection();
    let teams = pitkour_teams
        .load::<Team>(&connection)
        .expect("Error loading posts");
    println!("Displaying {} teams", teams.len());
    for team in teams {
        println!("{:?}", team);
    }
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(
            Team,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}

fn establish_connection() -> SqliteConnection {
    let database_url = "pitkour.db";
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
