use crate::dao::schema::roles;
use diesel::Identifiable;
use diesel::{insert_into, update, delete, prelude::*, result::Error as DieselError, Insertable, PgConnection};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
pub struct Role {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

#[derive(Debug, Validate, Insertable, Serialize, Deserialize)]
#[table_name = "roles"]
pub struct NewRole {
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

#[derive(Debug, Validate, AsChangeset, Serialize, Deserialize)]
#[table_name = "roles"]
pub struct UpdateRole {
  pub name: Option<String>,
  pub description: Option<String>,
  pub domain_id: Option<i32>,
}

impl NewRole {
  pub fn create(&self, conn: &PgConnection) -> Result<Role, DieselError> {
    insert_into(roles::table)
      .values(self)
      .get_result::<Role>(conn)
  }
}

impl UpdateRole{
  pub fn save(&self, id: i32, conn: &PgConnection) -> Result<Role, DieselError> {
    update(roles::table.find(id)).set(self).get_result::<Role>(conn)
  }
}

impl Role{
  pub fn delete_one(id: i32, conn: &PgConnection) -> Result<usize, DieselError> {
    delete(roles::table.find(id)).execute(conn)
  }
  pub fn find_one(id: i32, conn: &PgConnection) -> Result<Role, DieselError> {
    roles::table.find(id).first::<Role>(conn)
  }
  pub fn find_all(conn: &PgConnection) -> Result<Vec<Role>, DieselError> {
    roles::table.load::<Role>(conn)
  }
}
