use std::ptr::null;

use actix_web::{get, post, web, HttpResponse, Responder};
use crate::modules::profile_management::user::{model::User, service};

use super::service::Status;

#[get("/get-all")]
async fn get_users() -> impl Responder {
    let users = service::fetch_users().await;
    HttpResponse::Ok().json(users)
}

#[get("/auth/{email}/{password}")]
async fn auth(path: web::Path<(String, String)>) -> impl Responder {
    let (email, password) = path.into_inner();
    
    return HttpResponse::Ok().json("");
}

#[post("/add")]
async fn add_user(payload: web::Json<User>) -> impl Responder {
    let status = service::add_user(payload.into_inner()).await;

    match status {
        Status::SUCCESS => HttpResponse::Created().json("success"),
        _ => HttpResponse::InternalServerError().json("failed")
    }
}

