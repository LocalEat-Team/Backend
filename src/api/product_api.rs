
extern crate rocket;
use crate::{models::{product_model::Product}, repository::mongodb_repo::MongoRepo};
use reqwest::blocking::ClientBuilder;
use rocket::{http::{Status}, serde::json::{Json, serde_json}, State, response::status::BadRequest, time::Duration};
use mongodb::results::InsertOneResult;
use rocket::{get, response::status};
use serde::Deserialize;


#[get("/items/products")]
pub fn get_all_products(db: &State<MongoRepo>) -> Result<Json<Vec<Product>>, Status> {
    let products = db.get_products();
    match products {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

// #[get("/items/productsinfo/<codeBar>")]
// pub async get_info_products(db: &State<MongoRepo>, codeBar: &str) -> _ {

//         let body = reqwest::blocking::get("https://www.rust-lang.org")?
//         .text()?;

//         println!("body = {:?}", body);
//     }


#[post("/items/products", data = "<new_product>")]
pub fn create_product(
    db: &State<MongoRepo>,
    new_product: Json<Product>,
) -> Result<Json<InsertOneResult>, Status> {

    let data = Product {
        id: None,
        productname: new_product.productname.to_owned(),
        description: new_product.description.to_owned(),
        price: new_product.price.to_owned(),
        produceradress: new_product.produceradress.to_owned(),
        productimg : new_product.productimg.to_owned()
    };
    let product_detail = db.create_product(data);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}