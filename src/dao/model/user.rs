use crate::dao::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{prelude::*, Local};
use diesel::{
  insert_into, delete, prelude::*, result::Error as DieselError, update, PgConnection, Queryable,
};
use rocket::{request::FromForm};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub password: String,
  pub email: String,
  pub avatar: Option<String>,
  pub memo: Option<String>,
  pub last_logined_at: NaiveDateTime,
}

#[derive(FromForm, Validate)]
pub struct NewUser {
  pub username: String,
  pub password: String,
  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Validate, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
  pub email: Option<String>,
  pub avatar: Option<String>,
}

#[derive(FromForm, Queryable, Validate)]
pub struct LoginUser {
  pub username_or_email: String,
  pub password: String,
}

impl NewUser {
  pub fn is_valid_username(&self, conn: &PgConnection) -> bool {
    users::table
      .filter(users::username.eq(&self.username))
      .first::<User>(conn)
      .is_ok()
  }
  pub fn create(&self, conn: &PgConnection) -> Result<User, DieselError> {
    let now = Local::now().naive_local();
    let hash_password = hash(&self.password, DEFAULT_COST).unwrap();
    insert_into(users::table)
      .values((
        users::username.eq(&self.username),
        users::password.eq(hash_password),
        users::email.eq(&self.email),
        users::last_logined_at.eq(now),
      ))
      .get_result::<User>(conn)
  }
}

impl UpdateUser {
  pub fn save(&self, id: &Uuid, conn: &PgConnection) -> Result<User, DieselError> {
    update(users::table.find(id)).set(self).get_result::<User>(conn)
  }
}

impl LoginUser {
  pub fn find_one(&self, conn: &PgConnection) -> Result<User, DieselError> {
    users::table
      .filter(users::username.eq(&self.username_or_email))
      .or_filter(users::email.eq(&self.username_or_email))
      .get_result::<User>(conn)
  }
  pub fn is_password_matched<'a>(&self, target: &'a str) -> bool {
    verify(&self.password, target).unwrap()
  }
}

impl User {
  pub fn count_users(conn: &PgConnection) -> Result<i64, DieselError> {
    users::table.count().first::<i64>(conn)
  }
  pub fn find_all(page: i64, limit: i64, conn: &PgConnection) -> Result<Vec<User>, DieselError> {
    users::table
      .offset((page - 1) * limit)
      .limit(limit)
      .load::<User>(conn)
  }
  pub fn find_one(id: &Uuid, conn: &PgConnection) -> Result<User, DieselError> {
    users::table.find(id).first::<User>(conn)
  }
  pub fn delete_one(id: &Uuid, conn: &PgConnection) -> Result<usize, DieselError> {
    delete(users::table.find(id)).execute(conn)
  }
}
