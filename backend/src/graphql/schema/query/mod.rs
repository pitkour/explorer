pub mod course_query;
pub mod permanent_ban_query;
pub mod quest_query;
pub mod team_member_query;
pub mod team_query;
pub mod user_query;

use crate::graphql::context::Context;
use crate::graphql::schema::query::course_query::CourseQuery;
use crate::graphql::schema::query::permanent_ban_query::PermanentBanQuery;
use crate::graphql::schema::query::team_query::TeamQuery;
use crate::graphql::schema::query::user_query::UserQuery;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn course() -> CourseQuery {
        CourseQuery
    }

    fn team() -> TeamQuery {
        TeamQuery
    }

    fn user() -> UserQuery {
        UserQuery
    }

    fn permanent_ban() -> PermanentBanQuery {
        PermanentBanQuery
    }
}
