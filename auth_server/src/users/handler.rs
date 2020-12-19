use crate::connection::DbConn;
use diesel::result::Error;
use crate::users;
use users::User;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::response::status;
use std::env;
use rocket::http::{Cookies, Cookie};
use users::InsertableUser;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

#[post("/login", data = "<user>")]
pub fn login(user: Json<InsertableUser>, connection: DbConn, mut cookies: Cookies) -> Result<Json<Vec<User>>, Status> {
    users::repository::retrieve_user(user.into_inner(), &connection)
        .map(|user| {
            let mut header = Header::new(Algorithm::HS512);
            cookies.add_private(
                Cookie::new("jwt",encode(&header, &user, &EncodingKey::from_secret("secret".as_ref())).expect("error"))
            );
            Json(user)
        })
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
