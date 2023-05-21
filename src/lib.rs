use std::net::TcpListener;
use serde::Deserialize;
use actix_web::{web::{self, Form}, App, HttpResponse, HttpServer, HttpRequest, Responder, dev::Server};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[derive(Deserialize)]
struct SubDetails{
    name: String,
    email: String
} 

async fn subscribe(form: Form<SubDetails>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();

    Ok(server)
}