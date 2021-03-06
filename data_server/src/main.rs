#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate rand;
use dotenv::dotenv;

mod events;
mod schema;
mod connection;
mod router;
mod cors;

fn main() {
    dotenv().ok();
    router::create_routes();
}
