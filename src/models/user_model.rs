use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::location_models::Location;



#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub lastName: String,
    pub firstName: String,
    pub email: String,
    pub telNumber: String,
    pub profilImg: String,
    pub isShop: bool,
    pub shopName: String,
    pub shopDescription: String,
    pub shopAdress: String,
    pub location: Location 

    
}