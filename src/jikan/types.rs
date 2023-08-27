// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::JikanResponse;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: JikanResponse = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JikanResponse {
    pub data: Vec<Datum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datum {
    pub mal_id: i64,
    pub images: HashMap<String, Image>,
    pub approved: bool,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}
