use crate::database::model::{Team, TeamMember};
use crate::database::schema::pitkour_teams::columns::tag;
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::database::schema::pitkour_teams_members::dsl::pitkour_teams_members;
use crate::graphql::context::Context;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::FieldResult;

pub struct Query;

impl Default for Query {
    fn default() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Context)]
impl Query {
    fn teams(context: &Context) -> FieldResult<Vec<Team>> {
        let pool = context.database_pool();
        let connection = pool.get()?;
        let teams = pitkour_teams.load::<Team>(&connection)?;
        Ok(teams)
    }

    fn get_team_by_tag(context: &Context, team_tag: String) -> FieldResult<Option<Team>> {
        let pool = context.database_pool();
        let connection = pool.get()?;
        let mut found: Vec<Team> = pitkour_teams
            .filter(tag.eq(team_tag))
            .load::<Team>(&connection)?;
        Ok(if found.is_empty() {
            None
        } else {
            Some(found.remove(0))
        })
    }

    fn teams_members(context: &Context) -> FieldResult<Vec<TeamMember>> {
        let pool = context.database_pool();
        let connection = pool.get()?;
        let teams_members = pitkour_teams_members.load::<TeamMember>(&connection)?;
        Ok(teams_members)
    }
}
