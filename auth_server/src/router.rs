use crate::users;
use rocket;
use crate::connection;
use crate::cors;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(cors::CORS())
        .mount("/",
               routes![
               users::handler::login,
               users::handler::register
               ],
        ).launch();
}
