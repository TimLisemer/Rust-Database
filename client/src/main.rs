use reqwest::get;
use serde_json::Value;

#[tokio::main]
async fn main() {
    match get("http://localhost:3000/table").await {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(json) => println!("{}", json),
            Err(err) => eprintln!("Failed to parse JSON: {}", err),
        },
        Err(err) => eprintln!("Failed to make request: {}", err),
    }
}
