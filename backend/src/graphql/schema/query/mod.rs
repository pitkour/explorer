pub mod course_query;
pub mod permanent_ban_query;
pub mod quest_query;
pub mod team_member_query;
pub mod team_query;
pub mod user_query;

use crate::database::model::{Course, PermanentBan, Team, User};
use crate::graphql::context::Context;
use crate::graphql::schema::query::course_query::CourseQuery;
use crate::graphql::schema::query::permanent_ban_query::PermanentBanQuery;
use crate::graphql::schema::query::team_query::TeamQuery;
use crate::graphql::schema::query::user_query::UserQuery;
use juniper::FieldResult;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn courses(context: &Context, first: Option<i32>) -> FieldResult<Vec<Course>> {
        CourseQuery::courses(context, first)
    }

    fn course(
        context: &Context,
        course_id: Option<i32>,
        name: Option<String>,
    ) -> FieldResult<Option<Course>> {
        CourseQuery::course(context, course_id, name)
    }

    fn teams(
        context: &Context,
        first: Option<i32>,
        search_query: Option<String>,
    ) -> FieldResult<Vec<Team>> {
        TeamQuery::teams(context, first, search_query)
    }

    fn team(
        context: &Context,
        tag: Option<String>,
        name: Option<String>,
    ) -> FieldResult<Option<Team>> {
        TeamQuery::team(context, tag, name)
    }

    fn users(
        context: &Context,
        first: Option<i32>,
        search_query: Option<String>,
    ) -> FieldResult<Vec<User>> {
        UserQuery::users(context, first, search_query)
    }

    fn user(
        context: &Context,
        uuid: Option<String>,
        nick: Option<String>,
    ) -> FieldResult<Option<User>> {
        UserQuery::user(context, uuid, nick)
    }

    fn permanent_bans(
        context: &Context,
        first: Option<i32>,
        search_query: Option<String>,
    ) -> FieldResult<Vec<PermanentBan>> {
        PermanentBanQuery::permanent_bans(context, first, search_query)
    }

    fn permanent_ban(
        context: &Context,
        uuid: Option<String>,
        nick: Option<String>,
    ) -> FieldResult<Option<PermanentBan>> {
        PermanentBanQuery::permanent_ban(context, uuid, nick)
    }
}
