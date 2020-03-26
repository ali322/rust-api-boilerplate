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

mod api;
mod dao;

pub struct App;

#[get("/")]
pub fn index() -> &'static str {
  "welcome to aid"
}

impl App {
  pub fn new() -> rocket::Rocket {
    use api::*;
    use dao::Conn;
    rocket::ignite()
      .attach(Conn::fairing())
      .mount("/", routes![index])
      .mount(
        "/api/v1/",
        routes![
          auth::register,
          auth::login,
          auth::users,
          auth::user,
          auth::update_user,
          auth::delete_user,
        ],
      )
      .register(catchers![error::unprocessable_entity, error::unauthorized])
  }
}
