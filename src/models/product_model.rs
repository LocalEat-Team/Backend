use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub productname: String,
    pub location: String,
    pub description: String,
    pub price: String,
    pub produceradress: String,
    pub shopimg: String
}