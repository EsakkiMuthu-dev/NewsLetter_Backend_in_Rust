use std::io;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

async fn health_check() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub  fn run() -> Result<Server,io::Error>{
    let server = HttpServer:: new(||{
        App::new()
          .route("/healthcheck", web::get().to(health_check))
    }).bind("127.0.0.1:8000")?
    .run();
    Ok(server)
}