use crate::database::model::Team;
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::graphql::context::Context;
use diesel::QueryDsl;
use juniper::FieldResult;

pub struct Mutation;

impl Default for Mutation {
    fn default() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn set_team_name(context: &Context, tag: String, name: String) -> FieldResult<Team> {
        let pool = context.database_pool();
        let connection = pool.get()?;
        // diesel::update(pitkour_teams.find(tag)).set();
        unimplemented!()
    }
}
