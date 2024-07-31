//! Functions to interact with the server's API.
use log::{debug, error, info};
use std::error;
use reqwest::{Client};
use serde_json::json;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, RenameTableRequest, InsertColumnRequest, InsertRowRequest, SelectRequest, UpdateRequest};

/// Creates a new table on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `create_table_request` - The request object containing the name of the table to create.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use serde_json::json;
/// # use core::request_types::CreateRequests;
/// # async fn example(client: &Client, create_table_request: &CreateRequests) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/create");
/// let body = json!({
///     "name": create_table_request.name,
/// });
///
/// let resp = client.post(&url)
///     .json(&body)
///     .send()
///     .await?;
///
/// println!("Create Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn create(client: &Client, create_table_request: &CreateRequests) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/create".to_string();
    let body = json!({
        "name": create_table_request.name,
    });

    let resp = client.post(&url)
        .json(&body)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Create Response: {:?}", resp);
            info!("Created Table {:?}", create_table_request.name);
            Ok(())
        }
        false => {
            error!("Create Response: {:?}", resp);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create table")))
        }
    }
}

/// Creates a new table with specified columns on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `create_table_request` - The request object containing the table name and columns.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::CreateTableRequests;
/// # async fn example(client: &Client, create_table_request: &CreateTableRequests) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/create_table");
///
/// let resp = client.post(&url)
///     .json(create_table_request)
///     .send()
///     .await?;
///
/// println!("Create Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn create_table(client: &Client, create_table_request: &CreateTableRequests) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/create_table".to_string();

    let resp = client.post(&url)
        .json(create_table_request)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Create Table Response: {:?}", resp);
            info!("Created Table {:?}", create_table_request.name);
            Ok(())
        }
        false => {
            error!("Create Table Response: {:?}", resp);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create table with columns")))
        }
    }
}

/// Drops a table on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `drop_table_request` - The request object containing the name of the table to drop.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::DropTableRequest;
/// # async fn example(client: &Client, drop_table_request: &DropTableRequest) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/drop_table");
///
/// let resp = client.post(&url)
///     .json(drop_table_request)
///     .send()
///     .await?;
///
/// println!("Drop Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn drop_table(client: &Client, drop_table_request: &DropTableRequest) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/drop_table".to_string();

    let resp = client.post(&url)
        .json(drop_table_request)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Drop Table Response: {:?}", resp);
            info!("Dropped Table {:?}", drop_table_request.name);
        }
        false => {
            error!("Drop Table Response (Table probably doesnt exist): {:?}", resp);
        }
    }
    Ok(())
}

/// Renames a table's name on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `rename_table_request` - The request object containing the current and new names of the table.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::RenameTableRequest;
/// # async fn example(client: &Client, rename_table_request: &RenameTableRequest) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/rename_table");
///
/// let resp = client.post(&url)
///     .json(rename_table_request)
///     .send()
///     .await?;
///
/// println!("Rename Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn rename_table(client: &Client, rename_table_request: &RenameTableRequest) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/rename_table".to_string();

    let resp = client.post(&url)
        .json(rename_table_request)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Rename Table Response: {:?}", resp);
            info!("Renamed Table {:?}", rename_table_request);
            Ok(())
        }
        false => {
            error!("Rename Table Response: {:?}", resp);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to rename table")))
        }
    }
}

/// Inserts a new column into a table on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `insert_column_request` - The request object containing the table name and column details.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::InsertColumnRequest;
/// # async fn example(client: &Client, insert_column_request: &InsertColumnRequest) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/insert_column");
///
/// let resp = client.post(&url)
///     .json(insert_column_request)
///     .send()
///     .await?;
///
/// println!("Insert Column Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn insert_column(client: &Client, insert_column_request: &InsertColumnRequest) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/insert_column".to_string();

    let resp = client.post(&url)
        .json(insert_column_request)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Insert Column Response: {:?}", resp);
            info!("Inserted Column {:?}", insert_column_request);
            Ok(())
        }
        false => {
            error!("Insert Column Response: {:?}", resp);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to insert column")))
        }
    }
}

/// Inserts a new row into a table on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `insert_row_request` - The request object containing the table name and row data.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::{row::Row, value::Value};
/// # use core::request_types::InsertRowRequest;
/// # async fn example(client: &Client, insert_row_request: &InsertRowRequest) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/insert_row");
///
/// let resp = client.post(&url)
///     .json(insert_row_request)
///     .send()
///     .await?;
///
/// println!("Insert Row Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn insert_row(client: &Client, insert_row_request: &InsertRowRequest) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/insert_row".to_string();

    let resp = client.post(&url)
        .json(insert_row_request)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Insert Row Response: {:?}", resp);
            info!("Inserted Row {:?}", insert_row_request);
            Ok(())
        }
        false => {
            error!("Insert Row Response: {:?}", resp);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to insert row")))
        }
    }
}



/// Sends a select query to the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `select_request` - The request object containing the select query details.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::{SelectRequest, Condition};
/// # async fn example(client: &Client, select_request: &SelectRequest) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/select");
///
/// let resp = client.post(&url)
///     .json(select_request)
///     .send()
///     .await?;
///
/// println!("Select Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn select(client: &Client, select_request: &SelectRequest) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:3000/select".to_string();

    let resp = client.post(&url)
        .json(select_request)
        .send()
        .await?;

    // Extract the status code before consuming `resp`
    let status = resp.status();
    // Get the response body
    let body = resp.text().await?;
    match status.is_success() {
        true => {
            debug!("Select Response: {}", body); // Log the body content
            info!("Select result from 'test_create_table': {}", body);
            Ok(())
        }
        false => {
            error!("Select Response: {}", body); // Log the body content
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to select")))
        }
    }
}



/// Updates rows in a table on the server based on specified conditions.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `update_request` - The request object containing the table name, condition, and updates.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::UpdateRequest;
/// # async fn example(client: &Client, update_request: &UpdateRequest) -> Result<(), Box<dyn error::Error>> {
/// let url = format!("http://localhost:3000/update_table");
///
/// let resp = client.post(&url)
///    .json(update_request)
///    .send()
///    .await?;
///
/// println!("Update Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn update_table(client: &Client, update_request: &UpdateRequest) -> Result<(), Box<dyn error::Error>> {
    let url = "http://localhost:3000/update_table".to_string();

    let resp = client.post(&url)
        .json(update_request)
        .send()
        .await?;

    match resp.status().is_success() {
        true => {
            debug!("Update Table Response: {:?}", resp);
            info!("Updated Table {:?}", update_request.table_name);
            Ok(())
        }
        false => {
            error!("Update Table Response: {:?}", resp);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to update table")))
        }
    }
}

