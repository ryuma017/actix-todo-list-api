use actix_web::{self, get, middleware::Logger, App, HttpResponse, HttpServer};

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
