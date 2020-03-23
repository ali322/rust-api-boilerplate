
pub mod model;
pub mod schema;

use rocket_contrib::databases::diesel;

#[database("aid")]
pub struct Conn(diesel::PgConnection);