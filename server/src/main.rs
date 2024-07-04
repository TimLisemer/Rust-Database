use axum::{routing::get, Router, Json, extract::State};
use std::sync::{Arc, Mutex};
use tokio::{signal::ctrl_c, spawn};
use log::{error, info, LevelFilter};
use core::table::Table;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let app_state = AppState {
        table: Arc::new(Mutex::new(Table::new("Users".to_string()))),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/table", get(get_table))
        .with_state(app_state.clone()); // Clone app_state for the server task

    let address = "0.0.0.0:3000";
    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => {
            info!("Http service started running on http://{}", address);
            listener
        }
        Err(err) => {
            error!("Error: {}", err);
            return;
        }
    };

    let server_task = spawn(async move {
        if let Err(err) = axum::serve(listener, app).await {
            error!("Server error: {}", err);
        }
    });

    // Handle Ctrl+C (SIGINT) to gracefully shutdown the server
    let _ = tokio::spawn(async {
        ctrl_c().await.expect("Failed to listen for Ctrl+C");
    }).await;

    // Wait for server task to finish (though it should run indefinitely until SIGINT)
    if let Err(err) = server_task.await {
        error!("Server task error: {}", err);
    }
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_table(State(state): State<AppState>) -> Json<Table> {
    let table = state.table.lock().unwrap();
    Json(table.clone())
}

#[derive(Clone)]
struct AppState {
    table: Arc<Mutex<Table>>,
}
