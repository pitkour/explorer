use diesel::prelude::*;
use juniper::FieldResult;

use crate::database::model::{Team, TeamMember};
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::database::schema::pitkour_teams_members::dsl::pitkour_teams_members;
use crate::database::schema::pitkour_teams_members::tag as member_team_tag;
use crate::graphql::context::Context;

#[juniper::object(Context = Context)]
impl Team {
    fn tag(&self) -> &str {
        &self.tag
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn creator(&self) -> &str {
        &self.creator
    }

    fn create_time(&self) -> f64 {
        self.create_time
    }

    fn coins(&self) -> i32 {
        self.coins
    }

    fn members(&self, context: &Context, first: Option<i32>) -> FieldResult<Vec<TeamMember>> {
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

    fn member(&self, context: &Context, uuid: String) -> FieldResult<Option<TeamMember>> {
        let connection = context.connection()?;
        let member = pitkour_teams_members
            .find(uuid)
            .first(&connection)
            .optional()?;
        Ok(member)
    }
}

#[juniper::object(Context = Context)]
impl TeamMember {
    fn tag(&self) -> &str {
        &self.tag
    }

    fn uuid(&self) -> &str {
        &self.uuid
    }

    fn rank(&self) -> &str {
        &self.rank
    }

    fn join_time(&self) -> f64 {
        self.join_time
    }

    fn coins_paid(&self) -> i32 {
        self.coins_paid
    }

    fn team(&self, context: &Context) -> FieldResult<Team> {
        let connection = context.connection()?;
        let team = pitkour_teams.find(&self.tag).first(&connection)?;
        Ok(team)
    }
}
