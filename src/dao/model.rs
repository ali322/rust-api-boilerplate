use chrono::{prelude::*, Local};
use diesel::{
  insert_into, prelude::*, result::Error as DieselError, PgConnection, Queryable,
};
use rocket::request::FromForm;
use serde::{Deserialize, Serialize};
use validator::Validate;
use super::schema::users;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
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

impl NewUser {
  pub fn is_valid_username(&self, conn: &PgConnection) -> bool {
    users::table
      .filter(users::username.eq(&self.username))
      .first::<User>(conn)
      .is_ok()
  }
  pub fn create(&self, conn: &PgConnection) -> Result<User, DieselError> {
    let now = Local::now().naive_local();
    insert_into(users::table)
      .values((
        users::username.eq(&self.username),
        users::password.eq(&self.password),
        users::email.eq(&self.email),
        users::last_logined_at.eq(now),
      ))
      .get_result::<User>(&*conn)
  }
}
