use rocket::{fairing::{Fairing, Info, Kind}, Request, Response, Data};
use std::time::SystemTime;

pub struct RequestTimer;

struct TimerStart(Option<SystemTime>);

impl Fairing for RequestTimer{
  fn info(&self) -> Info {
    Info {
      name: "Request Timer",
      kind: Kind::Request | Kind::Response,
    }
  }
  fn on_request(&self, request: &mut Request, _: &Data) {
    request.local_cache(||TimerStart(Some(SystemTime::now())));
  }
  fn on_response(&self, request: &Request, response: &mut Response) {
    let start_time = request.local_cache(||TimerStart(None));
    if let Some(Ok(duration)) = start_time.0.map(|st|st.elapsed()) {
      let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
      response.set_raw_header("X-Reponse-Time", format!("{} ms", ms));
    }
  }
}