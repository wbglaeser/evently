use crate::events;
use rocket;
use crate::connection;
use crate::cors;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(cors::CORS())
        .mount("/",
               routes![
               events::handler::post,
               events::handler::get,
               ],
        ).launch();
}
