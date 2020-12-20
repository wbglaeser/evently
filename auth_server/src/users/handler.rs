use crate::connection::DbConn;
use diesel::result::Error;
use crate::users;
use users::User;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::response::status;
use std::env;
use rocket::http::{Cookies, Cookie};
use users::{ LoginUser, RegisterUser };
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

#[post("/login", data = "<user>")]
pub fn login(user: Json<LoginUser>, connection: DbConn, mut cookies: Cookies) -> Result<Json<User>, Status> {
    users::repository::retrieve_user(user.into_inner(), &connection)
        .map(|user| {
            let mut header = Header::new(Algorithm::HS512);
            let cookie = Cookie::build("jwt", encode(&header, &user, &EncodingKey::from_secret("secret".as_ref())).expect("error"))
                .path("/")
                .secure(true)
                .finish();
            cookies.add(cookie);
            Json(user)
        })
        .map_err(|error| error_status(error))
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Result<(), Status> {
    cookies.remove(Cookie::named("jwt"));
    Ok(())
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<RegisterUser>, connection: DbConn, mut cookies: Cookies) -> Result<Json<User>, Status> {
    users::repository::insert(user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| {
            println!("{:?}", error);
            error_status(error)
        })
}

#[post("/validate")]
pub fn validate(mut cookies: Cookies) -> Json<bool> {
    match cookies.get("jwt") {
        Some(_token) => {
            let validation = Validation {leeway: 60, validate_exp:false, algorithms: vec![Algorithm::HS512], ..Default::default()};
            match decode::<User>(&_token.value(), &DecodingKey::from_secret("secret".as_ref()), &validation) {
                Ok(token) => Json(true),
                Err(e) => Json(false)
            }
        },
        _ => Json(false)
    }
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
