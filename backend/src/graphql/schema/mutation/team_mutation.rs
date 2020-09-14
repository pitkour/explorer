use crate::database::model::Team;
use crate::database::schema::pitkour_teams::coins as team_coins;
use crate::database::schema::pitkour_teams::createTime as team_create_time;
use crate::database::schema::pitkour_teams::creator as team_creator;
use crate::database::schema::pitkour_teams::dsl::pitkour_teams;
use crate::database::schema::pitkour_teams::name as team_name;
use crate::database::schema::pitkour_teams::tag as team_tag;
use crate::graphql::context::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use std::time;
use std::time::SystemTime;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateTeamInput {
    tag: String,
    name: String,
    creator: String,
}

#[derive(juniper::GraphQLObject)]
pub struct CreateTeamPayload {
    affected_rows: i32,
}

impl CreateTeamPayload {
    fn new(affected_rows: i32) -> Self {
        Self { affected_rows }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateTeamInput {
    tag: String,
    name: Option<String>,
    creator: Option<String>,
    create_time: Option<f64>,
    coins: Option<i32>,
}

#[derive(juniper::GraphQLObject)]
pub struct UpdateTeamPayload {
    affected_rows: i32,
}

impl UpdateTeamPayload {
    fn new(affected_rows: i32) -> Self {
        Self { affected_rows }
    }
}

pub struct TeamMutation;

#[juniper::object(Context = Context)]
impl TeamMutation {
    pub fn create_team(
        context: &Context,
        input: CreateTeamInput,
    ) -> FieldResult<CreateTeamPayload> {
        let create_time = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_millis() as f64;
        let coins = 0;
        let connection = context.connection()?;
        let affected_rows = diesel::insert_into(pitkour_teams)
            .values((
                team_tag.eq(&input.tag),
                team_name.eq(&input.name),
                team_creator.eq(&input.creator),
                team_create_time.eq(create_time),
                team_coins.eq(coins),
            ))
            .execute(&connection)?;
        let payload = CreateTeamPayload::new(affected_rows as i32);
        Ok(payload)
    }

    pub fn update_team(
        context: &Context,
        input: UpdateTeamInput,
    ) -> FieldResult<UpdateTeamPayload> {
        let connection = context.connection()?;
        let tag = &input.tag;
        let found = pitkour_teams.find(tag);
        let team: Team = found.first(&connection)?;
        let (name, creator, create_time, coins) = get_update_team_values(&input, &team);
        let affected_rows = diesel::update(found)
            .set((
                team_name.eq(name),
                team_creator.eq(creator),
                team_create_time.eq(create_time),
                team_coins.eq(coins),
            ))
            .execute(&connection)?;
        let payload = UpdateTeamPayload::new(affected_rows as i32);
        Ok(payload)
    }
}

fn get_update_team_values<'a>(
    input: &'a UpdateTeamInput,
    team: &'a Team,
) -> (&'a String, &'a String, &'a f64, &'a i32) {
    let name = match &input.name {
        Some(name) => name,
        _ => &team.name,
    };
    let creator = match &input.creator {
        Some(creator) => creator,
        _ => &team.creator,
    };
    let create_time = match &input.create_time {
        Some(create_time) => create_time,
        _ => &team.create_time,
    };
    let coins = match &input.coins {
        Some(coins) => coins,
        _ => &team.coins,
    };
    (name, creator, create_time, coins)
}
