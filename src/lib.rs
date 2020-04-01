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
extern crate jsonwebtoken;
extern crate serde;
extern crate uuid;

mod api;
mod dao;
mod fairing;

pub struct App;

use rocket::{ Rocket, fairing::AdHoc};

impl App {
  pub fn new() -> Rocket {
    use api::*;
    use dao::Conn;
    rocket::ignite()
      .attach(Conn::fairing())
      .mount(
        "/api/v1/",
        apply_routes()
      )
      .attach(fairing::RequestTimer)
      .attach(AdHoc::on_attach("JWT Key", |rocket| {
        let key = rocket.config().get_str("jwt_key").unwrap().to_string();
        Ok(rocket.manage(Conf{ jwt_key: key}))
      }))
      .register(catchers![
        error::unprocessable_entity,
        error::unauthorized,
        error::not_found
      ])
  }
}
