use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::graphql::context::Context;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::FieldResult;

use crate::database::schema::pitkour_teams::name as team_name;

pub struct Mutation;

impl Default for Mutation {
    fn default() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn set_team_name(context: &Context, tag: String, name: String) -> FieldResult<i32> {
        let pool = context.database_pool();
        let connection = pool.get()?;
        let affected_rows = diesel::update(pitkour_teams.find(tag))
            .set(team_name.eq(name))
            .execute(&connection)?;
        Ok(affected_rows as i32)
    }
}
