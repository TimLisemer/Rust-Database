use client::functions::*;
use reqwest::Client;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest};
use core::row::Row;
use core::value::Value;

/// This main function demonstrates the usage of various client functions with example values.
///
/// # Examples
///
/// ```sh
/// # Create a table
/// curl -X POST http://localhost:3000/create -d '{"name":"test table"}'
///
/// # Drop the table
/// curl -X POST http://localhost:3000/drop_table -d '{"name":"test table"}'
///
/// # Create another table
/// curl -X POST http://localhost:3000/create -d '{"name":"test table again"}'
///
/// # Update the table name
/// curl -X POST http://localhost:3000/update_table -d '{"current_name":"test table again","new_name":"test table"}'
///
/// # Insert columns
/// curl -X POST http://localhost:3000/insert_column -d '{"table_name":"test table","key":"test key","primary_key":true,"non_null":true,"unique":true,"foreign_key":null}'
/// curl -X POST http://localhost:3000/insert_column -d '{"table_name":"test table","key":"test key2","primary_key":true,"non_null":true,"unique":true,"foreign_key":null}'
///
/// # Insert a row
/// curl -X POST http://localhost:3000/insert_row -d '{"table_name":"test table","row":["test value","test value2"]}'
///
/// # Create a table with columns
/// curl -X POST http://localhost:3000/create_table -d '{"name":"test create table", "insert_column_requests":[{"table_name":"test create table", "key":"test create key", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null},{"table_name":"test create table", "key":"test create key2", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null}]}'
///
/// # Insert a row into the newly created table
/// curl -X POST http://localhost:3000/insert_row -d '{"table_name":"test create table","row":["test create value","test create value2"]}'
/// ```
#[tokio::main]
async fn main() {
    let client = Client::new();

    create(&client, &CreateRequests { name: "test table".to_string() }).await.unwrap();

    drop_table(&client, &DropTableRequest { name: "test table".to_string() }).await.unwrap();

    create(&client, &CreateRequests { name: "test table again".to_string() }).await.unwrap();

    update_table(&client, &UpdateTableRequest { current_name: "test table again".to_string(), new_name: "test table".to_string() }).await.unwrap();

    let insert_column_request = InsertColumnRequest {
        table_name: "test table".to_string(),
        key: "test key".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    let insert_column_request2 = InsertColumnRequest {
        table_name: "test table".to_string(),
        key: "test key2".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    insert_column(&client, &insert_column_request).await.unwrap();
    insert_column(&client, &insert_column_request2).await.unwrap();

    let insert_row_request = InsertRowRequest {
        table_name: "test table".to_string(),
        row: Row::new(vec![Value::new("test value".to_string()), Value::new("test value2".to_string())]),
    };

    insert_row(&client, &insert_row_request).await.unwrap();

    let insert_column_request3 = InsertColumnRequest {
        table_name: "test create table".to_string(),
        key: "test create key".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    let insert_column_request4 = InsertColumnRequest {
        table_name: "test create table".to_string(),
        key: "test create key2".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    create_table(&client, &CreateTableRequests {
        name: "test create table".to_string(),
        insert_column_requests: vec![insert_column_request3, insert_column_request4],
    }).await.unwrap();

    let insert_row_request2 = InsertRowRequest {
        table_name: "test create table".to_string(),
        row: Row::new(vec![Value::new("test create value".to_string()), Value::new("test create value2".to_string())]),
    };

    insert_row(&client, &insert_row_request2).await.unwrap();
}
