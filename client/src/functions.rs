//! Functions to interact with the server's API.

use reqwest::{Client, Response};
use serde_json::json;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest};

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
/// # async fn example(client: &Client, create_table_request: &CreateRequests) -> Result<(), reqwest::Error> {
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
pub async fn create(client: &Client, create_table_request: &CreateRequests) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/create");
    let body = json!({
        "name": create_table_request.name,
    });

    let resp = client.post(&url)
        .json(&body)
        .send()
        .await?;

    println!("Create Table Response: {:?}", resp);
    Ok(resp)
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
/// # async fn example(client: &Client, create_table_request: &CreateTableRequests) -> Result<(), reqwest::Error> {
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
pub async fn create_table(client: &Client, create_table_request: &CreateTableRequests) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/create_table");

    let resp = client.post(&url)
        .json(create_table_request)
        .send()
        .await?;

    println!("Create Table Response: {:?}", resp);
    Ok(resp)
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
/// # async fn example(client: &Client, drop_table_request: &DropTableRequest) -> Result<(), reqwest::Error> {
/// let url = format!("http://localhost:3000/drop_table");
///
/// let resp = client.post(&url)
///     .json(drop_table_request)
///     .send()
///     .await?;
///
/// println!("Update Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn drop_table(client: &Client, drop_table_request: &DropTableRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/drop_table");

    let resp = client.post(&url)
        .json(drop_table_request)
        .send()
        .await?;

    println!("Update Table Response: {:?}", resp);
    Ok(resp)
}

/// Updates a table's name on the server.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `update_table_request` - The request object containing the current and new names of the table.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "doc_examples")] {
/// # use reqwest::Client;
/// # use core::request_types::UpdateTableRequest;
/// # async fn example(client: &Client, update_table_request: &UpdateTableRequest) -> Result<(), reqwest::Error> {
/// let url = format!("http://localhost:3000/update_table");
///
/// let resp = client.post(&url)
///     .json(update_table_request)
///     .send()
///     .await?;
///
/// println!("Update Table Response: {:?}", resp);
/// # Ok(())
/// # }
/// # }
/// ```
pub async fn update_table(client: &Client, update_table_request: &UpdateTableRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/update_table");

    let resp = client.post(&url)
        .json(update_table_request)
        .send()
        .await?;

    println!("Update Table Response: {:?}", resp);
    Ok(resp)
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
/// # async fn example(client: &Client, insert_column_request: &InsertColumnRequest) -> Result<(), reqwest::Error> {
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
pub async fn insert_column(client: &Client, insert_column_request: &InsertColumnRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/insert_column");

    let resp = client.post(&url)
        .json(insert_column_request)
        .send()
        .await?;

    println!("Insert Column Response: {:?}", resp);
    Ok(resp)
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
/// # async fn example(client: &Client, insert_row_request: &InsertRowRequest) -> Result<(), reqwest::Error> {
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
pub async fn insert_row(client: &Client, insert_row_request: &InsertRowRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/insert_row");

    let resp = client.post(&url)
        .json(insert_row_request)
        .send()
        .await?;

    println!("Insert Row Response: {:?}", resp);
    Ok(resp)
}
