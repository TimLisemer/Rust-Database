use log::LevelFilter;
use reqwest::Client;

use client::functions::*;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, InsertColumnRequest, InsertRowRequest, SelectRequest, UpdateTableRequest, Condition};
use core::row::Row;
use core::value::Value;

/// This main function demonstrates the usage of various client functions with example values.
///
/// # Examples
///
/// ```sh
/// # Create a table
/// curl -X POST http://localhost:3000/create -H '{"name":"test_table"}'
///
/// # Drop the table
/// curl -X POST http://localhost:3000/drop_table -H '{"name":"test_table"}'
///
/// # Create another table
/// curl -X POST http://localhost:3000/create -H '{"name":"test_table again"}'
///
/// # Update the table name
/// curl -X POST http://localhost:3000/update_table -H '{"current_name":"test_table again","new_name":"test_table"}'
///
/// # Insert columns
/// curl -X POST http://localhost:3000/insert_column -H '{"table_name":"test_table","key":"test_key","primary_key":true,"non_null":true,"unique":true,"foreign_key":null}'
/// curl -X POST http://localhost:3000/insert_column -H '{"table_name":"test_table","key":"test_key2","primary_key":true,"non_null":true,"unique":true,"foreign_key":null}'
///
/// # Insert a row
/// curl -X POST http://localhost:3000/insert_row -H '{"table_name":"test_table","row":["test_value","test_value2"]}'
///
/// # Create a table with columns
/// curl -X POST http://localhost:3000/create_table -H '{"name":"test_create_table", "insert_column_requests":[{"table_name":"test_create_table", "key":"test_create_key", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null},{"table_name":"test_create_table", "key":"test_create_key2", "primary_key":true, "non_null":true, "unique":true, "foreign_key":null}]}'
///
/// # Insert a row into the newly created table
/// curl -X POST http://localhost:3000/insert_row -H '{"table_name":"test_create_table","row":["test_create_value","test_create_value2"]}'
///
/// # Select from a table
/// curl -X POST http://localhost:3000/select -H '{"table_name":"test_table","columns":["test_key", "test_key2"]}'
/// ```
#[tokio::main]
async fn main() {

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let client = Client::new();

    // Drop previous tables
    drop_table(&client, &DropTableRequest { name: "test_table".to_string() }).await.unwrap();
    drop_table(&client, &DropTableRequest { name: "test_table2".to_string() }).await.unwrap();
    drop_table(&client, &DropTableRequest { name: "test_drop_table".to_string() }).await.unwrap();

    // Create a table
    create(&client, &CreateRequests { name: "test_table".to_string() }).await.unwrap();

    // Insert columns
    let insert_column_request = InsertColumnRequest {
        table_name: "test_table".to_string(),
        key: "test_key".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    let insert_column_request2 = InsertColumnRequest {
        table_name: "test_table".to_string(),
        key: "test_key2".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    let insert_column_request3 = InsertColumnRequest {
        table_name: "test_table".to_string(),
        key: "test_key3".to_string(),
        primary_key: true,
        non_null: false,
        unique: true,
        foreign_key: None,
    };

    insert_column(&client, &insert_column_request).await.unwrap();
    insert_column(&client, &insert_column_request2).await.unwrap();
    insert_column(&client, &insert_column_request3).await.unwrap();

    // Create new table to be dropped
    create_table(&client, &CreateTableRequests {
        name: "test_table2".to_string(),
        insert_column_requests: vec![insert_column_request3],
    }).await.unwrap();

    update_table(&client, &UpdateTableRequest { current_name: "test_table2".to_string(), new_name: "test_drop_table".to_string() }).await.unwrap();

    // Drop the table
    drop_table(&client, &DropTableRequest { name: "test_drop_table".to_string() }).await.unwrap();

    // Insert a row
    let insert_row_request = InsertRowRequest {
        table_name: "test_table".to_string(),
        row: Row::new(vec![Value::from("test_value".to_string()), Value::from(13)])
    };

    insert_row(&client, &insert_row_request).await.unwrap();

    // Insert a row
    let insert_row_request = InsertRowRequest {
        table_name: "test_table".to_string(),
        row: Row::new(vec![Value::from(true), Value::from(27.55), Value::from(128)])
    };

    insert_row(&client, &insert_row_request).await.unwrap();

    // Insert a row
    let insert_row_request = InsertRowRequest {
        table_name: "test_table".to_string(),
        row: Row::new(vec![Value::from("test_value_3".to_string()), Value::from(17.78)])
    };

    insert_row(&client, &insert_row_request).await.unwrap();


    // Select from the table without a condition
    let select_request = SelectRequest {
        table_name: "test_table".to_string(),
        columns: Option::from(vec!["test_key".to_string(), "test_key3".to_string()]), // Empty vec would mean *
        condition: None, // Add conditions if needed
    };

    select(&client, &select_request).await.unwrap();

    // Select from the table with a condition
    let select_request = SelectRequest {
        table_name: "test_table".to_string(),
        columns: Option::from(vec!["test_key".to_string(), "test_key3".to_string()]), // Empty vec would mean *
        condition: Option::from(Condition {
            column: "test_key".to_string(),
            value: "true".to_string(),
        }),
    };

    select(&client, &select_request).await.unwrap();

}
