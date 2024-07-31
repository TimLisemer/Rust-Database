//! # Client Crate
//!
//! This crate provides a client to interact with a server that manages tables and their data. It allows for creating, dropping, updating tables, inserting columns and rows, and selecting data from tables.
//!
//! ## Overview
//!
//! The client supports the following operations:
//!
//! - Creating tables
//! - Dropping tables
//! - Updating table names
//! - Inserting columns into tables
//! - Inserting rows into tables
//! - Selecting data from tables
//!
//! ## Usage
//!
//! To use this crate, create an instance of the HTTP client and call the desired functions with the appropriate request objects.
//!
//! ### Example
//!
//! ```rust,no_run
//! use log::LevelFilter;
//! use reqwest::Client;
//!
//! use client::{create, create_table, drop_table, insert_column, insert_row, update_table, select};
//! use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, InsertColumnRequest, InsertRowRequest, SelectRequest, UpdateTableRequest, Condition};
//! use core::row::Row;
//! use core::value::Value;
//!
//! #[tokio::main]
//! async fn main() {
//!     env_logger::builder()
//!         .filter_level(LevelFilter::Info)
//!         .format_timestamp_millis()
//!         .init();
//!
//!     let client = Client::new();
//!
//!     // Drop previous tables
//!     drop_table(&client, &DropTableRequest { name: "test_table".to_string() }).await.unwrap();
//!     drop_table(&client, &DropTableRequest { name: "test_table2".to_string() }).await.unwrap();
//!     drop_table(&client, &DropTableRequest { name: "test_drop_table".to_string() }).await.unwrap();
//!
//!     // Create a table
//!     create(&client, &CreateRequests { name: "test_table".to_string() }).await.unwrap();
//!
//!     // Insert columns
//!     let insert_column_request = InsertColumnRequest {
//!         table_name: "test_table".to_string(),
//!         key: "test_key".to_string(),
//!         primary_key: true,
//!         non_null: true,
//!         unique: true,
//!         foreign_key: None,
//!     };
//!
//!     let insert_column_request2 = InsertColumnRequest {
//!         table_name: "test_table".to_string(),
//!         key: "test_key2".to_string(),
//!         primary_key: true,
//!         non_null: true,
//!         unique: true,
//!         foreign_key: None,
//!     };
//!
//!     let insert_column_request3 = InsertColumnRequest {
//!         table_name: "test_table".to_string(),
//!         key: "test_key3".to_string(),
//!         primary_key: true,
//!         non_null: false,
//!         unique: true,
//!         foreign_key: None,
//!     };
//!
//!     insert_column(&client, &insert_column_request).await.unwrap();
//!     insert_column(&client, &insert_column_request2).await.unwrap();
//!     insert_column(&client, &insert_column_request3).await.unwrap();
//!
//!     // Create new table to be dropped
//!     create_table(&client, &CreateTableRequests {
//!         name: "test_table2".to_string(),
//!         insert_column_requests: vec![insert_column_request3],
//!     }).await.unwrap();
//!
//!     update_table(&client, &UpdateTableRequest { current_name: "test_table2".to_string(), new_name: "test_drop_table".to_string() }).await.unwrap();
//!
//!     // Drop the table
//!     drop_table(&client, &DropTableRequest { name: "test_drop_table".to_string() }).await.unwrap();
//!
//!     // Insert a row
//!     let insert_row_request = InsertRowRequest {
//!         table_name: "test_table".to_string(),
//!         row: Row::new(vec![Value::from("test_value".to_string()), Value::from(13)])
//!     };
//!
//!     insert_row(&client, &insert_row_request).await.unwrap();
//!
//!     // Insert a row
//!     let insert_row_request = InsertRowRequest {
//!         table_name: "test_table".to_string(),
//!         row: Row::new(vec![Value::from(true), Value::from(27.55), Value::from(128)])
//!     };
//!
//!     insert_row(&client, &insert_row_request).await.unwrap();
//!
//!     // Insert a row
//!     let insert_row_request = InsertRowRequest {
//!         table_name: "test_table".to_string(),
//!         row: Row::new(vec![Value::from("test_value_3".to_string()), Value::from(17.78)])
//!     };
//!
//!     insert_row(&client, &insert_row_request).await.unwrap();
//!
//!
//!     // Select from the table
//!     let select_request = SelectRequest {
//!         table_name: "test_table".to_string(),
//!         columns: Option::from(vec!["test_key".to_string(), "test_key3".to_string()]),
//!         condition: None, // Add conditions if needed
//!     };
//!
//!     select(&client, &select_request).await.unwrap();
//!
//!     // Select from the table without a condition
//!     let select_request = SelectRequest {
//!         table_name: "test_table".to_string(),
//!         columns: Option::from(vec!["test_key".to_string(), "test_key3".to_string()]), // Empty vec would mean *
//!         condition: None, // Add conditions if needed
//!     };
//!
//!     select(&client, &select_request).await.unwrap();
//!
//!     // Select from the table with a condition
//!     let select_request = SelectRequest {
//!         table_name: "test_table".to_string(),
//!         columns: Option::from(vec!["test_key".to_string(), "test_key3".to_string()]), // Empty vec would mean *
//!         condition: Option::from(Condition {
//!             column: "test_key".to_string(),
//!             value: "true".to_string(),
//!         }),
//!     };
//!
//!     select(&client, &select_request).await.unwrap();
//! }
//! ```



pub mod functions;

pub use crate::functions::{
    create, create_table, drop_table, update_table, insert_column, insert_row, select
};



#[cfg(feature = "doc_examples")]
extern crate reqwest;