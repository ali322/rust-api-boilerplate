use crate::api::{jwt::generate_token, APIError, APIResult, Conf};
use crate::dao::{model::user::*, Conn};
use reqwest;
use rocket::{Route, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Connect<'r> {
  pub id: &'r str,
  pub secret: &'r str,
  pub code: &'r str,
  pub redirect: &'r str,
}

const SSO_HOST: &'static str = "https://gateway-inner.asiainfo.com";

#[post("/nt/connect", format = "json", data = "<connect>")]
pub fn connected(connect: Json<Connect>, conf: State<Conf>, conn: Conn) -> APIResult {
  let mut url = format!("{}/gateway/bin/oauth2/token", SSO_HOST);
  let form = [
    ("client_id", connect.id),
    ("client_secret", connect.secret),
    ("code", connect.code),
    ("grant_type", "authorization_code"),
    ("redirect_uri", connect.redirect),
  ];
  let client = reqwest::blocking::Client::new();
  let mut ret = client
    .post(&url)
    .form(&form)
    .send()
    .map_err(|e| e.to_string())?;
  let body: Value = from_str(&ret.text_with_charset("utf-8").unwrap()).unwrap();
  let access_token = body["access_token"].as_str().unwrap();
  if !body["error"].is_null() {
    return Err(APIError::from(body["error_description"].to_string()));
  }
  url = format!("{}/api/v1.0.0/account/employee/getCurrentStaff", SSO_HOST);
  ret = client
    .get(&url)
    .header("Authorization", format!("bearer {}", access_token))
    .send()
    .map_err(|e| e.to_string())?;
  let body = from_str::<Value>(&ret.text_with_charset("utf-8").unwrap()).unwrap();
  if !body["error"].is_null() {
    return Err(APIError::from(body["error_description"].to_string()));
  }
  let username = body["staffInfo"]["username"].as_str().unwrap();
  let email = body["staffInfo"]["email"].as_str().unwrap();
  let info = body["staffInfo"].to_string();
  let new_user = NewUser {
    username: username.to_string(),
    password: "ai123456".to_string(),
    email: email.to_string(),
    memo: Some(info),
  };
  if let Ok(user) = new_user.exists(&*conn) {
    Ok(response!({
      "token": generate_token(user.clone(), conf.jwt_key.as_str()), "user": user
    }))
  } else {
    let user = new_user.create(&*conn)?;
    Ok(response!({
      "token": generate_token(user.clone(), &conf.jwt_key), "user": user
    }))
  }
}

pub fn apply_routes() -> Vec<Route> {
  routes![connected]
}
