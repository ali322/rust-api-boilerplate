pub mod multipart;
mod upload;

use rocket::{http::Method::*, Route};

pub fn apply_routes() -> Vec<Route> {
  let upload = Route::new(Post, "/upload", upload::upload);
  vec![upload]
}
