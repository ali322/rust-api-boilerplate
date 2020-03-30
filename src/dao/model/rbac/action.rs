use crate::dao::schema::actions;
use diesel::Identifiable;
use diesel::{insert_into, update, delete, prelude::*, result::Error as DieselError, Insertable, PgConnection};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
pub struct Action {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

#[derive(Debug, Validate, Insertable, Serialize, Deserialize)]
#[table_name = "actions"]
pub struct NewAction {
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

#[derive(Debug, Validate, AsChangeset, Serialize, Deserialize)]
#[table_name = "actions"]
pub struct UpdateAction {
  pub name: Option<String>,
  pub description: Option<String>,
  pub domain_id: Option<i32>,
}

impl NewAction {
  pub fn create(&self, conn: &PgConnection) -> Result<Action, DieselError> {
    insert_into(actions::table)
      .values(self)
      .get_result::<Action>(conn)
  }
}

impl UpdateAction{
  pub fn save(&self, id: i32, conn: &PgConnection) -> Result<Action, DieselError> {
    update(actions::table.find(id)).set(self).get_result::<Action>(conn)
  }
}

impl Action{
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
