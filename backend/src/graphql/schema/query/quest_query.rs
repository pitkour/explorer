use crate::database::model::{Course, Quest};
use crate::database::schema::pitkour_courses::dsl::pitkour_courses;
use crate::graphql::context::Context;
use diesel::prelude::*;
use juniper::FieldResult;

#[juniper::object(Context = Context)]
impl Quest {
    pub fn course(&self, context: &Context) -> FieldResult<Course> {
        let connection = context.connection()?;
        let course: Course = pitkour_courses.find(&self.course_id).first(&connection)?;
        Ok(course)
    }

    pub fn quest_id(&self) -> i32 {
        self.quest_id
    }

    pub fn course_id(&self) -> i32 {
        self.course_id
    }

    pub fn npc_uuid(&self) -> &str {
        &self.npc_uuid
    }

    pub fn npc_nick(&self) -> &str {
        &self.npc_nick
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn normal_dialog(&self) -> &str {
        &self.normal_dialog
    }

    pub fn tip_dialog(&self) -> &str {
        &self.tip_dialog
    }

    pub fn start_dialog(&self) -> &str {
        &self.start_dialog
    }

    pub fn end_dialog(&self) -> &str {
        &self.end_dialog
    }
}
