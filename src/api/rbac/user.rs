use crate::api::{APIResult};
use crate::dao::{model::rbac::role::*, Conn};
use rocket_contrib::json::Json;
use validator::Validate;

#[get("/access?<user_id>&<role_id>&<action_name>")]
pub fn access(user_id: UuidParam, role_id: i32, action_name: String, conn: Conn) -> APIResult {
  
}