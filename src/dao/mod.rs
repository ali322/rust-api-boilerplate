
pub mod model;
pub mod schema;

use rocket_contrib::databases::diesel;

#[database("main")]
pub struct Conn(diesel::PgConnection);