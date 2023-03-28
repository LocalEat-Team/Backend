use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Serialize, Deserialize};
use rocket::serde::json::serde_json::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub Location_type: String,
    pub Location_coordinates : Vec<Bson>
}