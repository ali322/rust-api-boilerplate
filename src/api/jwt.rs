use crate::api::Conf;
use crate::dao::model::user::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Result, Header, TokenData, Validation};
use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
  response::status,
  State,
};
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
  pub id: Uuid,
  pub username: String,
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
  pub iat: i64,
  pub exp: i64,
  pub payload: Payload,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
  type Error = status::Custom<JsonValue>;
  fn from_request(req: &'a Request<'r>) -> Outcome<Self, status::Custom<JsonValue>> {
    if let Some(auth_header) = req.headers().get_one("Authorization") {
      let auth_str = auth_header.to_string();
      if auth_str.starts_with("Bearer") {
        let token = auth_str[6..auth_str.len()].trim();
        if let Some(conf) = req.guard::<State<Conf>>().succeeded() {
          let decoded = decode_token(token.to_string(), &conf.jwt_key);
          if let Ok(token_data) = decoded {
            return Outcome::Success(token_data.claims);
          }
        }
      }
    }
    Outcome::Failure((
      Status::Unauthorized,
      status::Custom(Status::BadRequest, json!("auth token invalid").into()),
    ))
  }
}

pub fn generate_token<'r>(user: User, key: &str) -> String {
  let now = Utc::now();
  let payload = AuthToken {
    iat: now.timestamp(),
    exp: now.add(Duration::days(30)).timestamp(),
    payload: Payload {
      id: user.id,
      username: user.username,
      email: user.email,
    },
  };
  encode(&Header::default(), &payload, key.as_ref()).unwrap()
}

fn decode_token(token: String, key: &str) -> Result<TokenData<AuthToken>> {
  decode::<AuthToken>(&token, key.as_ref(), &Validation::default())
}
