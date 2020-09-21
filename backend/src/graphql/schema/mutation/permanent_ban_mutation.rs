use diesel::prelude::*;
use juniper::FieldResult;

use crate::database::model::PermanentBan;
use crate::database::schema::pitkour_permamentbans::createTime as permanent_ban_create_time;
use crate::database::schema::pitkour_permamentbans::dsl::pitkour_permamentbans as pitkour_permanent_bans;
use crate::database::schema::pitkour_permamentbans::nick as permanent_ban_nick;
use crate::database::schema::pitkour_permamentbans::performer as permanent_ban_performer;
use crate::database::schema::pitkour_permamentbans::reason as permanent_ban_reason;
use crate::database::schema::pitkour_permamentbans::uuid as permanent_ban_uuid;
use crate::graphql::context::Context;

#[derive(juniper::GraphQLInputObject)]
pub struct CreatePermanentBanInput {
    uuid: String,
    nick: String,
    reason: String,
    performer: String,
    create_time: f64,
}

#[derive(juniper::GraphQLObject)]
pub struct CreatePermanentBanPayload {
    affected_rows: i32,
}

impl CreatePermanentBanPayload {
    fn new(affected_rows: i32) -> Self {
        Self { affected_rows }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdatePermanentBanInput {
    uuid: String,
    nick: Option<String>,
    reason: Option<String>,
    performer: Option<String>,
    create_time: Option<f64>,
}

#[derive(juniper::GraphQLObject)]
pub struct UpdatePermanentBanPayload {
    affected_rows: i32,
}

impl UpdatePermanentBanPayload {
    fn new(affected_rows: i32) -> Self {
        Self { affected_rows }
    }
}

pub struct PermanentBanMutation;

impl PermanentBanMutation {
    pub fn create_permanent_ban(
        context: &Context,
        input: CreatePermanentBanInput,
    ) -> FieldResult<CreatePermanentBanPayload> {
        let connection = context.connection()?;
        let affected_rows = diesel::insert_into(pitkour_permanent_bans)
            .values((
                permanent_ban_uuid.eq(&input.uuid),
                permanent_ban_nick.eq(&input.nick),
                permanent_ban_reason.eq(&input.reason),
                permanent_ban_performer.eq(&input.performer),
                permanent_ban_create_time.eq(&input.create_time),
            ))
            .execute(&connection)?;
        let payload = CreatePermanentBanPayload::new(affected_rows as i32);
        Ok(payload)
    }

    pub fn update_permanent_ban(
        context: &Context,
        input: UpdatePermanentBanInput,
    ) -> FieldResult<UpdatePermanentBanPayload> {
        let connection = context.connection()?;
        let uuid = &input.uuid;
        let found = pitkour_permanent_bans.find(uuid);
        let permanent_ban: PermanentBan = found.first(&connection)?;
        let nick = input.nick.as_deref().unwrap_or(&permanent_ban.nick);
        let reason = input.reason.as_deref().unwrap_or(&permanent_ban.reason);
        let performer = input
            .performer
            .as_deref()
            .unwrap_or(&permanent_ban.performer);
        let create_time = input.create_time.unwrap_or(permanent_ban.create_time);
        let affected_rows = diesel::update(found)
            .set((
                permanent_ban_nick.eq(nick),
                permanent_ban_reason.eq(reason),
                permanent_ban_performer.eq(performer),
                permanent_ban_create_time.eq(create_time),
            ))
            .execute(&connection)?;
        let payload = UpdatePermanentBanPayload::new(affected_rows as i32);
        Ok(payload)
    }
}
