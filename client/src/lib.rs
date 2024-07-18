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
//! use reqwest::Client;
//! use client::{create, create_table, drop_table, insert_column, insert_row, update_table, select};
//! use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest, SelectRequest};
//! use core::row::Row;
//! use core::value::Value;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new();
//!
//!     create(&client, &CreateRequests { name: "test_table".to_string() }).await.unwrap();
//!
//!     drop_table(&client, &DropTableRequest { name: "test_table".to_string() }).await.unwrap();
//!
//!     create(&client, &CreateRequests { name: "test_table again".to_string() }).await.unwrap();
//!
//!     update_table(&client, &UpdateTableRequest { current_name: "test_table again".to_string(), new_name: "test_table".to_string() }).await.unwrap();
//!
//!     let insert_column_request = InsertColumnRequest {
//!         table_name: "test_table".to_string(),
//!         key: "test_key".to_string(),
//!         primary_key: true,
//!         non_null: true,
//!         unique: true,
//!         foreign_key: None,
//!     };
//!
//!     insert_column(&client, &insert_column_request).await.unwrap();
//!
//!     let insert_row_request = InsertRowRequest {
//!         table_name: "test_table".to_string(),
//!         row: Row::new(vec![Value::new("test_value".to_string()), Value::new("test_value2".to_string())]),
//!     };
//!
//!     insert_row(&client, &insert_row_request).await.unwrap();
//!
//!     let select_request = SelectRequest {
//!         table_name: "test_table".to_string(),
//!         columns: Some(vec!["test_key".to_string(), "test_key2".to_string()]),
//!         condition: None,
//!     };
//!
//!     let select_response = select(&client, &select_request).await.unwrap();
//!     let select_result = select_response.text().await.unwrap();
//!     println!("Select result from 'test_table': {}", select_result);
//! }
//! ```



pub mod functions;

pub use crate::functions::{
    create, create_table, drop_table, update_table, insert_column, insert_row, select
};



#[cfg(feature = "doc_examples")]
extern crate reqwest;