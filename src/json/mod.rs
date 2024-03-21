use std::collections::HashMap;

pub enum Json {
  Bool(bool),
  Int(i32),
  Float(f64),
  String(String),
  Map(HashMap<String, Json>),
  Vec(Vec<Json>),
  Null,
}
