use mongodb::bson::oid::ObjectId;
use rocket::serde::json::serde_json::Number;
use serde::{Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub productname: String,
    pub description: String,
    pub price: Number,
    // pub shopimg: String,
    // pub location: Object,
    // pub produceradress: String,
}