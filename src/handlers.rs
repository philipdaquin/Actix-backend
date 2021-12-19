use actix_web::{Responder, web};
use crate::models::Status;

pub async fn status() -> impl Responder { 
    web::HttpResponse::Ok()
        .json(Status { 
            status: "OK".to_string()
        })
}