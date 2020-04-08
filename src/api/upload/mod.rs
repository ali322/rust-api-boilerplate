pub mod multipart;
mod upload;
mod thumbnail;

use rocket::{http::Method::*, Route};

pub fn apply_routes() -> Vec<Route> {
  let upload = Route::new(Post, "/upload", upload::upload);
  let mut routes = vec![upload];
  routes.extend(routes![thumbnail::thumbnail].into_iter());
  routes
}
