use tutor_backend::server;

#[actix_web::main]
async fn main() {
    server::start_server().await;
}


