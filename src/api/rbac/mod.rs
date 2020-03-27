pub mod domain;
pub mod role;

use rocket::Route;

pub fn apply_routes() -> Vec<Route> {
  routes![
    domain::create_domain,
    domain::update_domain,
    domain::delete_domain,
    domain::domain,
    domain::domains,
  ]
}
