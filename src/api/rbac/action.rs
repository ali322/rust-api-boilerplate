use crate::api::APIResult;
use crate::dao::{model::rbac::action::*, Conn};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/action", data = "<new>")]
pub fn create_action(new: Json<NewAction>, conn: Conn) -> APIResult {
  new.validate()?;
  let action = new.create(&*conn)?;
  Ok(response!(action))
}

#[put("/action/<id>", data = "<update>")]
pub fn update_action(id: i32, update: Json<UpdateAction>, conn: Conn) -> APIResult {
  update.validate()?;
  let action = update.save(id, &*conn)?;
  Ok(response!(action))
}

#[delete("/action/<id>")]
pub fn delete_action(id: i32, conn: Conn) -> APIResult {
  let count = Action::delete_one(id, &*conn)?;
  Ok(response!(count))
}

#[get("/action/<id>")]
pub fn action(id: i32, conn: Conn) -> APIResult {
  let action = Action::find_one(id, &*conn)?;
  Ok(response!(action))
}

#[get("/action")]
pub fn actions(conn: Conn) -> APIResult {
  let actions = Action::find_all(&*conn)?;
  Ok(response!(actions))
}

#[post("/grant/action", data = "<grant>")]
pub fn grant_action(grant: Json<RoleHasActions>, conn: Conn) -> APIResult {
  let role_has_actions = grant.create(&*conn)?;
  Ok(response!(role_has_actions))
}

#[post("/revoke/action", data = "<revoke>")]
pub fn revoke_action(revoke: Json<RoleHasActions>, conn: Conn) -> APIResult {
  let count = revoke.delete_one(&*conn)?;
  Ok(response!(count))
}
