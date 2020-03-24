use super::APIResult;
use crate::dao::{model::*, Conn};
use diesel::{insert_into, prelude::*};
use rocket::request::Form;
use rocket_contrib::json::Json;
use serde_json::json;
use std::collections::HashMap;
use validator::Validate;

#[derive(FromForm, Validate)]
pub struct RegisterForm {
  username: String,
  password: String,
  #[validate(email)]
  email: String,
}

#[post("/register", data = "<form>")]
pub fn register(form: Form<RegisterForm>, conn: Conn) -> APIResult {
  use crate::dao::schema::users;
  use chrono::Local;
  form
    .validate()
    .map_err(|e| response!(validate_error!(e), -2))?;
  let ret = users::table
    .filter(users::username.eq(&form.username))
    .first::<User>(&*conn);
  if ret.is_ok() {
    return Err(response!("user existed", -2));
  }
  let now = Local::now().naive_local();
  let user = insert_into(users::table)
    .values((
      users::username.eq(&form.username),
      users::password.eq(&form.password),
      users::email.eq(&form.email),
      users::last_logined_at.eq(now),
    ))
    .get_result::<User>(&*conn)
    .map_err(|e| response!(e.to_string(), -1))?;

  Ok(response!({
    "token":"123", "user": user
  }))
}
