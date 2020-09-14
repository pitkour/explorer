pub mod team_mutation;

use crate::graphql::context::Context;
use crate::graphql::schema::mutation::team_mutation::TeamMutation;

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn team(&self) -> TeamMutation {
        TeamMutation
    }
}
