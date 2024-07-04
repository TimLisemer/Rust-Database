use axum::{
    routing::{get, post},
    Router, Json, extract::{State, Path},
};
use std::sync::{Arc, Mutex};
use tokio::{signal::ctrl_c, spawn};
use log::{error, info, LevelFilter};
use tokio::sync::RwLock;
use serde::Deserialize;
use core::table::Table;
use core::entry::Entry;
use core::request_types::{CreateTableRequest, DropTableRequest, UpdateTableRequest, InsertEntryRequest};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let app_state: Arc<AppState> = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(root))
        .route("/tables", get(get_tables))
        .route("/create", post(create_table))
        .route("/drop_table", post(drop_table))
        .route("/update_table", post(update_table))
        .route("/insert_entry", post(insert_entry))
        .with_state(app_state); // Clone app_state for the server task

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

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_tables(State(state): State<Arc<AppState>>) -> Json<Vec<Table>> {
    let tables = state.get_all().await.iter()
        .map(|table| table.clone()).collect();

    let json = Json(tables);
    info!("Tables: {:?}", json);
    json
}

async fn create_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequest>
) -> Result<Json<Table>, String> {
    let table_name = payload.name;

    if state.get(table_name.clone()).await.is_some() {
        return Err(format!("Table '{}' already exists", table_name));
    }

    let new_table = Table {
        name: table_name.clone(),
        entries: Vec::new(),
    };

    state.create(new_table.clone()).await;

    info!("Created table: {:?}", new_table);
    Ok(Json(new_table))
}

async fn drop_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DropTableRequest>
) -> Result<Json<String>, String> {
    let table_name = payload.name;

    if state.drop_table(table_name.clone()).await {
        info!("Dropped table: {}", table_name);
        Ok(Json(format!("Dropped table '{}'", table_name)))
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

async fn update_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTableRequest>
) -> Result<Json<String>, String> {
    let current_name = payload.current_name;
    let new_name = payload.new_name;

    if let Some(mut table) = state.get(current_name.clone()).await {
        table.name = new_name.clone();
        state.drop_table(current_name.clone()).await;
        state.create(table).await;
        info!("Updated table name from '{}' to '{}'", current_name, new_name);
        Ok(Json(format!("Updated table name from '{}' to '{}'", current_name, new_name)))
    } else {
        Err(format!("Table '{}' does not exist", current_name))
    }
}

async fn insert_entry(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<InsertEntryRequest>
) -> Result<Json<Entry>, String> {
    let table_name = payload.table_name;

    if let Some(mut table) = state.get(table_name.clone()).await {
        let entry = Entry::new(
            payload.key,
            payload.value,
            payload.primary_key,
            payload.non_null,
            payload.unique,
            payload.foreign_key.map(|fk| fk.into_iter().map(Box::new).collect()),
        );
        table.add_entry(entry.clone());
        state.drop_table(table_name.clone()).await;
        state.create(table).await;
        info!("Inserted entry into table '{}': {:?}", table_name, entry);
        Ok(Json(entry))
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

#[derive(Clone)]
struct AppState {
    tables: Arc<RwLock<Vec<Table>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            tables: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn create(&self, table: Table) {
        let mut lock = self.tables.write().await;
        lock.push(table);
    }

    pub async fn get_all(&self) -> Vec<Table> {
        let lock = self.tables.read().await;
        lock.iter().map(|table| table.clone()).collect()
    }

    pub async fn get(&self, table_name: String) -> Option<Table> {
        let lock = self.tables.read().await;
        lock.iter().find(|table| table.name == table_name).cloned()
    }

    pub async fn drop_table(&self, table_name: String) -> bool {
        let mut lock = self.tables.write().await;
        if let Some(index) = lock.iter().position(|table| table.name == table_name) {
            lock.remove(index);
            true
        } else {
            false
        }
    }
}
