#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use std::sync::Mutex;

pub mod routes;
use crate::routes::{index, new_url, UrlStruct};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, new_url])
        .manage(Mutex::new(UrlStruct::new()))
        .mount("/", FileServer::from(relative!("static")))
        .attach(rocket_dyn_templates::Template::fairing())
}
