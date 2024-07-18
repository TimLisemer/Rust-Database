//! # Core Module Documentation
//!
//! This module contains the core structures and types used in the project.
//!
//! ## Modules
//!
//! - [`table`](table): Defines the `Table` structure representing a database table.
//! - [`column`](crate::column): Defines the `Column` structure representing a column in a table.
//! - [`request_types`](crate::request_types): Defines various request types used in interacting with tables.
//! - [`value`](value): Defines the `Value` structure representing a value in a table.
//! - [`row`](row): Defines the `Row` structure representing a row in a table.
//!
//! Each module encapsulates related functionality and data structures essential for database operations.
//!
//! ## Example Usage
//!
//! ```
//! // Example demonstrating table creation
//! use core::table::Table;
//! use core::column::Column;
//!
//! let mut table = Table::new("users".to_string());
//! let column = Column::new("id".to_string(), true, true, true, None);
//! table.add_column(column);
//!
//! println!("Created table: {:?}", table);
//! ```
//!
//! This module serves as the foundational layer for database operations, providing essential structures
//! for defining tables, columns, rows, and their interactions.
//!

pub mod table;
pub mod column;
pub mod request_types;
pub mod value;
pub mod row;
