use crate::api::{jwt::AuthToken, APIResult};
use crate::dao::{model::rbac::domain::*, Conn};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/domain", data = "<new>")]
pub fn create_domain(new: Json<NewDomain>, _token: AuthToken, conn: Conn) -> APIResult {
  new.validate()?;
  let domain = new.create(&*conn)?;
  Ok(response!(domain))
}

#[put("/domain/<id>", data = "<update>")]
pub fn update_domain(
  id: i32,
  update: Json<UpdateDomain>,
  _token: AuthToken,
  conn: Conn,
) -> APIResult {
  update.validate()?;
  let domain = update.save(id, &*conn)?;
  Ok(response!(domain))
}

#[delete("/domain/<id>")]
pub fn delete_domain(id: i32, _token: AuthToken, conn: Conn) -> APIResult {
  let count = Domain::delete_one(id, &*conn)?;
  Ok(response!(count))
}

#[get("/domain/<id>")]
pub fn domain(id: i32, _token: AuthToken, conn: Conn) -> APIResult {
  let domain = Domain::find_one(id, &*conn)?;
  Ok(response!(domain))
}

#[get("/domain")]
pub fn domains(_token: AuthToken, conn: Conn) -> APIResult {
  let domains = Domain::find_all(&*conn)?;
  Ok(response!(domains))
}
