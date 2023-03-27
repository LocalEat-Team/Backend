use crate::{models::product_model::Product, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};


#[get("/items/products")]
pub fn get_all_products(db: &State<MongoRepo>) -> Result<Json<Vec<Product>>, Status> {
    let products = db.get_products();
    match products {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(Status::InternalServerError),
    }
}