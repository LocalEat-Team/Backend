mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::user_api::create_user;
use api::product_api::get_all_products;
use repository::mongodb_repo::MongoRepo;
use rocket::{http::{Method, Header}, fairing::{Fairing, Info, Kind}, Request, Response};
use rocket_cors::{AllowedOrigins, CorsOptions};


#[get("/")]
fn index() -> &'static str {
    "Hello, world! Bienvenue sur notre super backend LocalEat ! Cette nouvelle version est déployé grâce à une intégration continue"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init(); 

    rocket::build()
        .manage(db)
        .attach(Cors)
        .mount("/", routes![index])
        .mount("/", routes![create_user])
        .mount("/", routes![get_all_products])
        
}


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}