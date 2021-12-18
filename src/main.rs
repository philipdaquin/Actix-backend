mod models;
use crate::models::*;

mod config;
use crate::config::*;

use actix_web::{HttpServer, App, web, Responder};
use dotenv::dotenv;

async fn status() -> impl Responder { 
    web::HttpResponse::Ok()
        .json(Status { 
            status: "OK".to_string()
        })
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();


    println!("Starting a server: {}:{}", config.server.host, config.server.port);

    HttpServer::new(||  { 
        App::new()
        .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.host))?
    .run()
    .await
}
