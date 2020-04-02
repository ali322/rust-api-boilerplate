use rocket::{http::ContentType, State};
use super::multipart::MultipartData;
use crate::api::{APIResult, Conf};
use std::path::Path;

#[post("/simple/upload", data="<data>")]
pub fn upload(content_type: &ContentType, data: MultipartData, conf: State<Conf>) -> APIResult {
  for f in data.files {
    f.save(Path::new(&conf.upload));
  }
  Ok(response!("ok"))
}