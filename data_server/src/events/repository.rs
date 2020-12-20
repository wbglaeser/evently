#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::events;
use crate::events::{ InsertableEvent, Event };

pub fn insert(event: InsertableEvent, connection: &PgConnection) -> QueryResult<Event> {
    diesel::insert_into(events::table)
        .values(&event)
        .get_result(connection)
}

pub fn get(id: String, connection: &PgConnection) -> QueryResult<Vec<Event>> {
    match events::table.filter(
        events::user_id.eq(&id)
    ).get_results::<Event>(&*connection) {
        Ok(events) => Ok(events),
        Err(e) => Err(e),
    }}
