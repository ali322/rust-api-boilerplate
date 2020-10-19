use diesel::result::Error as DieselError;
use rocket::{
  http::RawStr,
  request::{FromFormValue, FromParam, Request},
  response::{Responder, Result as RocketResult},
  Route,
};
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use thiserror::Error;
// use std::error::Error;
// use std::fmt::{self, Display, Formatter};
use uuid::{Error as UuidParseError, Uuid};
use validator::ValidationErrors;

#[derive(Debug, Error)]
#[error("{:?}", self)]
pub struct APIError {
  message: String,
  code: i8,
}

// impl Display for APIError {
//   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//     f.write_str(&self.message)
//   }
// }

// impl Error for APIError {}

impl<'r> Responder<'r> for APIError {
  fn respond_to(self, req: &Request) -> RocketResult<'r> {
    let body: JsonValue = json!({"code": self.code, "message": self.message.as_str()}).into();
    body.respond_to(req)
    // Response::build()
    //   .status(Status::Ok)
    //   .header(ContentType::new("application", "json"))
    //   .sized_body(Cursor::new(body))
    //   .ok()
  }
}

impl From<DieselError> for APIError {
  fn from(e: DieselError) -> Self {
    APIError {
      code: -2,
      message: e.to_string(),
    }
  }
}

impl From<ValidationErrors> for APIError {
  fn from(e: ValidationErrors) -> Self {
    let errors = e
      .field_errors()
      .iter()
      .map(|(k, v)| {
        let errors: Vec<String> = v
          .iter()
          .map(|e| {
            if let Some(msg) = e.message.as_ref() {
              return msg.to_string();
            }
            return format!("{} is invalid", e.code.to_string());
          })
          .collect();
        (*k, errors)
      })
      .collect::<HashMap<_, _>>();
    APIError {
      code: -1,
      message: format!("{:?}", errors),
    }
  }
}

impl From<&'static str> for APIError {
  fn from(e: &'static str) -> Self {
    APIError {
      code: -1,
      message: e.to_string(),
    }
  }
}

impl From<String> for APIError {
  fn from(e: String) -> Self {
    APIError {
      code: -1,
      message: e,
    }
  }
}

pub type APIResult = Result<JsonValue, APIError>;

pub struct UuidParam(Uuid);

impl<'r> FromParam<'r> for UuidParam {
  type Error = UuidParseError;
  fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
    Uuid::parse_str(param.as_str()).map(|v| UuidParam(v))
  }
}

impl<'r> FromFormValue<'r> for UuidParam {
  type Error = UuidParseError;
  fn from_form_value(form_value: &'r RawStr) -> Result<Self, Self::Error> {
    Uuid::parse_str(form_value.as_str()).map(|v| UuidParam(v))
  }
}

impl UuidParam {
  pub fn into_inner(self) -> Uuid {
    self.0
  }
}

macro_rules! response {
  ($val:tt) => {
    json!({
      "code": 0,
      "data" : $val
    }).into()
  };
}

#[derive(Debug)]
pub struct Conf {
  pub jwt_key: String,
  pub upload_base_url: String,
  pub upload_dir: String,
  pub upload_size_limit: u64,
  pub upload_allowed_extension: String,
}

pub mod auth;
pub mod error;
pub mod jwt;
pub mod nt;
pub mod rbac;
pub mod upload;

pub fn apply_routes() -> Vec<Route> {
  let mut routes: Vec<Route> = vec![];
  routes.extend(auth::apply_routes().iter().cloned());
  routes.extend(nt::apply_routes().iter().cloned());
  routes.extend(rbac::apply_routes().iter().cloned());
  routes.extend(upload::apply_routes().iter().cloned());
  routes
}
