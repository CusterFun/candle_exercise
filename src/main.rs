use std::net::SocketAddr;

use axum::routing::post;
use axum::{routing::get, Router};
use tokio::signal::{unix, unix::SignalKind};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use exercise_screen::app_state::SharedState;
use exercise_screen::send_frame::{start_camera, stop_camera, ws_handler};

#[tokio::main]
async fn main() {
    // open_camera();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "exercise_screen=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = SharedState::default();

    let app = Router::new()
        .route("/start_camera", post(start_camera))
        .route("/stop_camera", post(stop_camera))
        .route("/ws", get(ws_handler))
        .with_state(std::sync::Arc::clone(&shared_state));

    // run it with hyer
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {addr:?}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
