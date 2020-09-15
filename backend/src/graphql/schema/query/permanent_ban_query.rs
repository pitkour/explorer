use crate::database::model::{PermanentBan, User};
use crate::database::schema::pitkour_permamentbans::dsl::pitkour_permamentbans as pitkour_permanent_bans;
use crate::database::schema::pitkour_permamentbans::nick as permanent_ban_nick;
use crate::database::schema::pitkour_users::dsl::pitkour_users;
use crate::graphql::context::Context;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct PermanentBanQuery;

impl PermanentBanQuery {
    pub fn permanent_bans(context: &Context, first: Option<i32>) -> FieldResult<Vec<PermanentBan>> {
        let connection = context.connection()?;
        let permanent_bans = if let Some(first) = first {
            pitkour_permanent_bans
                .limit(first as i64)
                .load::<PermanentBan>(&connection)?
        } else {
            pitkour_permanent_bans.load::<PermanentBan>(&connection)?
        };
        Ok(permanent_bans)
    }

    pub fn permanent_ban(
        context: &Context,
        uuid: Option<String>,
        nick: Option<String>,
    ) -> FieldResult<Option<PermanentBan>> {
        if let Some(uuid) = uuid {
            let connection = context.connection()?;
            let permanent_ban: Option<PermanentBan> = pitkour_permanent_bans
                .find(uuid)
                .first(&connection)
                .optional()?;
            Ok(permanent_ban)
        } else if let Some(nick) = nick {
            let connection = context.connection()?;
            let permanent_ban: Option<PermanentBan> = pitkour_permanent_bans
                .filter(permanent_ban_nick.eq(nick))
                .first(&connection)
                .optional()?;
            Ok(permanent_ban)
        } else {
            Err(FieldError::from(
                "Missing a 'uuid' or 'nick' of a permanent ban.",
            ))
        }
    }
}

#[juniper::object(Context = Context)]
impl PermanentBan {
    pub fn user(&self, context: &Context) -> FieldResult<Option<User>> {
        let connection = context.connection()?;
        let user = pitkour_users
            .find(&self.uuid)
            .first(&connection)
            .optional()?;
        Ok(user)
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn nick(&self) -> &str {
        &self.nick
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn performer(&self) -> &str {
        &self.performer
    }

    pub fn create_time(&self) -> f64 {
        self.create_time
    }
}
