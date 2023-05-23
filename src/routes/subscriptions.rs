//! src/routes/subscriptions.rs
use actix_web::HttpResponse;
use actix_web::web::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubDetails{
    name: String,
    email: String
}

pub async fn subscribe(form: Form<SubDetails>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
