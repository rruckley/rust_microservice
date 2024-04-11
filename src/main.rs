//! Rust Template for microservices
pub mod handlers;
pub mod payloads;
pub mod error;

use handlers::handle_post_request;

use actix_web::{App,HttpServer};
use actix_web::middleware::Logger;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    info!("Starting {pkg} v{ver}");
    let port = String::from("8000");
    let port = port.parse::<u16>().unwrap();

    HttpServer::new(move || {
        App::new()
            .service(handle_post_request)
            // Data source can be passed in here, must impl sync for threading support.
            //.app_data(ext)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0",port))?
        .run()
        .await
}
