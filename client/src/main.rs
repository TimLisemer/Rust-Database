use client::functions::*;
use reqwest::Client;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest, SelectRequest};
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
    let client = Client::new();

    // Create a table
    create(&client, &CreateRequests { name: "test_table".to_string() }).await.unwrap();
    println!("Created table: 'test_table'");

    // Drop the table
    drop_table(&client, &DropTableRequest { name: "test_table".to_string() }).await.unwrap();
    println!("Dropped table: 'test_table'");

    // Create another table
    create(&client, &CreateRequests { name: "test_table again".to_string() }).await.unwrap();
    println!("Created table: 'test_table again'");

    // Update the table name
    update_table(&client, &UpdateTableRequest { current_name: "test_table again".to_string(), new_name: "test_table".to_string() }).await.unwrap();
    println!("Updated table name from 'test_table again' to 'test_table'");

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

    insert_column(&client, &insert_column_request).await.unwrap();
    println!("Inserted column 'test_key' into table 'test_table'");
    insert_column(&client, &insert_column_request2).await.unwrap();
    println!("Inserted column 'test_key2' into table 'test_table'");

    // Insert a row
    let insert_row_request = InsertRowRequest {
        table_name: "test_table".to_string(),
        row: Row::new(vec![Value::new("test_value".to_string()), Value::new("test_value2".to_string())]),
    };

    insert_row(&client, &insert_row_request).await.unwrap();
    println!("Inserted row ['test_value', 'test_value2'] into table 'test_table'");

    // Create a table with columns
    let insert_column_request3 = InsertColumnRequest {
        table_name: "test_create_table".to_string(),
        key: "test_create_key".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    let insert_column_request4 = InsertColumnRequest {
        table_name: "test_create_table".to_string(),
        key: "test_create_key2".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    };

    create_table(&client, &CreateTableRequests {
        name: "test_create_table".to_string(),
        insert_column_requests: vec![insert_column_request3, insert_column_request4],
    }).await.unwrap();
    println!("Created table 'test_create_table' with columns 'test_create_key' and 'test_create_key2'");

    // Insert a row into the newly created table
    let insert_row_request2 = InsertRowRequest {
        table_name: "test_create_table".to_string(),
        row: Row::new(vec![Value::new("test_create_value".to_string()), Value::new("test_create_value2".to_string())]),
    };

    insert_row(&client, &insert_row_request2).await.unwrap();
    println!("Inserted row ['test_create_value', 'test_create_value2'] into table 'test_create_table'");

    // Select from the table
    let select_request = SelectRequest {
        table_name: "test_table".to_string(),
        columns: Option::from(vec!["test_key".to_string(), "test_key2".to_string()]),
        condition: None, // Add conditions if needed
    };

    let select_response = select(&client, &select_request).await.unwrap();
    let select_result = select_response.text().await.unwrap();
    println!("Select result from 'test_table': {}", select_result);

    let select_request2 = SelectRequest {
        table_name: "test_create_table".to_string(),
        columns: Option::from(vec!["test_create_key".to_string(), "test_create_key2".to_string()]),
        condition: None,
    };

    let select_response2 = select(&client, &select_request2).await.unwrap();
    let select_result2 = select_response2.text().await.unwrap();
    println!("Select result from 'test_create_table': {}", select_result2);

    drop_table(&client, &DropTableRequest { name: "test_create_table".to_string() }).await.unwrap();
    println!("Dropped table: 'test_create_table'");
}
