#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
extern crate validator;

extern crate chrono;
extern crate serde;
extern crate jsonwebtoken;

mod dao;
mod api;

pub struct App;

#[get("/")]
pub fn index() -> &'static str {
  "welcome to aid"
}

impl App {
  pub fn new() -> rocket::Rocket{
    use dao::Conn;
    use api::*;
    rocket::ignite()
      .attach(Conn::fairing())
      .mount("/", routes![index])
      .mount("/api/v1/", routes![auth::register, auth::auth])
      .register(catchers![error::unprocessable_entity, error::unauthorized])
  }
}