pub mod team_mutation;

use crate::graphql::context::Context;
use crate::graphql::schema::mutation::team_mutation::{
    CreateTeamInput, CreateTeamPayload, TeamMutation, UpdateTeamInput, UpdateTeamPayload,
};
use juniper::FieldResult;

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_team(context: &Context, input: CreateTeamInput) -> FieldResult<CreateTeamPayload> {
        TeamMutation::create_team(context, input)
    }

    fn update_team(context: &Context, input: UpdateTeamInput) -> FieldResult<UpdateTeamPayload> {
        TeamMutation::update_team(context, input)
    }
}
