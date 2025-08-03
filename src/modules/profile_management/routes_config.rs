use actix_web::{web};

use crate::modules::profile_management::{tutor::handler::{find_by_id, update_tutor}, user::handler::{add_user, get_users}};

static SERVICE_PATH: &str = "profile_management";

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    let user_path = "user";

    cfg.service(
        web::scope(format!("{}/{}", SERVICE_PATH, user_path).as_str())
            .service(get_users)
            .service(add_user),
        );
}

pub fn configure_tutor_routes(cfg: &mut web::ServiceConfig) {
    let tutor_path = "tutor";

    cfg.service(
        web::scope(format!("{}/{}", SERVICE_PATH, tutor_path).as_str())
            .service(find_by_id)
            .service(update_tutor),
    );
}

