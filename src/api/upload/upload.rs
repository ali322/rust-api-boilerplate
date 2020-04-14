use super::multipart::handle_multipart;
use crate::api::{APIError, Conf};
use crate::dao::{model::rbac::domain::*, Conn};
use rocket::{handler::Outcome, request::Request, Data, State};
use rocket_contrib::json::JsonValue;
use std::{collections::HashMap, path::Path};

pub fn upload<'r>(req: &'r Request, data: Data) -> Outcome<'r> {
  let conf = req.guard::<State<Conf>>().succeeded().unwrap();
  let conn = req.guard::<Conn>().succeeded().unwrap();
  let domain_id = req
    .get_query_value("domain_id")
    .and_then(|r: Result<String, _>| r.ok());
  if domain_id.is_none() {
    return Outcome::from(req, APIError::from("domain_id is empty"));
  }
  let parsed_id = domain_id.unwrap().as_str().parse::<i32>();
  if parsed_id.is_err() {
    return Outcome::from(req, APIError::from("domain_id is invalid"));
  }
  let domain = Domain::find_one(parsed_id.unwrap(), &*conn);
  if domain.is_err() {
    return Outcome::from(req, APIError::from("domain not exists"));
  }
  let domain_name = domain.unwrap().name;
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
        match f.save(&Path::new(&conf.upload_dir).join(&domain_name)) {
          Ok((file_name, dest_name)) => {
            files.insert(
              file_name,
              format!("{}/{}/{}", conf.upload_base_url, domain_name, dest_name),
            );
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
