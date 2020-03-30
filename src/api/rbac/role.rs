use crate::dao::{Conn, model::rbac::role::*};
use crate::api::{APIResult};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/role", data="<new>")]
pub fn create_role(new: Json<NewRole>,conn: Conn) -> APIResult {
  new.validate()?;
  let role = new.create(&*conn)?;
  Ok(response!(role))
}

#[put("/role/<id>", data="<update>")]
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
pub fn role(id: i32, conn: Conn) -> APIResult{
  let role = Role::find_one(id, &*conn)?;
  Ok(response!(role)) 
}

#[get("/role")]
pub fn roles(conn: Conn) -> APIResult {
  let roles = Role::find_all(&*conn)?;
  Ok(response!(roles))
}