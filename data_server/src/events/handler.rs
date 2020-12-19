use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::events;
use crate::sessions;
use events::Event;
use events::repository::InsertableEvent;
use rocket::http::Status;
use crate::users;
use users::User;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::{Cookies, Cookie};

#[post("/", data = "<event>")]
pub fn post(event: Json<Event>, connection: DbConn, mut cookies: Cookies) -> Result<status::Created<Json<InsertableEvent>>, Status> {
    sessions::repository::validate_session(cookies, &connection);
    let newEvent: InsertableEvent = InsertableEvent::from_event(event.into_inner(), String::from("test"));
    events::repository::insert(newEvent, &connection)
        .map(|event| event_created(event))
        .map_err(|error| error_status(error))
}
/*

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<InsertableEvent>>, Status> {
    events::repository::all(&connection)
        .map(|event| Json(event))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<InsertableEvent>, Status> {
    events::repository::get(id, &connection)
        .map(|event| Json(event))
        .map_err(|error| error_status(error))
}

#[get("/", data="<user>")]
pub fn get_events_by_user(user: Json) -> Result<Json<Vec<Event>>, Status> {
    events::repository::get_events_by_user(user, &connection)
        .map(|event| Json(event))
        .map_err(|error| error_status(error))
}


#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match events::repository::get(id, &connection) {
        Ok(_) => events::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
*/
fn event_created(event: InsertableEvent) -> status::Created<Json<InsertableEvent>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
    status::Created(
        format!("{host}:{port}/evently", host = host, port = port).to_string(),
        Some(Json(event)))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
