use crate::dao::schema::{roles, user_has_roles};
use chrono::{prelude::*};
use diesel::Identifiable;
use diesel::{
  delete, insert_into, prelude::*, result::Error as DieselError, update, Insertable, PgConnection,
};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
pub struct Role {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

impl Role {
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

#[derive(Debug, Validate, Insertable, Serialize, Deserialize)]
#[table_name = "roles"]
pub struct NewRole {
  pub name: String,
  pub description: String,
  pub domain_id: i32,
}

impl NewRole {
  pub fn create(&self, conn: &PgConnection) -> Result<Role, DieselError> {
    insert_into(roles::table)
      .values(self)
      .get_result::<Role>(conn)
  }
}

#[derive(Debug, Validate, AsChangeset, Serialize, Deserialize)]
#[table_name = "roles"]
pub struct UpdateRole {
  pub name: Option<String>,
  pub description: Option<String>,
  pub domain_id: Option<i32>,
}

impl UpdateRole {
  pub fn save(&self, id: i32, conn: &PgConnection) -> Result<Role, DieselError> {
    update(roles::table.find(id))
      .set(self)
      .get_result::<Role>(conn)
  }
}

#[derive(Debug, Queryable, Insertable, Validate, Serialize, Deserialize)]
#[table_name = "user_has_roles"]
pub struct UserHasRoles {
  pub user_id: Uuid,
  pub role_id: i32,
  #[serde(with = "date_format")]
  pub expire: NaiveDateTime,
}

mod date_format {
  use serde::{self, Deserialize, Serializer, Deserializer};
  use chrono::{NaiveDateTime, TimeZone};
  const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
  pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
  }
  pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
  }
}

impl UserHasRoles {
  pub fn create(&self, conn: &PgConnection) -> Result<UserHasRoles, DieselError> {
    insert_into(user_has_roles::table)
      .values(self)
      .get_result::<UserHasRoles>(conn)
  }
}

#[derive(Debug, Queryable, Validate, Serialize, Deserialize)]
pub struct DeleteUserHasRoles {
  pub user_id: Uuid,
  pub role_id: i32,
}

impl DeleteUserHasRoles {
  pub fn delete_one(&self, conn: &PgConnection) -> Result<usize, DieselError> {
    delete(
      user_has_roles::table
        .filter(user_has_roles::role_id.eq(self.role_id))
        .filter(user_has_roles::user_id.eq(self.user_id)),
    )
    .execute(conn)
  }
}
