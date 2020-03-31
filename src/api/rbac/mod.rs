pub mod action;
pub mod domain;
pub mod role;
pub mod rbac;

use rocket::Route;

pub fn apply_routes() -> Vec<Route> {
  routes![
    domain::create_domain,
    domain::update_domain,
    domain::delete_domain,
    domain::domain,
    domain::domains,
    role::create_role,
    role::update_role,
    role::delete_role,
    role::role,
    role::roles,
    role::grant_role,
    role::revoke_role,
    action::create_action,
    action::update_action,
    action::delete_action,
    action::action,
    action::actions,
    action::grant_action,
    action::revoke_action,
    rbac::user_access,
    rbac::user_roles,
    rbac::role_actions,
  ]
}
