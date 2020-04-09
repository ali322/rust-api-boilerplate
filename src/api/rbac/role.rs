use crate::api::APIResult;
use crate::dao::{model::rbac::role::*, Conn};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/role", data = "<new>")]
pub fn create_role(new: Json<NewRole>, conn: Conn) -> APIResult {
  new.validate()?;
  let role = new.create(&*conn)?;
  Ok(response!(role))
}

#[put("/role/<id>", data = "<update>")]
pub fn update_role(id: i32, update: Json<UpdateRole>, conn: Conn) -> APIResult {
  update.validate()?;
  let role = update.save(id, &*conn)?;
  Ok(response!(role))
}

#[delete("/role/<id>")]
pub fn delete_role(id: i32, conn: Conn) -> APIResult {
  let count = Role::delete_one(id, &*conn)?;
  Ok(response!(count))
}

#[get("/role/<id>")]
pub fn role(id: i32, conn: Conn) -> APIResult {
  let role = Role::find_one(id, &*conn)?;
  Ok(response!(role))
}

#[get("/role?<domain_id>")]
pub fn roles(domain_id: Option<i32>, conn: Conn) -> APIResult {
  let roles = Role::find_all(domain_id, &*conn)?;
  Ok(response!(roles))
}

#[post("/grant/role", data = "<grant>")]
pub fn grant_role(grant: Json<UserHasRoles>, conn: Conn) -> APIResult {
  let user_has_roles = grant.create(&*conn)?;
  Ok(response!(user_has_roles))
}

#[post("/revoke/role", data = "<revoke>")]
pub fn revoke_role(revoke: Json<DeleteUserHasRoles>, conn: Conn) -> APIResult {
  let count = revoke.delete_one(&*conn)?;
  Ok(response!(count))
}
