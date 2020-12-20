#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::users;

pub mod handler;
pub mod repository;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct User {
    id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct LoginUser {
    pub email: String,
    pub password: String
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String
}
