pub mod multipart;
mod thumbnail;
mod upload;

use rocket::{http::Method::*, Route};

pub fn apply_routes() -> Vec<Route> {
  let upload = Route::new(Post, "/upload?<domain_id>", upload::upload);
  let mut routes = vec![upload];
  routes.extend(routes![thumbnail::thumbnail].into_iter());
  routes
}
