use infras::web::run;

pub mod app;
pub mod domain;
mod errors;
mod infras;
pub mod present;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    run().await
}
