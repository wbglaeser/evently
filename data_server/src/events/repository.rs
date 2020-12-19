#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::events;
use crate::events::Event;

pub fn insert(event: InsertableEvent, connection: &PgConnection) -> QueryResult<Event> {
    diesel::insert_into(events::table)
        .values(&event)
        .get_result(connection)
}

/*
pub fn all(connection: &PgConnection) -> QueryResult<Vec<InsertableEvent>> {
    events::table.load::<InsertableEvent>(&*connection)
}



pub fn get(id: i32, connection: &PgConnection) -> QueryResult<InsertableEvent> {
    events::table.find(id).get_result::<InsertableEvent>(connection)
}
*/
pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(events::table.find(id))
        .execute(connection)
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "events"]
pub struct InsertableEvent {
    userid: String,
    name: String,
    date: String,
    location: String,
}

impl InsertableEvent {

    pub fn from_event(event: Event, userid: String) -> InsertableEvent {
        InsertableEvent {
            userid: userid,
            name: event.name,
            date: event.date,
            location: event.location
        }
    }
}
