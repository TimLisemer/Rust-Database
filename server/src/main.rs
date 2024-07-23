use std::io::Error;
use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use std::sync::Arc;
use axum::response::{Html, IntoResponse};
use tokio::{signal::ctrl_c, spawn};
use log::{error, info, LevelFilter};
use tokio::sync::Mutex;
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader};
use core::{
    table::Table,
    column::Column,
    request_types::{
        CreateRequests, CreateTableRequests, DropTableRequest,
        UpdateTableRequest, InsertColumnRequest, InsertRowRequest, SelectRequest
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
        .route("/select", post(select))
        .with_state(Arc::clone(&app_state));

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
        let app_state = Arc::clone(&app_state);
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
async fn root(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tables = state.get_all().await;
    Html(format_tables_html(tables))
}

/// Format tables data into HTML
fn format_tables_html(tables: Vec<Table>) -> String {
    let mut html = String::new();

    html.push_str(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Database Tables</title>
            <style>
                body { font-family: Arial, sans-serif; }
                table { width: 100%; border-collapse: collapse; }
                th, td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }
                th { background-color: #f2f2f2; }
            </style>
        </head>
        <body>
            <h1>Database Tables</h1>
    "#);

    for table in tables {
        html.push_str(&format!(r#"
            <h2>{}</h2>
            <table>
                <tr><th>Column Name</th><th>Primary Key</th><th>Non Null</th><th>Unique</th></tr>
        "#, table.name));

        for column in table.columns {
            html.push_str(&format!(r#"
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
            "#, column.key, column.primary_key, column.non_null, column.unique));
        }

        html.push_str(r#"
            </table>
            <h3>Rows</h3>
            <table>
                <tr><th>Row Values</th></tr>
        "#);

        for row in table.rows {
            let row_values: Vec<String> = row.values.into_iter().map(|value| value.value).collect();
            html.push_str(&format!(r#"
                <tr><td>{}</td></tr>
            "#, row_values.join(", ")));
        }

        html.push_str("</table>");
    }

    html.push_str(r#"
            </body>
        </html>
    "#);

    html
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
/// curl -X POST http://localhost:3000/create -H '{"name":"test_table"}'
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
        name: table_name,
        columns: Vec::new(),
        rows: Vec::new(),
    };

    state.create(new_table.clone()).await;
    state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after creating

    info!("Created table: {:?}", &new_table);
    Ok(Json(new_table))
}

/// Handler to drop a table
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/drop_table -H '{"name":"test_table"}'
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
/// curl -X POST http://localhost:3000/update_table -H '{"current_name":"test_table again","new_name":"test_table"}'
/// ```
async fn update_table(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTableRequest>
) -> Result<Json<String>, String> {
    let current_name = payload.current_name;
    let new_name = payload.new_name;

    if let Some(mut table) = state.get(&current_name).await {
        table.name = new_name;
        state.drop_table(&current_name).await;
        state.create(table.clone()).await;
        state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after updating
        info!("Updated table name from '{}' to '{}'", current_name, table.name);
        Ok(Json(format!("Updated table name from '{}' to '{}'", current_name, table.name)))
    } else {
        Err(format!("Table '{}' does not exist", current_name))
    }
}

/// Handler to insert a new column into a table
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/insert_column -H '{"table_name":"test_table","key":"test_key","primary_key":true,"non_null":true,"unique":true,"foreign_key":null}'
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
        state.create(table).await;
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
/// curl -X POST http://localhost:3000/create_table -H '{"name":"test_create_table", "insert_column_requests":[{"table_name":"test_create_table", "key":"test_create_key", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null},{"table_name":"test_create_table", "key":"test_create_key2", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null}]}'
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
        name: table_name,
        columns: Vec::new(),
        rows: Vec::new(),
    };

    for insert_column_request in payload.insert_column_requests {
        let column = Column::new(
            insert_column_request.key,
            insert_column_request.primary_key,
            insert_column_request.non_null,
            insert_column_request.unique,
            insert_column_request.foreign_key.map(|fk| fk.into_iter().map(Box::new).collect()),
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
/// curl -X POST http://localhost:3000/insert_row -H '{"table_name":"test_table","row":["test_value","test_value2"]}'
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


        let json = Ok(Json(row.values.iter().map(|cell| cell.value.clone()).collect()));
        info!("Inserted row into table '{}': {:?}", table_name, row);
        table.add_row(row);
        state.drop_table(&table_name).await;
        state.create(table).await;
        state.save().await.map_err(|err| format!("Failed to save state: {}", err))?; // Save after inserting row
        json
    } else {
        Err(format!("Table '{}' does not exist", table_name))
    }
}

/// Handler to select rows from a table based on specified conditions or retrieve all rows if no conditions are provided.
///
/// # Example
///
/// ```
/// curl -X POST http://localhost:3000/select -H '{"table_name":"test_table", "columns":["column1", "column2"], "condition":{"column":"column1", "value":"some_value"}}'
/// ```
///
/// Retrieves rows from the specified table (`table_name`) optionally filtered by columns (`columns`) and a conditional (`condition`).
/// If `columns` is not provided, all columns are selected (`SELECT *`).
///
/// ## Parameters
///
/// - `table_name`: Name of the table from which rows are selected.
/// - `columns`: Optional. List of columns to select. If not provided, all columns are selected.
/// - `condition`: Optional. Specifies a condition to filter rows. Only rows matching this condition are returned.
///
/// ## Returns
///
/// Returns a JSON array of rows, where each row is represented as an array of strings (values of selected columns).
///
/// ## Errors
///
/// - Returns an error if the specified `table_name` does not exist in the application state.
/// - Returns an error if the specified `condition.column` does not exist in the table.
///
/// ## Notes
///
/// - This handler supports flexible column selection and row filtering based on conditions.
///
async fn select(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SelectRequest>
) -> Result<Json<Vec<Vec<String>>>, String> {
    if let Some(table) = state.get(payload.table_name.as_str()).await {
        let selected_columns = match payload.columns {
            Some(cols) => cols,
            None => table.columns.iter().map(|col| col.key.clone()).collect(), // SELECT *
        };

        let mut rows = vec![];

        for row in &table.rows {
            if let Some(cond) = &payload.condition {
                if let Some(col_index) = table.columns.iter().position(|col| col.key == cond.column) {
                    if row.values[col_index].value != cond.value {
                        continue;
                    }
                } else {
                    return Err(format!("Column '{}' not found", cond.column));
                }
            }

            let selected_values = selected_columns.iter().filter_map(|col_key| {
                table.columns.iter().position(|col| col.key.eq(col_key)).map(|index| row.values[index].value.clone())
            }).collect::<Vec<String>>();

            rows.push(selected_values);
        }

        Ok(Json(rows))
    } else {
        Err(format!("Table '{}' does not exist", payload.table_name))
    }
}

/// Application state holding tables
#[derive(Clone)]
struct AppState {
    tables: Arc<Mutex<Vec<Table>>>,
}

impl AppState {
    /// Create a new instance of AppState
    pub fn new() -> Self {
        AppState {
            tables: Arc::new(Mutex::new(Vec::new())),
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
            tables: Arc::new(Mutex::new(tables)),
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
        let mut lock = self.tables.lock().await;
        lock.push(table);
    }

    /// Get all tables from the application state
    pub async fn get_all(&self) -> Vec<Table> {
        let lock = self.tables.lock().await;
        lock.iter().cloned().collect()
    }

    /// Get a specific table from the application state by name
    pub async fn get(&self, table_name: &str) -> Option<Table> {
        let lock = self.tables.lock().await;
        lock.iter().find(|table| table.name == table_name).cloned()
    }

    /// Drop a table from the application state by name
    pub async fn drop_table(&self, table_name: &str) -> bool {
        let mut lock = self.tables.lock().await;
        if let Some(index) = lock.iter().position(|table| table.name == table_name) {
            lock.remove(index);
            true
        } else {
            false
        }
    }
}
