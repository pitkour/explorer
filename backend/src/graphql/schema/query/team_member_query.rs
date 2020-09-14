use crate::database::model::{Team, TeamMember, User};
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::database::schema::pitkour_users::dsl::pitkour_users;
use crate::graphql::context::Context;
use diesel::prelude::*;
use juniper::FieldResult;

#[juniper::object(Context = Context)]
impl TeamMember {
    pub fn team(&self, context: &Context) -> FieldResult<Team> {
        let connection = context.connection()?;
        let team: Team = pitkour_teams.find(&self.tag).first(&connection)?;
        Ok(team)
    }

    pub fn user(&self, context: &Context) -> FieldResult<User> {
        let connection = context.connection()?;
        let user: User = pitkour_users.find(&self.uuid).first(&connection)?;
        Ok(user)
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn rank(&self) -> &str {
        &self.rank
    }

    pub fn join_time(&self) -> f64 {
        self.join_time
    }

    pub fn coins_paid(&self) -> i32 {
        self.coins_paid
    }
}
