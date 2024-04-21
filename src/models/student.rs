use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub registration: Option<String>,
    pub grades: Option<String>,
}
