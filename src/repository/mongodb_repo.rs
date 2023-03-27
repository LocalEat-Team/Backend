use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::{user_model::User, product_model::Product};

pub struct MongoRepo {
    col_user: Collection<User>,
    col_product: Collection<Product>
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
        let col_user: Collection<User> = db.collection("User");
        let col_product: Collection<Product> = db.collection("Product");
        MongoRepo { col_user, col_product }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col_user
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_products(&self) -> Result<Vec<Product>, Error> {

        let cursors = self
            .col_product
            .find(None, None)
            .ok()
            .expect("Error getting list of products");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}