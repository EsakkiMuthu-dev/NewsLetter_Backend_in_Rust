use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe};

pub fn run(listener : TcpListener , connection_pool:PgPool) -> Result<Server , std::io::Error>{
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            // middle ware for logger
            .wrap(TracingLogger::default())
            .route("/health_check",web::get().to(health_check))
            .route("/subscribe",web::post().to(subscribe))
            .app_data(connection_pool.clone())
    }).listen(listener)?
        .run();
    Ok(server)
}