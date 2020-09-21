use diesel::prelude::*;
use juniper::FieldResult;

use crate::database::model::User;
use crate::database::schema::pitkour_users::dsl::pitkour_users;
use crate::database::schema::pitkour_users::experience as user_experience;
use crate::database::schema::pitkour_users::firstNick as user_first_nick;
use crate::database::schema::pitkour_users::level as user_level;
use crate::database::schema::pitkour_users::nick as user_nick;
use crate::database::schema::pitkour_users::uuid as user_uuid;
use crate::graphql::context::Context;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateUserInput {
    uuid: String,
    first_nick: String,
    nick: String,
    experience: i32,
    level: i32,
}

#[derive(juniper::GraphQLObject)]
pub struct CreateUserPayload {
    affected_rows: i32,
}

impl CreateUserPayload {
    fn new(affected_rows: i32) -> Self {
        Self { affected_rows }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateUserInput {
    uuid: String,
    first_nick: Option<String>,
    nick: Option<String>,
    experience: Option<i32>,
    level: Option<i32>,
}

#[derive(juniper::GraphQLObject)]
pub struct UpdateUserPayload {
    affected_rows: i32,
}

impl UpdateUserPayload {
    fn new(affected_rows: i32) -> Self {
        Self { affected_rows }
    }
}

pub struct UserMutation;

impl UserMutation {
    pub fn create_user(
        context: &Context,
        input: CreateUserInput,
    ) -> FieldResult<CreateUserPayload> {
        let connection = context.connection()?;
        let affected_rows = diesel::insert_into(pitkour_users)
            .values((
                user_uuid.eq(&input.uuid),
                user_first_nick.eq(&input.first_nick),
                user_nick.eq(&input.nick),
                user_experience.eq(&input.experience),
                user_level.eq(&input.level),
            ))
            .execute(&connection)?;
        let payload = CreateUserPayload::new(affected_rows as i32);
        Ok(payload)
    }

    pub fn update_user(
        context: &Context,
        input: UpdateUserInput,
    ) -> FieldResult<UpdateUserPayload> {
        let connection = context.connection()?;
        let uuid = &input.uuid;
        let found = pitkour_users.find(uuid);
        let user: User = found.first(&connection)?;
        let first_nick = input.first_nick.as_deref().unwrap_or(&user.first_nick);
        let nick = input.nick.as_deref().unwrap_or(&user.nick);
        let experience = input.experience.unwrap_or(user.experience);
        let level = input.level.unwrap_or(user.level);
        let affected_rows = diesel::update(found)
            .set((
                user_first_nick.eq(first_nick),
                user_nick.eq(nick),
                user_experience.eq(experience),
                user_level.eq(level),
            ))
            .execute(&connection)?;
        let payload = UpdateUserPayload::new(affected_rows as i32);
        Ok(payload)
    }
}
