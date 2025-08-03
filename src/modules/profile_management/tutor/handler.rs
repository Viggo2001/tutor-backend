use actix_web::{get, put, web, HttpResponse, Responder};

use crate::modules::profile_management::tutor::{model::PartialTutor, service};

#[get("/find_by_id/{id}")]
async fn find_by_id(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    let tutor = service::find_by_id(id.as_str()).await;

    match tutor {
        Some(tutor) => {
            HttpResponse::Ok().json(tutor)
        },
        None => {
            HttpResponse::InternalServerError().json({})
        }
    }
}

#[put("/update")]
async fn update_tutor(request: web::Json<PartialTutor>) -> impl Responder {
    let result = service::update(request.into_inner()).await;
    
    HttpResponse::Ok().json(result)
}
