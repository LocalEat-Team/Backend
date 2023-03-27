mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::user_api::create_user;
use api::product_api::create_product;
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! Bienvenue sur notre super backend LocalEat ! Cette nouvelle version est déployé grâce à une intégration continue"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/", routes![create_user])
        .mount("/", routes![create_product])
        
}