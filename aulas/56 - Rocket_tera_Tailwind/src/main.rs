#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![
            controllers::home_controller::index,
            controllers::home_controller::sobre,
        ])
        .attach(Template::fairing())
}
