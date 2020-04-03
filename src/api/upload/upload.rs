use super::multipart::handle_multipart;
use crate::api::{APIError, Conf};
use rocket::{handler::Outcome, request::Request, Data, State};
use rocket_contrib::json::JsonValue;
use std::{collections::HashMap, path::Path};

pub fn upload<'r>(req: &'r Request, data: Data) -> Outcome<'r> {
  let conf = req.guard::<State<Conf>>().succeeded().unwrap();
  let content_type = req
    .headers()
    .get_one("Content-Type")
    .expect("no content-type");
  match handle_multipart(
    content_type,
    data,
    conf.upload_size_limit,
    &conf.upload_allowed_extension,
  ) {
    Ok(multiparts) => {
      let mut files: HashMap<String, String> = HashMap::new();
      for f in multiparts.files {
        match f.save(Path::new(&conf.upload_dir)) {
          Ok((file_name, dest_name)) => {
            files.insert(file_name, dest_name);
          }
          Err(e) => {
            return Outcome::from(req, APIError::from(e));
          }
        }
      }
      Outcome::from(req, JsonValue(response!(files)))
    }
    Err(e) => Outcome::from(req, APIError::from(e)),
  }
}
