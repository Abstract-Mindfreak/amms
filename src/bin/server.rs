use axum::handler::HandlerWithoutStateExt;
use axum::routing::get_service;
use axum::Router;
use mmss::routes;
use mmss::state::AppState;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let state = AppState::initialize(None)?;
    let api_router = routes::build_router().with_state(state.clone());

    let static_service = get_service(ServeDir::new("src/web")).into_service();

    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(static_service)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = std::env::var("MMSS_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = TcpListener::bind(&addr).await?;

    println!("MMSS server listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            let _ = signal::ctrl_c().await;
            println!("Shutting down by signal");
        })
        .await?;

    Ok(())
}
