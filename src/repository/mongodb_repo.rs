use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::{user_model::User, product_model::Product};

pub struct MongoRepo {
    colUser: Collection<User>,
    colProduct: Collection<Product>
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let colUser: Collection<User> = db.collection("User");
        let colProduct: Collection<Product> = db.collection("Product");
        MongoRepo { colUser, colProduct }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .colUser
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {

        let cursors = self
            .colProduct
            .find(None, None)
            .ok()
            .expect("Error getting list of products");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}