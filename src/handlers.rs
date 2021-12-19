use actix_web::{Responder, web, HttpResponse};
use crate::models::{Status, CreateToDoList, ResultResponse};
use deadpool_postgres::{Pool, Client};
use crate::db;
pub async fn status() -> impl Responder { 
    web::HttpResponse::Ok()
        .json(Status { 
            status: "OK".to_string()
        })
}
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder { 

    let client: Client = db_pool
        .get()
        .await
        .expect("Failed to connect to database");
    let result = db::get_todos(&client).await;

    result  
        .map(|response| HttpResponse::Ok().json(response))
        .map_err(|_| HttpResponse::InternalServerError())
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder { 
    let client: Client = db_pool    
        .get()
        .await
        .expect("Could not get user items");
    let result = db::get_items(&client, path.0).await;

    result  
        .map(|response| HttpResponse::Ok().json(response))
        .map_err(|_| HttpResponse::InternalServerError())
}

pub async fn create_items(
    db_pool: web::Data<Pool>, 
    json: web::Json<CreateToDoList>
) -> impl Responder { 
    let client: Client = db_pool    
        .get()
        .await
        .expect("Could not get user items");
    let result = db::create_items(&client, json.title.clone()).await;
    
    result  
        .map(|response| HttpResponse::Ok().json(response))
        .map_err(|_| HttpResponse::InternalServerError())
}

pub async fn check_item(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>
) -> impl Responder { 
    let client: Client = db_pool    
        .get()
        .await
        .expect("Could not get user items");
    let result = db::check_item(&client, path.0, path.1).await;
    
    result  
        .map(|_| HttpResponse::Ok().json(ResultResponse {
            sucess: true
        }))
        .map_err(|_| HttpResponse::InternalServerError())
}