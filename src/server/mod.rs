use actix_web::{App, HttpServer};
use crate::modules::profile_management::handler::configure_user_routes;

pub async fn start_server() {
    HttpServer::new(|| {
        App::new()
            .configure(configure_user_routes)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed to bind server")
    .run()
    .await
    .unwrap();
}
