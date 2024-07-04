use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use std::sync::Arc;
use tokio::{signal::ctrl_c, spawn};
use log::{error, info, LevelFilter};
use tokio::sync::RwLock;
use core::table::Table;
use core::column::Column;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest};

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
        .route("/create", post(create))
        .route("/create_table", post(create_table))
        .route("/drop_table", post(drop_table))
        .route("/update_table", post(update_table))
        .route("/insert_column", post(insert_column))
        .route("/insert_row", post(insert_row))
        .with_state(app_state);

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
    let tables = state.get_all().await;
    let json = Json(tables);
    info!("Tables: {:?}", json);
    json
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRequests>
) -> Result<Json<Table>, String> {
    let table_name = payload.name;

    if state.get(&table_name).await.is_some() {
        return Err(format!("Table '{}' already exists", table_name));
    }

    let new_table = Table {
        name: table_name.clone(),
        columns: Vec::new(),
        rows: Vec::new(),
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

    if state.drop_table(&table_name).await {
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

    if let Some(mut table) = state.get(&current_name).await {
        table.name = new_name.clone();
        state.drop_table(&current_name).await;
        state.create(table).await;
        info!("Updated table name from '{}' to '{}'", current_name, new_name);
        Ok(Json(format!("Updated table name from '{}' to '{}'", current_name, new_name)))
    } else {
        Err(format!("Table '{}' does not exist", current_name))
    }
}

async fn insert_column(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<InsertColumnRequest>
) -> Result<Json<Column>, String> {
    let table_name = payload.table_name;

    if let Some(mut table) = state.get(&table_name).await {
        let column = Column::new(
            payload.key,
            payload.primary_key,
            payload.non_null,
            payload.unique,
            payload.foreign_key.map(|fk| fk.into_iter().map(Box::new).collect()),
        );
        table.add_column(column.clone());
        state.drop_table(&table_name).await;
        state.create(table).await;
        info!("Inserted column into table '{}': {:?}", table_name, column);
        Ok(Json(column))
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

async fn create_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequests>
) -> Result<Json<Table>, String> {
    let table_name = payload.name;

    if state.get(&table_name).await.is_some() {
        return Err(format!("Table '{}' already exists", table_name));
    }

    let mut new_table = Table {
        name: table_name.clone(),
        columns: Vec::new(),
        rows: Vec::new(),
    };

    for insert_column_request in &payload.insert_column_requests {
        let column = Column::new(
            insert_column_request.key.clone(),
            insert_column_request.primary_key,
            insert_column_request.non_null,
            insert_column_request.unique,
            insert_column_request.foreign_key.clone().map(|fk| fk.into_iter().map(Box::new).collect()),
        );
        new_table.add_column(column);
    }

    state.create(new_table.clone()).await;

    info!("Created table: {:?}", new_table);
    Ok(Json(new_table))
}

async fn insert_row(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<InsertRowRequest>
) -> Result<Json<Vec<String>>, String> {
    let table_name = payload.table_name;

    if let Some(mut table) = state.get(&table_name).await {
        let row = payload.row;

        let columns_len = table.columns.len();
        if row.values.len() > columns_len {
            return Err(format!("Row has too many values ({}), expected {}", row.values.len(), columns_len));
        }

        table.add_row(row.clone());
        state.drop_table(&table_name).await;
        state.create(table).await;
        info!("Inserted row into table '{}': {:?}", table_name, row);
        Ok(Json(row.values.into_iter().map(|cell| cell.value).collect()))
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
        lock.iter().cloned().collect()
    }

    pub async fn get(&self, table_name: &str) -> Option<Table> {
        let lock = self.tables.read().await;
        lock.iter().find(|table| table.name == table_name).cloned()
    }

    pub async fn drop_table(&self, table_name: &str) -> bool {
        let mut lock = self.tables.write().await;
        if let Some(index) = lock.iter().position(|table| table.name == table_name) {
            lock.remove(index);
            true
        } else {
            false
        }
    }
}
