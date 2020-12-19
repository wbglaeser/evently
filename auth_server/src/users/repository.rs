#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use crate::schema::users;
use crate::users::{ User, InsertableUser };
use diesel::result::Error;

pub fn retrieve_user(user: InsertableUser, connection: &PgConnection) -> QueryResult<Vec<User>> {
    match users::table.filter(
        users::email.eq(&user.email)
    ).filter(
        users::password.eq(&user.password)
    ).get_results::<User>(&*connection) {
        Ok(user) => Ok(user.clone()),
        Err(e) => return Err(Error::NotFound),
    }
}
