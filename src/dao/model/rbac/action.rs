use crate::dao::{
  model::rbac::domain::Domain,
  schema::{actions, role_has_actions},
};
use diesel::Identifiable;
use diesel::{
  delete, insert_into, pg::Pg, prelude::*, result::Error as DieselError, update, Insertable,
  PgConnection,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Identifiable, Insertable, Associations, Queryable, Serialize, Deserialize)]
#[belongs_to(Domain)]
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
  pub fn find_all(domain_id: Option<i32>, conn: &PgConnection) -> Result<Vec<Action>, DieselError> {
    let mut query: actions::BoxedQuery<Pg> = actions::table.into_boxed();
    if let Some(x) = domain_id {
      query = query.filter(actions::domain_id.eq(x));
    }
    query.load::<Action>(conn)
  }
  pub fn find_all_by_name(
    names: Vec<String>,
    conn: &PgConnection,
  ) -> Result<Vec<Action>, DieselError> {
    actions::table
      .filter(actions::name.eq_any(names))
      .load::<Action>(conn)
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

#[derive(Debug, Insertable, Associations, Queryable, Validate, Serialize, Deserialize)]
#[table_name = "role_has_actions"]
#[belongs_to(Action)]
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
  pub fn find_one(
    role_id: i32,
    action_id: i32,
    conn: &PgConnection,
  ) -> Result<RoleHasActions, DieselError> {
    role_has_actions::table
      .filter(role_has_actions::role_id.eq(role_id))
      .filter(role_has_actions::action_id.eq(action_id))
      .first::<RoleHasActions>(conn)
  }
  pub fn find_all(role_id: i32, conn: &PgConnection) -> Result<Vec<RoleHasActions>, DieselError> {
    role_has_actions::table
      .filter(role_has_actions::role_id.eq(role_id))
      .load::<RoleHasActions>(conn)
  }
}
