use rocket::serde::json::{Value, json};

pub fn home() -> Value {
  json!({"message":String::from("Chad rust server"),"body":{}})
}
