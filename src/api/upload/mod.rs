// mod upload;
mod simple;
pub mod multipart;

use rocket::Route;

pub fn apply_routes() -> Vec<Route> {
  routes![simple::upload]
}
