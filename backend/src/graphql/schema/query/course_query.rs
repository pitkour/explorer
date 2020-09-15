use crate::database::model::{Course, Quest};
use crate::database::schema::pitkour_courses::dsl::pitkour_courses;
use crate::database::schema::pitkour_courses::name as course_name;
use crate::database::schema::pitkour_quests::courseID as quest_course_id;
use crate::database::schema::pitkour_quests::dsl::pitkour_quests;
use crate::graphql::context::Context;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct CourseQuery;

impl CourseQuery {
    pub fn courses(context: &Context, first: Option<i32>) -> FieldResult<Vec<Course>> {
        let connection = context.connection()?;
        let courses = match first {
            Some(first) => pitkour_courses
                .limit(first as i64)
                .load::<Course>(&connection)?,
            None => pitkour_courses.load::<Course>(&connection)?,
        };
        Ok(courses)
    }

    pub fn course(
        context: &Context,
        course_id: Option<i32>,
        name: Option<String>,
    ) -> FieldResult<Option<Course>> {
        if let Some(course_id) = course_id {
            let connection = context.connection()?;
            let course: Option<Course> = pitkour_courses
                .find(course_id)
                .first(&connection)
                .optional()?;
            Ok(course)
        } else if let Some(name) = name {
            let connection = context.connection()?;
            let user: Option<Course> = pitkour_courses
                .filter(course_name.eq(name))
                .first(&connection)
                .optional()?;
            Ok(user)
        } else {
            Err(FieldError::from(
                "Missing a 'courseId' or 'name' of a course.",
            ))
        }
    }
}

#[juniper::object(Context = Context)]
impl Course {
    pub fn quest(&self, context: &Context) -> FieldResult<Option<Quest>> {
        let connection = context.connection()?;
        let quest = pitkour_quests
            .filter(quest_course_id.eq(&self.course_id))
            .first(&connection)
            .optional()?;
        Ok(quest)
    }

    pub fn course_id(&self) -> i32 {
        self.course_id
    }

    pub fn experience(&self) -> i32 {
        self.experience
    }

    pub fn required_level(&self) -> i32 {
        self.required_level
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start(&self) -> &str {
        &self.start
    }

    pub fn category(&self) -> &str {
        &self.category
    }
}
