use rocket::request::Request;

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> &'static str {
  "unprocessable entity when parse request"
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> &'static str {
  "unauthorized request"
}

#[catch(404)]
pub fn not_found(_req: &Request) -> &'static str {
  "request not found"
}