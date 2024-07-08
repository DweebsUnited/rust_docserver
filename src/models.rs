use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Doc {
    pub id: i32,
    pub name: String,
    pub storagename: String,
    pub date: String,
    pub lastupdate: String,

    pub tags: Vec<Tag>,
    pub types: Vec<Type>,
    pub people: Vec<Person>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct DocDB {
    pub id: i32,
    pub name: String,
    pub storagename: String,
    pub date: String,
    pub lastupdate: String
}

#[derive(Deserialize)]
pub struct DocAPI {
    pub id: Option<i32>,
    pub name: String,
    pub date: String,

    pub tags: Vec<i32>,
    pub types: Vec<i32>,
    pub people: Vec<i32>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Type {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub color: String,
}