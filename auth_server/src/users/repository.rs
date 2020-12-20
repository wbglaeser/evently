#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use crate::schema::users;
use crate::users::{ User, LoginUser, RegisterUser };
use diesel::result::Error;

pub fn retrieve_user(user: LoginUser, connection: &PgConnection) -> QueryResult<User> {
    match users::table.filter(
        users::email.eq(&user.email)
    ).filter(
        users::password.eq(&user.password)
    ).get_results::<User>(&*connection) {
        Ok(user) => Ok(user[0].clone()),
        Err(e) => return Err(Error::NotFound),
    }
}

pub fn insert(user: RegisterUser, connection: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(connection)
}
