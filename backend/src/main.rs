#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod database;
mod graphql;
mod handlers;

use crate::graphql::context::Context;
use crate::graphql::schema::mutation::Mutation;
use crate::graphql::schema::query::Query;
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use juniper::RootNode;

type Schema = RootNode<'static, Query, Mutation>;
type DatabasePool = Pool<ConnectionManager<SqliteConnection>>;
type Connection = PooledConnection<ConnectionManager<SqliteConnection>>;

fn main() -> Result<()> {
    let pool = create_pool()?;
    let context = Context::new(pool);
    let schema = Schema::new(Query, Mutation);
    let root_routes = rocket::routes![
        handlers::root_handler,
        handlers::get_graphql_handler,
        handlers::post_graphql_handler,
        handlers::graphiql_handler,
        handlers::playground_handler
    ];
    rocket::ignite()
        .manage(context)
        .manage(schema)
        .mount("/", root_routes)
        .launch();
    Ok(())
}

const DATABASE_URL: &str = "pitkour.db";

fn create_pool() -> Result<DatabasePool> {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    let pool = Pool::builder().max_size(3).build(manager)?;
    Ok(pool)
}
