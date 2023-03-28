use std::env;
extern crate dotenv;
use dotenv::dotenv;

use geojson::GeoJson;
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
            lastName: new_user.lastName.to_owned(),
            firstName: new_user.firstName.to_owned(),
            email: new_user.email.to_owned(),
            telNumber: new_user.telNumber.to_owned(),
            profilImg: new_user.profilImg.to_owned(),
            isShop: new_user.isShop.to_owned(),
            shopName: new_user.shopName.to_owned(),
            shopDescription: new_user.shopDescription.to_owned(),
            shopAdress: new_user.shopAdress.to_owned(),
            location: new_user.location
        };
        let user = self
            .col_user
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_shops(&self) -> Result<Vec<User>, Error> {

        let cursor = self
            .col_user
            .find(doc!{"isShop": true}, None)
            .ok()
            .expect("Error getting list of products");

        let users = cursor.map(|doc| doc.unwrap()).collect();
        Ok(users)
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


    pub fn create_product(&self, new_product: Product) -> Result<InsertOneResult, Error> {
        let new_doc = Product {
            id: None,
            productname: new_product.productname,
            description: new_product.description,
            price: new_product.price,
            location: new_product.location,
            produceradress: new_product.produceradress
           
        };
        let product = self
            .col_product
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating product");
        Ok(product)

    }
}