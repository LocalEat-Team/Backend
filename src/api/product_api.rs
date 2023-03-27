use crate::{models::product_model::Product, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/product", data = "<new_product>")]
pub fn create_product(
    db: &State<MongoRepo>,
    new_product: Json<Product>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Product {
        id: None,
        productname: new_product.productname.to_owned(),
        description: new_product.description.to_owned(),
        price: new_product.price.to_owned(),
    };
    let product_detail = db.create_product(data);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}