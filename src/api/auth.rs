use crate::api::{jwt::generate_token, APIError, APIResult, Conf, UuidParam};
use crate::dao::{model::user::*, Conn};
use rocket::{Route, State};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/register", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>, conf: State<Conf>, conn: Conn) -> APIResult {
  new_user.validate()?;
  if new_user.exists(&*conn).is_ok() {
    return Err(APIError::from("use existed"));
  }
  let user = new_user.create(&*conn)?;
  Ok(response!({
    "token": generate_token(user.clone(), &conf.jwt_key), "user": user
  }))
}

#[post("/login", data = "<login_user>")]
pub fn login(login_user: Json<LoginUser>, conf: State<Conf>, conn: Conn) -> APIResult {
  login_user.validate()?;
  let user = login_user.find_one(&*conn)?;
  if !login_user.is_password_matched(&user.password) {
    return Err(APIError::from("password not matched"));
  }
  Ok(response!({
    "token": generate_token(user.clone(), conf.jwt_key.as_str()), "user": user
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
pub fn user(id: UuidParam, conn: Conn) -> APIResult {
  let user = User::find_one(&id.into_inner(), &*conn)?;
  Ok(response!(user))
}

#[put("/user/<id>", format = "json", data = "<update_user>")]
pub fn update_user(id: UuidParam, update_user: Json<UpdateUser>, conn: Conn) -> APIResult {
  update_user.validate()?;
  let updated = update_user.save(&id.into_inner(), &*conn)?;
  Ok(response!(updated))
}

#[delete("/user/<id>")]
pub fn delete_user(id: UuidParam, conn: Conn) -> APIResult {
  let count = User::delete_one(&id.into_inner(), &*conn)?;
  Ok(response!(count))
}

pub fn apply_routes() -> Vec<Route> {
  routes![register, login, users, user, update_user, delete_user,]
}
