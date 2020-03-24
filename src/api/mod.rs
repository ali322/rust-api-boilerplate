use rocket_contrib::json::Json;
use serde_json::Value;

type APIResult = Result<Json<Value>, Json<Value>>;

macro_rules! response {
  ($val:tt) => {
    Json(json!({
      "code": 0,
      "data" : $val
    }))
  };
  ($val:expr, $code:expr) => {
    Json(json!({
      "code": $code,
      "message" : $val
    }))
  };
}

macro_rules! validate_error {
  ($val:expr) => {
    $val.field_errors()
      .iter()
      .map(|(k, v)| {
        let errors: Vec<String> = v
          .iter()
          .map(|e| {
            if let Some(msg) = e.message.as_ref() {
              return msg.to_string();
            }
            return format!("{} is invalid", e.code.to_string());
          })
          .collect();
        (*k, errors)
      })
      .collect::<HashMap<_, _>>()
  };
}

pub mod auth;
pub mod error;
