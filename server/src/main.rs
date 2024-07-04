use axum::{routing::get, Router, Json, extract::State};
use std::sync::{Arc, Mutex};
use tokio::{signal::ctrl_c, spawn};
use log::{error, info, LevelFilter};
use core::table::Table;
use core::entry::Entry;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    // Create tables and populate them
    let table1 = create_table_users();
    let table2 = create_table_products();

    let app_state = AppState {
        tables: vec![
            Arc::new(Mutex::new(table1)),
            Arc::new(Mutex::new(table2)),
        ],
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/tables", get(get_tables))
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

    // Handle Ctrl+C (SIGINT) to gracefully shut down the server
    let _ = spawn(async {
        ctrl_c().await.expect("Failed to listen for Ctrl+C");
    }).await;

    // Wait for server task to finish (though it should run indefinitely until SIGINT)
    if let Err(err) = server_task.await {
        error!("Server task error: {}", err);
    }
}

fn create_table_users() -> Table {
    let mut table = Table::new("Users".to_string());

    let entry1 = Entry::new("id".to_string(), "1".to_string(), true, true, true, None);
    let entry2 = Entry::new("name".to_string(), "Alice".to_string(), false, true, false, None);
    let entry3 = Entry::new("email".to_string(), "alice@example.com".to_string(), false, true, true, None);

    table.add_entry(entry1);
    table.add_entry(entry2);
    table.add_entry(entry3);

    table
}

fn create_table_products() -> Table {
    let mut table = Table::new("Products".to_string());

    let entry4 = Entry::new("id".to_string(), "2".to_string(), true, true, true, None);

    table.add_entry(entry4);

    table
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_tables(State(state): State<AppState>) -> Json<Vec<Table>> {
    let tables = state.tables.iter()
        .map(|table| {
            let table = table.lock().unwrap();
            table.clone()
        })
        .collect();

    Json(tables)
}

#[derive(Clone)]
struct AppState {
    tables: Vec<Arc<Mutex<Table>>>,
}
