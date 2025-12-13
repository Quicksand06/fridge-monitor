mod models;
mod routes;

use axum::Router;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing::error;
use tracing::log::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // set up logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // respects RUST_LOG
        .init();

    info!("Starting the application...");

    let db_url = env::var("DATABASE_URL")?;

    // Postgres pool
    let db = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {e}");
            std::process::exit(1);
        }
    };

    // Run migrations automatically
    if let Err(e) = sqlx::migrate!("./migrations").run(&db).await {
        error!("Failed to run migrations: {}", e);
        std::process::exit(1);
    }

    info!("Migrations applied successfully");

    // build routes
    let app = Router::<PgPool>::new()
        .merge(routes::health::routes())
        .merge(routes::fridge::routes())
        .with_state(db);

    // address & port
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    let ip = env::var("HOST_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr: SocketAddr = format!("{}:{}", ip, port)
        .parse()
        .expect("Invalid IP or port");
    println!("listening on http://{}", addr);

    // run server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
