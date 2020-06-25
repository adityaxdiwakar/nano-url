#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;

use rocket_contrib::json::{Json, JsonError};
use rocket::response::{self, Response, Responder};
use rocket::request::{self, Request};
use rocket::http::{ContentType, Status};
use serde::{Serialize, Deserialize};
use std::result::Result;

extern crate nanoid;
use nanoid::nanoid;


#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct UrlNanoPayload {
    url: String,
    password: Option<String>,
    slug: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    response: String
}

struct SuccessResponse<T> {
    payload: T,
    status: Status
}

type ApiResponse<T> = Result<SuccessResponse<Json<T>>, ErrorResponse>;


impl<'r> Responder<'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            Ok(json) => Response::build_from(self.json.respond_to(req).unwrap())
                .status(Status::Ok),
        }
    }
}

// nano_request will shorten a link, provided a URL is in the JSON body
// TODO: Change from "String" response type, respond with Result<ApiResponse, ErrorResponse>
// Implement Responder for Result<> mentioned above
#[rocket::post("/create", format="json", data="<payload>")]
fn nano_request(payload: Result<Json<UrlNanoPayload>, JsonError>) -> ApiResponse<UrlNanoPayload> {
    
    // Handles deserialization errors (Using rocket__contrib::json::JsonError)
    let mut inner_payload: Json<UrlNanoPayload> = match payload {
        Ok(payload) => payload,
        Err(e) => return Err(ErrorResponse{code: 400, response: format!{"{:?}", e}})
    };

    if inner_payload.slug == None {
        inner_payload.slug = Some(nanoid!(5).to_string())
    }

    inner_payload
}


fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![nano_request])
        .launch();
}
