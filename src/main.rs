mod db;
mod utils;
mod upload_file;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use db::setup::setup;
use upload_file::upload_file;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {

    match setup() {
        Ok(_) => HttpResponse::Ok().body("Hey there!"),
        Err(err) => HttpResponse::Ok().body(format!("Error:, {}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/upload", web::post().to(upload_file))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}