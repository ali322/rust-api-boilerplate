use rocket::{http::ContentType, State};
use super::multipart::MultipartData;
use crate::api::{APIResult, APIError, Conf};
use std::{path::Path, collections::HashMap};

#[post("/simple/upload", data="<data>")]
pub fn upload(content_type: &ContentType, data: MultipartData, conf: State<Conf>) -> APIResult {
  let  mut files: HashMap<String, String> = HashMap::new();
  for f in data.files {
    let (file_name, dest_name) = f.save(Path::new(&conf.upload)).map_err(|e|APIError::from(e))?;
    files.insert(file_name, dest_name);
  }
  Ok(response!(files))
}