use crate::events;
use crate::users;
use crate::sessions;
use rocket;
use crate::connection;
use crate::cors;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(cors::CORS())
        .mount("/events",
               routes![
               events::handler::post],
        ).launch();
}
