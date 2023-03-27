use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::user_model::User;
use crate::models::product_model::Product;

pub struct MongoRepo {
    col: Collection<User>,
    colP: Collection<Product>,
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
        let col: Collection<User> = db.collection("User");
        let colP: Collection<Product> = db.collection("Product");
        MongoRepo { col, colP }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn create_product(&self, new_product: Product) -> Result<InsertOneResult, Error> {
        let new_doc = Product {
            id: None,
            productname: new_product.productname,
            description: new_product.description,
            price: new_product.price,
        };
        let product = self
            .colP
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating product");
        Ok(product)
    }
}