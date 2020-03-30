use crate::dao::{Conn, model::rbac::action::*};
use crate::api::{APIResult};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/action", data="<new>")]
pub fn create_action(new: Json<NewAction>,conn: Conn) -> APIResult {
  new.validate()?;
  let action = new.create(&*conn)?;
  Ok(response!(action))
}

#[put("/action/<id>", data="<update>")]
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
pub fn action(id: i32, conn: Conn) -> APIResult{
  let action = Action::find_one(id, &*conn)?;
  Ok(response!(action)) 
}

#[get("/action")]
pub fn actions(conn: Conn) -> APIResult {
  let actions = Action::find_all(&*conn)?;
  Ok(response!(actions))
}