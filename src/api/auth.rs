use crate::api::{APIResult, APIError};
use crate::dao::{model::*, Conn};
use diesel::{insert_into, prelude::*};
use rocket::request::Form;
use rocket_contrib::json::JsonValue;
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
  form.validate()?;
  let ret = users::table
    .filter(users::username.eq(&form.username))
    .first::<User>(&*conn);
  if ret.is_ok() {
    return Err(APIError::from("use existed!"));
  }
  let now = Local::now().naive_local();
  let user = insert_into(users::table)
    .values((
      users::username.eq(&form.username),
      users::password.eq(&form.password),
      users::email.eq(&form.email),
      users::last_logined_at.eq(now),
    ))
    .get_result::<User>(&*conn)?;

  Ok(response!({
    "token":"123", "user": user
  }))
}
