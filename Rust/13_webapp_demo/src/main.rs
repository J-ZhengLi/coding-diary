use actix_web::{get, web, App, HttpServer, Responder};

const ADDR: &str = "localhost";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind((ADDR, PORT))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}
