use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::database::model::{Team, TeamMember};
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::database::schema::pitkour_teams::name as team_name;
use crate::database::schema::pitkour_teams::tag as team_tag;
use crate::database::schema::pitkour_teams_members::dsl::pitkour_teams_members;
use crate::database::schema::pitkour_teams_members::tag as member_team_tag;
use crate::graphql::context::Context;

pub struct TeamQuery;

impl TeamQuery {
    pub fn teams(
        context: &Context,
        first: Option<i32>,
        search_query: Option<String>,
    ) -> FieldResult<Vec<Team>> {
        let connection = context.connection()?;
        let teams = match first {
            Some(first) => match search_query {
                Some(search_query) => pitkour_teams
                    .filter(team_tag.like(format!("{}%", search_query)))
                    .or_filter(team_name.like(format!("{}%", search_query)))
                    .limit(first as i64)
                    .load::<Team>(&connection)?,
                None => pitkour_teams
                    .limit(first as i64)
                    .load::<Team>(&connection)?,
            },
            None => match search_query {
                Some(search_query) => pitkour_teams
                    .filter(team_tag.like(format!("{}%", search_query)))
                    .or_filter(team_name.like(format!("{}%", search_query)))
                    .load::<Team>(&connection)?,
                None => pitkour_teams.load::<Team>(&connection)?,
            },
        };
        Ok(teams)
    }

    pub fn team(
        context: &Context,
        tag: Option<String>,
        name: Option<String>,
    ) -> FieldResult<Option<Team>> {
        if let Some(tag) = tag {
            let connection = context.connection()?;
            let team: Option<Team> = pitkour_teams
                .find(tag)
                .first::<Team>(&connection)
                .optional()?;
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

#[juniper::object(Context = Context)]
impl Team {
    pub fn members_count(&self, context: &Context) -> FieldResult<i32> {
        let connection = context.connection()?;
        let count: i64 = pitkour_teams_members
            .filter(member_team_tag.eq(&self.tag))
            .count()
            .first(&connection)?;
        Ok(count as i32)
    }

    pub fn members(&self, context: &Context, first: Option<i32>) -> FieldResult<Vec<TeamMember>> {
        let connection = context.connection()?;
        let members = if let Some(first) = first {
            pitkour_teams_members
                .limit(first as i64)
                .filter(member_team_tag.eq(&self.tag))
                .load::<TeamMember>(&connection)?
        } else {
            pitkour_teams_members
                .filter(member_team_tag.eq(&self.tag))
                .load::<TeamMember>(&connection)?
        };
        Ok(members)
    }

    pub fn member(&self, context: &Context, uuid: String) -> FieldResult<Option<TeamMember>> {
        let connection = context.connection()?;
        let member = pitkour_teams_members
            .find(uuid)
            .first(&connection)
            .optional()?;
        Ok(member)
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn creator(&self) -> &str {
        &self.creator
    }

    pub fn create_time(&self) -> f64 {
        self.create_time
    }

    pub fn coins(&self) -> i32 {
        self.coins
    }
}
