use crate::{Connection, DatabasePool};
use anyhow::Result;

pub struct Context {
    database_pool: DatabasePool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(database_pool: DatabasePool) -> Self {
        Context { database_pool }
    }

    pub fn connection(&self) -> Result<Connection> {
        let connection = self.database_pool.get()?;
        Ok(connection)
    }
}
