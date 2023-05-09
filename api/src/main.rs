use actix_multipart::Multipart;
use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::env;
mod config;
use config::Config;

#[post("/abis")]
async fn upload_abis(mut payload: Multipart, req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Okay")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = Config::new();

    dotenv::from_filename(config.dotenv_path).ok();

    let host = env::var("API_HOST").expect("API_HOST must be set");
    let port = env::var("API_PORT")
        .expect("API_PORT must be set")
        .parse()
        .unwrap();

    HttpServer::new(|| App::new().service(ping).service(upload_abis))
        .bind((host, port))?
        .run()
        .await
}
