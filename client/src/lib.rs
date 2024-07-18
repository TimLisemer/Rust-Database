// src/lib.rs

//! # Client Crate
//!
//! This crate provides a client to interact with a server that manages tables and their data. It allows for creating, dropping, updating tables, and inserting columns and rows.
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
//!
//! ## Usage
//!
//! To use this crate, create an instance of the HTTP client and call the desired functions with the appropriate request objects.
//!
//! ### Example
//!
//! ```rust,no_run
//! use reqwest::Client;
//! use client::{create, create_table, drop_table, insert_column, insert_row, update_table};
//! use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest};
//! use core::row::Row;
//! use core::value::Value;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new();
//!
//!     create(&client, &CreateRequests { name: "test table".to_string() }).await.unwrap();
//!
//!     drop_table(&client, &DropTableRequest { name: "test table".to_string() }).await.unwrap();
//!
//!     create(&client, &CreateRequests { name: "test table again".to_string() }).await.unwrap();
//!
//!     update_table(&client, &UpdateTableRequest { current_name: "test table again".to_string(), new_name: "test table".to_string() }).await.unwrap();
//!
//!     let insert_column_request = InsertColumnRequest {
//!         table_name: "test table".to_string(),
//!         key: "test key".to_string(),
//!         primary_key: true,
//!         non_null: true,
//!         unique: true,
//!         foreign_key: None,
//!     };
//!
//!     insert_column(&client, &insert_column_request).await.unwrap();
//!
//!     let insert_row_request = InsertRowRequest {
//!         table_name: "test table".to_string(),
//!         row: Row::new(vec![Value::new("test value".to_string()), Value::new("test value2".to_string())]),
//!     };
//!
//!     insert_row(&client, &insert_row_request).await.unwrap();
//! }
//! ```
//!

pub mod functions;

pub use crate::functions::{
    create, create_table, drop_table, update_table, insert_column, insert_row,
};



#[cfg(feature = "doc_examples")]
extern crate reqwest;