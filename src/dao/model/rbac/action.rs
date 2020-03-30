use crate::dao::schema::{actions, role_has_actions};
use diesel::Identifiable;
use diesel::{
  delete, insert_into, prelude::*, result::Error as DieselError, update, Insertable, PgConnection,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
pub struct Action {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

impl Action {
  pub fn delete_one(id: i32, conn: &PgConnection) -> Result<usize, DieselError> {
    delete(actions::table.find(id)).execute(conn)
  }
  pub fn find_one(id: i32, conn: &PgConnection) -> Result<Action, DieselError> {
    actions::table.find(id).first::<Action>(conn)
  }
  pub fn find_all(conn: &PgConnection) -> Result<Vec<Action>, DieselError> {
    actions::table.load::<Action>(conn)
  }
}

#[derive(Debug, Validate, Insertable, Serialize, Deserialize)]
#[table_name = "actions"]
pub struct NewAction {
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

impl NewAction {
  pub fn create(&self, conn: &PgConnection) -> Result<Action, DieselError> {
    insert_into(actions::table)
      .values(self)
      .get_result::<Action>(conn)
  }
}

#[derive(Debug, Validate, AsChangeset, Serialize, Deserialize)]
#[table_name = "actions"]
pub struct UpdateAction {
  pub name: Option<String>,
  pub description: Option<String>,
  pub domain_id: Option<i32>,
}

impl UpdateAction {
  pub fn save(&self, id: i32, conn: &PgConnection) -> Result<Action, DieselError> {
    update(actions::table.find(id))
      .set(self)
      .get_result::<Action>(conn)
  }
}

#[derive(Debug, Insertable, Queryable, Validate, Serialize, Deserialize)]
#[table_name = "role_has_actions"]
pub struct RoleHasActions {
  pub role_id: i32,
  pub action_id: i32,
}

impl RoleHasActions {
  pub fn create(&self, conn: &PgConnection) -> Result<RoleHasActions, DieselError> {
    insert_into(role_has_actions::table)
      .values(self)
      .get_result::<RoleHasActions>(conn)
  }
  pub fn delete_one(&self, conn: &PgConnection) -> Result<usize, DieselError> {
    delete(
      role_has_actions::table
        .filter(role_has_actions::role_id.eq(self.role_id))
        .filter(role_has_actions::action_id.eq(self.action_id)),
    )
    .execute(conn)
  }
}
