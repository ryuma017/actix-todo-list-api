use actix_web::{self, middleware::Logger, App, HttpServer};

use todo_list_api::routes::health_check;

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
