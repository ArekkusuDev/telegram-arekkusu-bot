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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JikanResponse {
    pub data: Vec<Datum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datum {
    pub mal_id: i64,
    pub images: HashMap<ImageExtension, Image>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub episodes: i64,
    pub status: Status,
    pub aired: Aired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum ImageExtension {
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "webp")]
    Webp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aired {
    pub string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Finished Airing")]
    FinishedAiring,
    #[serde(rename = "Currently Airing")]
    Airing,
    #[serde(rename = "Not yet aired")]
    NotYetAired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Season {
    #[serde(rename = "winter")]
    Winter,
    #[serde(rename = "spring")]
    Spring,
    #[serde(rename = "summer")]
    Summer,
    #[serde(rename = "fall")]
    Fall,
}
