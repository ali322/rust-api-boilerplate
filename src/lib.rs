#![feature(proc_macro_hygiene, decl_macro)]

extern crate openssl;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
extern crate validator;

extern crate base64;
extern crate chrono;
extern crate image;
extern crate jsonwebtoken;
extern crate multipart;
extern crate reqwest;
extern crate rocket_cors;
extern crate serde;
extern crate uuid;

mod api;
mod dao;
mod fairing;

pub struct App;

use rocket::{fairing::AdHoc, Rocket};

impl App {
  pub fn new() -> Rocket {
    use api::*;
    use dao::Conn;

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
      .attach(Conn::fairing())
      .mount("/api/v1/", apply_routes())
      .attach(cors)
      .attach(fairing::RequestTimer)
      .attach(AdHoc::on_attach("JWT Key", |rocket| {
        let key = rocket.config().get_str("jwt_key").unwrap().to_string();
        let upload_base_url = rocket
          .config()
          .get_str("upload_base_url")
          .unwrap()
          .to_string();
        let upload_dir = rocket.config().get_str("upload_dir").unwrap().to_string();
        let upload_allowed_extension = rocket
          .config()
          .get_str("upload_allowed_extension")
          .unwrap()
          .to_string();
        let upload_size_limit = rocket.config().get_int("upload_size_limit").unwrap() as u64;
        Ok(rocket.manage(Conf {
          jwt_key: key,
          upload_base_url,
          upload_dir,
          upload_size_limit,
          upload_allowed_extension,
        }))
      }))
      .register(catchers![
        error::unprocessable_entity,
        error::unauthorized,
        error::not_found
      ])
  }
}
