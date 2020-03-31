use jsonwebtoken::{Header, encode, decode, TokenData, Validation, errors:: Result};
use serde::{Serialize, Deserialize};
use rocket::{http::Status, request::{FromRequest, Request, Outcome}, response::status};
use rocket_contrib::json::JsonValue;
use crate::dao::model::user::User;
use chrono::{Utc, Duration};
use std::ops::Add;

const KEY: &'static str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken{
  pub iat: i64,
  pub exp: i64,
  pub user: User,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken{
  type Error = status::Custom<JsonValue>;
  fn from_request(req: &'a Request<'r>) -> Outcome<Self, status::Custom<JsonValue>> {
    if let Some(auth_header) = req.headers().get_one("Authorization") {
      let auth_str = auth_header.to_string();
      if auth_str.starts_with("Bearer") {
        let token = auth_str[6..auth_str.len()].trim();
        let ret = decode_token(token.to_string());
        if let Ok(token_data) = ret {
          return Outcome::Success(token_data.claims);
        }
      }
    }
    Outcome::Failure((
      Status::Unauthorized,
      status::Custom(
        Status::BadRequest,
        json!("auth token invalid").into()
      )
    ))
  }
}

pub fn generate_token<'r>(user: User) -> String {
  let now = Utc::now();
  let payload = AuthToken{
    iat: now.timestamp(),
    exp: now.add(Duration::days(30)).timestamp(),
    user: user,
  };
  encode(&Header::default(), &payload, KEY.as_ref()).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<AuthToken>>{
  decode::<AuthToken>(&token, KEY.as_ref(), &Validation::default())
}