use crate::dao::model::*;
use crate::dao::Conn;
use diesel::insert_into;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(FromForm)]
pub struct RegisterForm {
  username: String,
  password: String,
  email: String,
}

#[derive(Serialize)]
pub struct AuthResult {
  token: &'static str,
  user: User,
}

#[post("/register", data = "<form>")]
pub fn register(
  form: Form<RegisterForm>,
  conn: Conn,
) -> Result<Json<AuthResult>, BadRequest<String>> {
  use crate::dao::schema::users::dsl::*;
  use chrono::Local;
  let ret = users.filter(username.eq(&form.username)).first::<User>(&*conn);
  if ret.is_ok() {
    return Err(BadRequest(Some("use existed".to_string())));
  }
  let now = Local::now().naive_local();
  let ret = insert_into(users)
    .values((
      username.eq(&form.username),
      password.eq(&form.password),
      email.eq(&form.email),
      last_logined_at.eq(now),
    ))
    .get_result::<User>(&*conn);
  if ret.is_err() {
    return Err(BadRequest(ret.err().map(|e| e.to_string())));
  }

  Ok(Json(AuthResult {
    token: "test",
    user: ret.unwrap(),
  }))
}
