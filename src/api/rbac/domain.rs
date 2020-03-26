use crate::dao::{Conn, model::rbac::domain::*};
use crate::api::{APIResult};
use rocket_contrib::json::Json;
use validator::Validate;

#[post("/domain", data="<upset_domain>")]
pub fn create_domain(upset_domain: Json<UpsetDomain>,conn: Conn) -> APIResult {
  upset_domain.validate()?;
  let domain = upset_domain.create(&*conn)?;
  Ok(response!(domain))
}

#[put("/domain/<id>", data="<upset_domain>")]
pub fn update_domain(id: i32, upset_domain: Json<UpsetDomain>, conn: Conn) -> APIResult {
  upset_domain.validate()?;
  let domain = upset_domain.save(id, &*conn)?;
  Ok(response!(domain))
}

#[delete("/domain/<id>")]
pub fn delete_domain(id: i32, conn: Conn) -> APIResult {
  let count = Domain::delete_one(id, &*conn)?;
  Ok(response!(count))
}

#[get("/domain/<id>")]
pub fn domain(id: i32, conn: Conn) -> APIResult{
  let domain = Domain::find_one(id, &*conn)?;
  Ok(response!(domain)) 
}

#[get("/domain")]
pub fn domains(conn: Conn) -> APIResult {
  let domains = Domain::find_all(&*conn)?;
  Ok(response!(domains))
}