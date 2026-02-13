use crate::config::root::configuration::Configuration;

mod api;
mod config;
mod database;

#[tokio::main]
async fn main() {
    let config = match Configuration::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Invalid configuration: {}", err);
            std::process::exit(1);
        }
    };

    let pool = match database::connection::create_pool(&config.database).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to create database pool: {}", err);
            std::process::exit(1);
        }
    };

    let router = api::router::todo_router::routes(pool);

    let addr = format!("0.0.0.0:{}", config.app.service_port());
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind to {}: {}", addr, err);
            std::process::exit(1);
        }
    };

    println!(
        "Server {} running on port {}",
        config.app.service_name(),
        config.app.service_port()
    );

    if let Err(err) = axum::serve(listener, router).await {
        eprintln!("Server error: {}", err);
        std::process::exit(1);
    }
}
