use std::io;
use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

async fn health_check() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub  fn run(listener : TcpListener) -> Result<Server,io::Error>{
    let server = HttpServer:: new(||{
        App::new()
          .route("/healthcheck", web::get().to(health_check))
    }).listen(listener)?
    .run();
    Ok(server)
}