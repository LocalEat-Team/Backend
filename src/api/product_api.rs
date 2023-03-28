use crate::{models::{product_model::Product, location_models::Location}, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};
use mongodb::results::InsertOneResult;

#[get("/items/products")]
pub fn get_all_products(db: &State<MongoRepo>) -> Result<Json<Vec<Product>>, Status> {
    let products = db.get_products();
    match products {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(Status::InternalServerError),
    }
}



#[post("/items/products", data = "<new_product>")]
pub fn create_product(
    db: &State<MongoRepo>,
    new_product: Json<Product>,
) -> Result<Json<InsertOneResult>, Status> {
    let loc = Location {
         Location_type: new_product.location.Location_type.to_owned(),
         Location_coordinates: new_product.location.Location_coordinates.to_owned()
    };

    let data = Product {
        id: None,
        productname: new_product.productname.to_owned(),
        location:loc,
        description: new_product.description.to_owned(),
        price: new_product.price.to_owned(),
        produceradress: new_product.produceradress.to_owned(),
        shopimg: new_product.shopimg.to_owned()
    };
    let product_detail = db.create_product(data);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}