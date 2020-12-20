use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::events;
use events::{Event, EventRequest, InsertableEvent};
use events::User;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::{Cookies, Cookie};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors};

#[post("/", data = "<event>")]
pub fn post(event: Json<EventRequest>, connection: DbConn, mut cookies: Cookies) -> Result<Json<Event>, Status> {
    let _token = cookies.get("jwt").expect("No cookie set");
    let validation = Validation {leeway: 60, validate_exp:false, algorithms: vec![Algorithm::HS512], ..Default::default()};
    match decode::<User>(&_token.value(), &DecodingKey::from_secret("secret".as_ref()), &validation) {
        Ok(token) => {
            let new_event = InsertableEvent::from_request(event.into_inner(), token.claims);
            events::repository::insert(new_event, &connection)
                .map(|event| Json(event))
                .map_err(|error| {
                    println!("{:?}", error);
                    error_status(error)
                })
            },
        Err(e) => {
            Err(Status::InternalServerError)
        }
    }
}

#[get("/all")]
pub fn get(connection: DbConn, mut cookies: Cookies) -> Result<Json<Vec<Event>>, Status> {
    let _token = cookies.get("jwt").expect("No cookie set");
    let validation = Validation {leeway: 60, validate_exp:false, algorithms: vec![Algorithm::HS512], ..Default::default()};
    match decode::<User>(&_token.value(), &DecodingKey::from_secret("secret".as_ref()), &validation) {
        Ok(token) => {
            events::repository::get(token.claims.id.to_string(), &connection)
                .map(|events| Json(events))
                .map_err(|error| {
                    println!("{:?}", error);
                    error_status(error)
                })
            },
        _ => Err(Status::NotFound)
    }
}



fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
