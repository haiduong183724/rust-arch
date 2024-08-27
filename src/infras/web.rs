use actix_web::{middleware::Logger, web, HttpServer};
use actix_web_lab::middleware::from_fn;
use log::info;

use crate::present::{middlewares, routes::user_routes};

use super::repo::user_repo::PgUserRepo;

pub async fn run() -> std::io::Result<()> {
    let repo = PgUserRepo::new();
    let app_data = web::Data::new(repo);
    let SERVER = "0.0.0.0:9999";
    info!("Starting server...{}", SERVER);

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap(from_fn(middlewares::auth_middleware::auth_middleware))
            .configure(user_routes::routes)
    })
    .bind(SERVER)
    .unwrap()
    .run()
    .await
}
