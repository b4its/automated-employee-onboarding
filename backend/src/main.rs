mod application;
mod domain;
mod infrastructur;
mod interface;

use application::AppService;
use infrastructur::database::PostgresRepository;
use interface::controllers::{create_router, AppState};

use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env untuk local dev
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Dependency Injection
    let repo = PostgresRepository::new(pool);
    let arc_repo = Arc::new(repo);
    
    // Inject repository yang sama untuk User dan Onboarding
    let service = AppService::new(arc_repo.clone(), arc_repo.clone());
    let state = AppState {
        service: Arc::new(service),
    };

    let app = create_router(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 Server running on http://0.0.0.0:8080");
    
    axum::serve(listener, app).await?;

    Ok(())
}