use crate::api::{APIResult, APIError, jwt::{generate_token, AuthToken}};
use crate::dao::{model::*, Conn};
use rocket::request::LenientForm;
use validator::Validate;

#[post("/register", data = "<new_user>")]
pub fn register(new_user: LenientForm<NewUser>, conn: Conn) -> APIResult {
  new_user.validate()?;
  if !new_user.is_valid_username(&*conn) {
    return Err(APIError::from("use existed!"));
  }
  let user = new_user.create(&*conn)?;
  Ok(response!({
    "token": generate_token(user.clone()), "user": user
  }))
}

#[get("/test")]
pub fn auth(token: AuthToken) -> APIResult {
  println!("token: {:?}", token);
  Ok(response!("ok"))
}