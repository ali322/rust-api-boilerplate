use crate::api::{
  jwt::{generate_token, AuthToken},
  APIError, APIResult,
};
use crate::dao::{model::user::*, Conn};
use rocket::request::Form;
use validator::Validate;

#[post("/register", data = "<new_user>")]
pub fn register(new_user: Form<NewUser>, conn: Conn) -> APIResult {
  new_user.validate()?;
  if new_user.is_valid_username(&*conn) {
    return Err(APIError::from("use existed!"));
  }
  let user = new_user.create(&*conn)?;
  Ok(response!({
    "token": generate_token(user.clone()), "user": user
  }))
}

#[post("/login", data = "<login_user>")]
pub fn login(login_user: Form<LoginUser>, conn: Conn) -> APIResult {
  login_user.validate()?;
  let user = login_user.find_one(&*conn)?;
  if !login_user.is_password_matched(&user.password) {
    return Err(APIError::from("password not matched"));
  }
  Ok(response!({
    "token": generate_token(user.clone()), "user": user
  }))
}

#[get("/user?<page>&<limit>")]
pub fn users(page: Option<i64>, limit: Option<i64>, conn: Conn) -> APIResult {
  let page = page.unwrap_or(1);
  let limit = limit.unwrap_or(10);
  let count = User::count_users(&*conn)?;
  let rows = User::find_all(page, limit, &*conn)?;
  Ok(response!({ "count": count, "rows": rows}))
}

#[get("/user/<id>")]
pub fn user(id: i32, conn: Conn) -> APIResult {
  let user = User::find_one(id, &*conn)?;
  Ok(response!(user))
}

#[put("/user/<id>", data = "<update_user>")]
pub fn update_user(id: i32, update_user: Form<UpdateUser>, conn: Conn) -> APIResult {
  update_user.validate()?;
  let updated = update_user.save(id, &*conn)?;
  Ok(response!(updated))
}

#[delete("/user/<id>")]
pub fn delete_user(id: i32, conn: Conn) -> APIResult {
  let count = User::delete_one(id, &*conn)?;
  Ok(response!(count))
}

#[get("/test")]
pub fn auth(token: AuthToken) -> APIResult {
  println!("token: {:?}", token);
  Ok(response!("ok"))
}
