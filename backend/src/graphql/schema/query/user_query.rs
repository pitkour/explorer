use crate::database::model::{TeamMember, User};
use crate::database::schema::pitkour_teams_members::dsl::pitkour_teams_members;
use crate::database::schema::pitkour_users::dsl::pitkour_users;
use crate::database::schema::pitkour_users::nick as user_nick;
use crate::graphql::context::Context;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct UserQuery;

impl UserQuery {
    pub fn users(context: &Context, first: Option<i32>) -> FieldResult<Vec<User>> {
        let connection = context.connection()?;
        let users = if let Some(first) = first {
            pitkour_users
                .limit(first as i64)
                .load::<User>(&connection)?
        } else {
            pitkour_users.load::<User>(&connection)?
        };
        Ok(users)
    }

    pub fn user(
        context: &Context,
        uuid: Option<String>,
        nick: Option<String>,
    ) -> FieldResult<Option<User>> {
        if let Some(uuid) = uuid {
            let connection = context.connection()?;
            let user: Option<User> = pitkour_users.find(uuid).first(&connection).optional()?;
            Ok(user)
        } else if let Some(nick) = nick {
            let connection = context.connection()?;
            let user: Option<User> = pitkour_users
                .filter(user_nick.eq(nick))
                .first(&connection)
                .optional()?;
            Ok(user)
        } else {
            Err(FieldError::from("Missing a 'uuid' or 'nick' of a user."))
        }
    }
}

#[juniper::object(Context = Context)]
impl User {
    pub fn team_member(&self, context: &Context) -> FieldResult<Option<TeamMember>> {
        let connection = context.connection()?;
        let member = pitkour_teams_members
            .find(&self.uuid)
            .first(&connection)
            .optional()?;
        Ok(member)
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn first_nick(&self) -> &str {
        &self.first_nick
    }

    pub fn nick(&self) -> &str {
        &self.nick
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn permission_group(&self) -> &str {
        &self.permission_group
    }

    pub fn experience(&self) -> i32 {
        self.experience
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn quest_progress(&self) -> i32 {
        self.quest_progress
    }

    pub fn coins(&self) -> i32 {
        self.coins
    }

    pub fn spent_coins(&self) -> i32 {
        self.spent_coins
    }

    pub fn chests(&self) -> i32 {
        self.chests
    }

    pub fn opened_chests(&self) -> i32 {
        self.opened_chests
    }

    pub fn won_duels(&self) -> i32 {
        self.won_duels
    }

    pub fn lost_duels(&self) -> i32 {
        self.lost_duels
    }

    pub fn first_join_time(&self) -> f64 {
        self.first_join_time
    }

    pub fn last_join_time(&self) -> f64 {
        self.last_join_time
    }

    pub fn last_quit_time(&self) -> f64 {
        self.last_quit_time
    }

    pub fn play_time(&self) -> f64 {
        self.play_time
    }
}
