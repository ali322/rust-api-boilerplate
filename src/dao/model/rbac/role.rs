use diesel::{Identifiable};
use serde::{Serialize, Deserialize};
use super::domain::Domain;
use crate::dao::schema::roles;

#[derive(Debug, Identifiable, Associations, Queryable, Serialize, Deserialize)]
#[belongs_to(Domain)]
pub struct Role{
  pub id: i32,
  pub domain_id: i32,
  pub name: String,
  pub description: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct NewRole{
  pub domain_id: i32,
  pub name: String,
  pub description: String,
}