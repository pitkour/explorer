use crate::DatabasePool;

pub struct Context {
    database_pool: DatabasePool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(database_pool: DatabasePool) -> Self {
        Context { database_pool }
    }

    pub fn database_pool(&self) -> &DatabasePool {
        &self.database_pool
    }
}
