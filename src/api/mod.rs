use diesel::result::Error as DieselError;
use rocket::{
  http::{ContentType, Status, RawStr},
  request::{Request, FromParam},
  response::{Responder, Response, Result as RocketResult},
};
use rocket_contrib::json::JsonValue;
use std::error::Error;
use std::fmt::{Display, Formatter, self};
use std::collections::HashMap;
use validator::{ValidationErrors};
use uuid::{Uuid, parser::ParseError as UuidParseError};

#[derive(Debug)]
pub struct APIError {
  code: i8,
  message: JsonValue,
}

impl Display for APIError{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    f.write_str(&format!("{:?}", self))
  }
}

impl Error for APIError {}

impl<'r> Responder<'r> for APIError {
  fn respond_to(self, _: &Request) -> RocketResult<'r> {
    use std::io::Cursor;
    let body = json!({"code": self.code, "message": self.message}).to_string();
    Response::build()
      .status(Status::Ok)
      .header(ContentType::new("application", "json"))
      .sized_body(Cursor::new(body))
      .ok()
  }
}

impl From<DieselError> for APIError {
  fn from(e: DieselError) -> Self {
    APIError {
      code: -2,
      message: json!(e.to_string()),
    }
  }
}

impl From<ValidationErrors> for APIError{
  fn from(e: ValidationErrors) -> Self {
    let errors = e.field_errors()
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
        message: json!(errors),
      }
  }
}

impl From<&'static str> for APIError{
  fn from(e: &'static str) -> Self {
    APIError{code: -1, message: json!(e)}
  }
}

pub type APIResult = Result<JsonValue, APIError>;


macro_rules! response {
  ($val:tt) => {
    json!({
      "code": 0,
      "data" : $val
    }).into()
  };
}

pub struct UuidParam(Uuid);

impl<'r> FromParam<'r> for UuidParam{
  type Error = UuidParseError;
  fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
    Uuid::parse_str(param.as_str()).map(|v|UuidParam(v))
  }
}

impl UuidParam {
  pub fn into_inner(self) -> Uuid {
    self.0
  }
}

pub mod auth;
pub mod error;
pub mod jwt;
pub mod rbac;
