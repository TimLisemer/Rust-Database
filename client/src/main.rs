// Client code

use reqwest::{Client, Response};
use serde_json::json;
use core::request_types::{CreateTableRequest, DropTableRequest, UpdateTableRequest, InsertColumnRequest};

#[tokio::main]
async fn main() {
    let client = Client::new();

    create(&client, &CreateTableRequest { name: "test table".to_string() }).await.unwrap();

    drop_table(&client, &DropTableRequest { name: "test table".to_string() }).await.unwrap();

    create(&client, &CreateTableRequest { name: "test table again".to_string() }).await.unwrap();

    update_table(&client, &UpdateTableRequest { current_name: "test table again".to_string(), new_name: "test table".to_string() }).await.unwrap();

    insert_column(&client, &InsertColumnRequest {
        table_name: "test table".to_string(),
        key: "test key".to_string(),
        value: "test value".to_string(),
        primary_key: true,
        non_null: true,
        unique: true,
        foreign_key: None,
    }).await.unwrap();
}

async fn create(client: &Client, create_table_request: &CreateTableRequest) -> Result<Response, reqwest::Error> {
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
