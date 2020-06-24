#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::json::{Json, JsonError};
use serde::{Serialize, Deserialize};

extern crate nanoid;
use nanoid::nanoid;


#[derive(Debug, PartialEq, Eq, Deserialize)]
struct UrlNanoRequest {
    url: String,
    password: Option<String>,
    slug: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
   Code: u16,
   Response: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    Code: u16,
    Response: String
}



#[post("/create", format="json", data="<payload>")]
fn nano_request(mut payload: Result<Json<UrlNanoRequest>, JsonError>) -> String {
    let mut inner_payload: Json<UrlNanoRequest> = match payload {
        Ok(payload) => payload,
        Err(e) => return "Something went wrong".to_string(),
    };

    if inner_payload.slug == None {
        inner_payload.slug = Some(nanoid!(5).to_string())
    }

    format!("{:?}", inner_payload) 
}

fn main() {
    rocket::ignite().mount("/", routes![nano_request])
                    .launch();
}
