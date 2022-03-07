use serde::{Serialize};
use diesel::Queryable;
use diesel::Insertable;
#[derive(Queryable, Serialize)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub age: String,
    pub gender: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub age: &'a str,
    pub gender: &'a str,
}