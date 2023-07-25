use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::net::TcpListener;
use zero2prod::run;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:8000");
    run(address.unwrap())?.await
}