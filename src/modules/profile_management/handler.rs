use actix_web::{get, web, HttpResponse, Responder};
use crate::modules::profile_management::service;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let users = service::fetch_users().await;
    HttpResponse::Ok().json(users)
}

