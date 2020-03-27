use crate::dao::schema::domains;
use diesel::Identifiable;
use diesel::{insert_into, update, delete, prelude::*, result::Error as DieselError, Insertable, PgConnection};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
pub struct Domain {
  pub id: i32,
  pub name: String,
  pub description: String,
}

#[derive(Debug, Validate, Insertable, Serialize, Deserialize)]
#[table_name = "domains"]
pub struct NewDomain {
  pub name: String,
  pub description: String,
}

#[derive(Debug, Validate, AsChangeset, Serialize, Deserialize)]
#[table_name = "domains"]
pub struct UpdateDomain {
  pub name: Option<String>,
  pub description: Option<String>,
}

impl NewDomain {
  pub fn create(&self, conn: &PgConnection) -> Result<Domain, DieselError> {
    insert_into(domains::table)
      .values(self)
      .get_result::<Domain>(conn)
  }
}

impl UpdateDomain{
  pub fn save(&self, id: i32, conn: &PgConnection) -> Result<Domain, DieselError> {
    update(domains::table.find(id)).set(self).get_result::<Domain>(conn)
  }
}

impl Domain{
  pub fn delete_one(id: i32, conn: &PgConnection) -> Result<usize, DieselError> {
    delete(domains::table.find(id)).execute(conn)
  }
  pub fn find_one(id: i32, conn: &PgConnection) -> Result<Domain, DieselError> {
    domains::table.find(id).first::<Domain>(conn)
  }
  pub fn find_all(conn: &PgConnection) -> Result<Vec<Domain>, DieselError> {
    domains::table.load::<Domain>(conn)
  }
}
