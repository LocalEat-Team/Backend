use crate::{models::{user_model::User, location_models::Location}, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let loc;
    let data;

    if new_user.isShop == false {

        loc = Location {
            Location_type: new_user.isShop.to_owned().to_string(),
            Location_coordinates: vec![new_user.isShop.to_owned().to_string(), new_user.isShop.to_owned().to_string()]
       };

        data = User {
            id: None,
            lastName: new_user.lastName.to_owned(),
            firstName: new_user.firstName.to_owned(),
            email: new_user.email.to_owned(),
            telNumber: new_user.telNumber.to_owned(),
            profilImg: new_user.profilImg.to_owned(),
            isShop: new_user.isShop.to_owned(),
            shopDescription: new_user.isShop.to_owned().to_string(),
            shopName: new_user.isShop.to_owned().to_string(),
            shopAdress: new_user.isShop.to_owned().to_string(),
            location: loc
        };
        
    } else {
        
        loc = Location {
            Location_type: new_user.location.Location_type.to_owned(),
            Location_coordinates: new_user.location.Location_coordinates.to_owned()
       };
    
    
        data = User {
            id: None,
            lastName: String::from("None"),
            firstName: String::from("None"),
            email: String::from("None"),
            telNumber: String::from("None"),
            profilImg: String::from("None"),
            isShop: new_user.isShop.to_owned(),
            shopName: new_user.shopName.to_owned(),
            shopDescription: new_user.shopDescription.to_owned(),
            shopAdress: new_user.shopAdress.to_owned(),
            location: loc
        };
    }

    
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}



#[get("/items/shops")]
pub fn get_all_shops(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let shops = db.get_shops();
    match shops {
        Ok(shop) => Ok(Json(shop)),
        Err(_) => Err(Status::InternalServerError),
    }
}
