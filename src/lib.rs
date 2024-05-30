use std::io;
use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

pub mod configuration;
pub mod routes;
pub mod startup;


#[derive(serde::Deserialize)]
pub struct FormData{
    name : String,
    email : String
}

async fn health_check() -> HttpResponse{
    HttpResponse::Ok().finish()
}

async fn handle_subscriptions(form_data: web::Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub  fn run(listener : TcpListener) -> Result<Server,io::Error>{
    let server = HttpServer:: new(||{
        App::new()
            .route("/healthcheck", web::get().to(health_check))
            .route("/subscriptions",web::post().to(handle_subscriptions))
    }).listen(listener)?
    .run();
    Ok(server)
}