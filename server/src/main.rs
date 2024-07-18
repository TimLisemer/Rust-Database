use std::io::Error;
use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use std::sync::Arc;
use tokio::{signal::ctrl_c, spawn};
use log::{error, info, LevelFilter};
use tokio::sync::RwLock;
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader};
use core::{
    table::Table,
    column::Column,
    request_types::{
        CreateRequests, CreateTableRequests, DropTableRequest,
        UpdateTableRequest, InsertColumnRequest, InsertRowRequest,
    },
};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    // Load application state from file or create new state
    let app_state: Arc<AppState> = Arc::new(AppState::load().await.unwrap_or_else(|_| AppState::new()));

    // Define routes and handlers
    let app = Router::new()
        .route("/", get(root))
        .route("/tables", get(get_tables))
        .route("/create", post(create))
        .route("/create_table", post(create_table))
        .route("/drop_table", post(drop_table))
        .route("/update_table", post(update_table))
        .route("/insert_column", post(insert_column))
        .route("/insert_row", post(insert_row))
        .with_state(app_state.clone());

    // Start HTTP server
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
    let _ = spawn({
        let app_state = app_state.clone();
        async move {
            ctrl_c().await.expect("Failed to listen for Ctrl+C");
            if let Err(err) = app_state.save().await {
                error!("Failed to save state: {}", err);
            }
        }
    }).await;

    // Wait for server task to finish (though it should run indefinitely until SIGINT)
    if let Err(err) = server_task.await {
        error!("Server task error: {}", err);
    }
}

/// Handler for root endpoint
async fn root() -> &'static str {
    "Hello, world!"
}

/// Handler to get all tables
async fn get_tables(State(state): State<Arc<AppState>>) -> Json<Vec<Table>> {
    let tables = state.get_all().await;
    let json = Json(tables);
    info!("Tables: {:?}", json);
    json
}

/// Handler to create a new table
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/create -d '{"name":"test table"}'
/// ```
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
    state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after creating

    info!("Created table: {:?}", new_table);
    Ok(Json(new_table))
}

/// Handler to drop a table
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/drop_table -d '{"name":"test table"}'
/// ```
async fn drop_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DropTableRequest>
) -> Result<Json<String>, String> {
    let table_name = payload.name;

    if state.drop_table(&table_name).await {
        state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after dropping
        info!("Dropped table: {}", table_name);
        Ok(Json(format!("Dropped table '{}'", table_name)))
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

/// Handler to update a table's name
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/update_table -d '{"current_name":"test table again","new_name":"test table"}'
/// ```
async fn update_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTableRequest>
) -> Result<Json<String>, String> {
    let current_name = payload.current_name;
    let new_name = payload.new_name;

    if let Some(mut table) = state.get(&current_name).await {
        table.name = new_name.clone();
        state.drop_table(&current_name).await;
        state.create(table.clone()).await;
        state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after updating
        info!("Updated table name from '{}' to '{}'", current_name, new_name);
        Ok(Json(format!("Updated table name from '{}' to '{}'", current_name, new_name)))
    } else {
        Err(format!("Table '{}' does not exist", current_name))
    }
}

/// Handler to insert a new column into a table
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/insert_column -d '{"table_name":"test table","key":"test key","primary_key":true,"non_null":true,"unique":true,"foreign_key":null}'
/// ```
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
        state.create(table.clone()).await;
        state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after inserting column
        info!("Inserted column into table '{}': {:?}", table_name, column);
        Ok(Json(column))
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

/// Handler to create a new table with specified columns
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/create_table -d '{"name":"test create table", "insert_column_requests":[{"table_name":"test create table", "key":"test create key", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null},{"table_name":"test create table", "key":"test create key2", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null}]}'
/// ```
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
    state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after creating table

    info!("Created table: {:?}", new_table);
    Ok(Json(new_table))
}

/// Handler to insert a new row into a table
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/insert_row -d '{"table_name":"test table","row":["test value","test value2"]}'
/// ```
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
        state.create(table.clone()).await;
        state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after inserting row
        info!("Inserted row into table '{}': {:?}", table_name, row);
        Ok(Json(row.values.into_iter().map(|cell| cell.value).collect()))
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

/// Application state holding tables
#[derive(Clone)]
struct AppState {
    tables: Arc<RwLock<Vec<Table>>>,
}

impl AppState {
    /// Create a new instance of AppState
    pub fn new() -> Self {
        AppState {
            tables: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Load application state from file
    pub async fn load() -> Result<Self, Error> {
        let file = File::open("db.json").await.map_err(|_| Error::new(io::ErrorKind::NotFound, "File not found"))?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        let tables: Vec<Table> = serde_json::from_str(&contents)?;
        Ok(AppState {
            tables: Arc::new(RwLock::new(tables)),
        })
    }

    /// Save application state to file
    pub async fn save(&self) -> Result<(), Error> {
        let tables = self.get_all().await;
        let contents = serde_json::to_string(&tables)?;
        let file = OpenOptions::new().create(true).write(true).truncate(true).open("db.json").await?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(contents.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }

    /// Add a new table to the application state
    pub async fn create(&self, table: Table) {
        let mut lock = self.tables.write().await;
        lock.push(table);
    }

    /// Get all tables from the application state
    pub async fn get_all(&self) -> Vec<Table> {
        let lock = self.tables.read().await;
        lock.iter().cloned().collect()
    }

    /// Get a specific table from the application state by name
    pub async fn get(&self, table_name: &str) -> Option<Table> {
        let lock = self.tables.read().await;
        lock.iter().find(|table| table.name == table_name).cloned()
    }

    /// Drop a table from the application state by name
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
