#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::events;

pub mod handler;
pub mod repository;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String,
    user_id: String,
    date: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct EventRequest {
    name: String,
    date: String,
    location: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "events"]
pub struct InsertableEvent {
    name: String,
    user_id: String,
    date: String,
    location: String,
}

impl InsertableEvent {
    fn from_request(event: EventRequest, user: User) -> InsertableEvent {
        InsertableEvent {
            name: event.name,
            user_id: user.id.to_string(),
            date: event.date,
            location: event.location,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}
