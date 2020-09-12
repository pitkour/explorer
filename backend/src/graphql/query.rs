use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use juniper::{FieldError, FieldResult};

use crate::database::model::Team;
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::database::schema::pitkour_teams::name as team_name;
use crate::graphql::context::Context;

pub struct Query;

impl Default for Query {
    fn default() -> Self {
        Self {}
    }
}

#[juniper::object(Context = Context)]
impl Query {
    fn teams(context: &Context, first: Option<i32>) -> FieldResult<Vec<Team>> {
        let connection = context.connection()?;
        let teams = if let Some(first) = first {
            pitkour_teams
                .limit(first as i64)
                .load::<Team>(&connection)?
        } else {
            pitkour_teams.load::<Team>(&connection)?
        };
        Ok(teams)
    }

    fn team(
        context: &Context,
        tag: Option<String>,
        name: Option<String>,
    ) -> FieldResult<Option<Team>> {
        if let Some(tag) = tag {
            let connection = context.connection()?;
            let team = pitkour_teams.find(tag).first(&connection).optional()?;
            Ok(team)
        } else if let Some(name) = name {
            let connection = context.connection()?;
            let team = pitkour_teams
                .filter(team_name.eq(name))
                .first(&connection)
                .optional()?;
            Ok(team)
        } else {
            Err(FieldError::from("Missing a 'tag' or 'name' of a team."))
        }
    }
}
