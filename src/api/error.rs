use rocket::request::Request;

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> &'static str {
  "unprocessable entity when parse request"
}