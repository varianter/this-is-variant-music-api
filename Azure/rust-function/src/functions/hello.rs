use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
    http::Status,
};

use musicfunc::helper::link_parser::LinkParser;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Data {
    #[serde(rename = "User", skip)]
    user: String,
    #[serde(skip)]
    timestamp: String,
    #[serde(rename = "Text")]
    text: String,
}

#[func]
pub async fn hello(req: HttpRequest) -> HttpResponse {

    if let Ok(data) = req.body().as_json::<Data>() {
        return HttpResponse::build()
            .body("")
            .status(Status::Ok)
            .finish()
    } else {
        HttpResponse::build()
            .body("")
            .status(Status::BadRequest)
            .finish()
    }
}
