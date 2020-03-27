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

pub struct App;

impl App {
  pub fn new() -> rocket::Rocket {
    use api::*;
    use dao::Conn;
    rocket::ignite()
      .attach(Conn::fairing())
      .mount(
        "/api/v1/",
        api::apply_routes()
        // routes![
          
          
        //   rbac::domain::create_domain,
        //   rbac::domain::update_domain,
        //   rbac::domain::delete_domain,
        //   rbac::domain::domain,
        //   rbac::domain::domains,
        // ],
      )
      .register(catchers![
        error::unprocessable_entity,
        error::unauthorized,
        error::not_found
      ])
  }
}
