mod models;
use crate::models::*;

mod handlers;
use crate::handlers::*;

mod config;
use crate::config::Config;

mod db;
//use //crate::db::*;

use dotenv::dotenv;
use tokio_postgres::NoTls;
use actix_web::{HttpServer, App, web};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting a server: {}:{}", config.server.host, config.server.port);

    HttpServer::new(move ||  { 
        App::new()
        .data(pool.clone())
        .route("/", web::get().to(status))
        .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.host))?
    .run()
    .await
}
