use juniper::FieldResult;

use crate::graphql::context::Context;
use crate::graphql::schema::mutation::permanent_ban_mutation::{
    CreatePermanentBanInput, CreatePermanentBanPayload, PermanentBanMutation,
    UpdatePermanentBanInput, UpdatePermanentBanPayload,
};
use crate::graphql::schema::mutation::team_mutation::{
    CreateTeamInput, CreateTeamPayload, TeamMutation, UpdateTeamInput, UpdateTeamPayload,
};
use crate::graphql::schema::mutation::user_mutation::{
    CreateUserInput, CreateUserPayload, UpdateUserInput, UpdateUserPayload, UserMutation,
};

pub mod permanent_ban_mutation;
pub mod team_mutation;
pub mod user_mutation;

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_user(context: &Context, input: CreateUserInput) -> FieldResult<CreateUserPayload> {
        UserMutation::create_user(context, input)
    }

    fn update_user(context: &Context, input: UpdateUserInput) -> FieldResult<UpdateUserPayload> {
        UserMutation::update_user(context, input)
    }

    fn create_team(context: &Context, input: CreateTeamInput) -> FieldResult<CreateTeamPayload> {
        TeamMutation::create_team(context, input)
    }

    fn update_team(context: &Context, input: UpdateTeamInput) -> FieldResult<UpdateTeamPayload> {
        TeamMutation::update_team(context, input)
    }

    fn create_permanent_ban(
        context: &Context,
        input: CreatePermanentBanInput,
    ) -> FieldResult<CreatePermanentBanPayload> {
        PermanentBanMutation::create_permanent_ban(context, input)
    }

    fn update_permanent_ban(
        context: &Context,
        input: UpdatePermanentBanInput,
    ) -> FieldResult<UpdatePermanentBanPayload> {
        PermanentBanMutation::update_permanent_ban(context, input)
    }
}
