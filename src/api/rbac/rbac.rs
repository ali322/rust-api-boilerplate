use crate::api::{jwt::AuthToken, APIResult, UuidParam};
use crate::dao::{
  model::rbac::{action::*, role::*},
  Conn,
};
use std::collections::HashMap;

#[get("/access?<user_id>&<role_id>&<action_name>")]
pub fn user_access(user_id: UuidParam, role_id: i32, action_name: String, conn: Conn) -> APIResult {
  let action_names: Vec<String> = action_name
    .as_str()
    .split(",")
    .map(|e| e.to_string())
    .collect();
  let actions = Action::find_all_by_name(action_names, &*conn)?;
  let role_has_actions = RoleHasActions::find_all(role_id, &*conn)?;
  let mut permissions: HashMap<String, bool> = HashMap::new();
  for action in actions {
    if role_has_actions.iter().any(|e| e.action_id == action.id) {
      permissions.insert(action.name, true);
    } else {
      permissions.insert(action.name, false);
    }
  }
  let user_has_roles = UserHasRoles::find_one(&user_id.into_inner(), role_id, &*conn);
  if user_has_roles.is_err() {
    for (_, val) in permissions.iter_mut() {
      *val = false;
    }
  }
  Ok(response!(permissions))
}

#[get("/user/role?<user_id>&<domain_id>")]
pub fn user_roles(
  user_id: UuidParam,
  domain_id: i32,
  _token: AuthToken,
  conn: Conn,
) -> APIResult {
  let user_has_roles = UserHasRoles::find_all(&user_id.into_inner(), &*conn)?;
  let role_ids = user_has_roles.iter().map(|e| e.role_id).collect::<Vec<i32>>();
  let mut roles = Role::find_all(Some(domain_id), &*conn)?;
  roles = roles
    .into_iter()
    .filter(|e| role_ids.contains(&e.id))
    .collect();
  Ok(response!(roles))
}

#[get("/role/action?<role_id>&<domain_id>")]
pub fn role_actions(
  role_id: i32,
  domain_id: i32,
  _token: AuthToken,
  conn: Conn,
 ) -> APIResult {
  let role_has_actions = RoleHasActions::find_all(role_id, &*conn)?;
  let action_ids = role_has_actions.iter().map(|e| e.action_id).collect::<Vec<i32>>();
  let mut actions = Action::find_all(Some(domain_id), &*conn)?;
  actions = actions.into_iter().filter(|e|action_ids.contains(&e.id)).collect();
  Ok(response!(actions))
}
