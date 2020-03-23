use chrono::prelude::*;
use diesel::{Queryable};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User{
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub avatar: Option<String>,
  pub memo: Option<String>,
  pub last_logined_at: NaiveDateTime,
}