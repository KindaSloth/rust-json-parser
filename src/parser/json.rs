use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    JsonNull,
    JsonBool(bool),
    JsonNumber(f32),
    JsonString(String),
    JsonArray(Vec<Json>),
    JsonObject(HashMap<String, Json>),
}
