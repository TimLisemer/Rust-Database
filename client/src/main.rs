// Client code

use reqwest::{Client, Response};
use serde_json::json;
use core::request_types::{CreateRequests, CreateTableRequests, DropTableRequest, UpdateTableRequest, InsertColumnRequest, InsertRowRequest};
use core::row::Row;
use core::value::Value;

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

async fn create(client: &Client, create_table_request: &CreateRequests) -> Result<Response, reqwest::Error> {
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

async fn create_table(client: &Client, create_table_request: &CreateTableRequests) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/create_table");

    let resp = client.post(&url)
        .json(create_table_request)
        .send()
        .await?;

    println!("Create Table Response: {:?}", resp);
    Ok(resp)
}

async fn drop_table(client: &Client, drop_table_request: &DropTableRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/drop_table");

    let resp = client.post(&url)
        .json(drop_table_request)
        .send()
        .await?;

    println!("Update Table Response: {:?}", resp);
    Ok(resp)
}

async fn update_table(client: &Client, update_table_request: &UpdateTableRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/update_table");

    let resp = client.post(&url)
        .json(update_table_request)
        .send()
        .await?;

    println!("Update Table Response: {:?}", resp);
    Ok(resp)
}

async fn insert_column(client: &Client, insert_column_request: &InsertColumnRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/insert_column");

    let resp = client.post(&url)
        .json(insert_column_request)
        .send()
        .await?;

    println!("Insert Column Response: {:?}", resp);
    Ok(resp)
}

async fn insert_row(client: &Client, insert_row_request: &InsertRowRequest) -> Result<Response, reqwest::Error> {
    let url = format!("http://localhost:3000/insert_row");

    let resp = client.post(&url)
        .json(insert_row_request)
        .send()
        .await?;

    println!("Insert Row Response: {:?}", resp);
    Ok(resp)
}