
use tide::{Request, Response, Result};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use std::time::Instant;
use serde_json::json;

#[derive(Serialize)]
struct ResponseData {
    message: String,
    status: String,
    time_taken: String,
}

async fn handle_request(_req: Request<()>, pool: SqlitePool) -> Result<Response> {
    let start = Instant::now();

    // Open a connection to SQLite
    let sqlite_status = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => "Connection opened and closed successfully".to_string(),
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            "Failed to open SQLite connection".to_string()
        }
    };

    let duration = start.elapsed();
    println!("Time taken to open and close SQLite connection: {:?}", duration);

    // Create a JSON response
    let response_data = ResponseData {
        message: "SQLite connection open/close measured".to_string(),
        status: sqlite_status,
        time_taken: format!("{:?}", duration),
    };

    let response_body = json!(response_data).to_string();
    let mut response = Response::new(200);
    response.set_body(response_body);
    response.insert_header("Content-Type", "application/json");

    Ok(response)
}

pub async fn tide_fn(){
    let pool = SqlitePool::connect("sqlite://my_database.db").await.unwrap();
        
    let mut app = tide::new();
    
    // Define a route that handles all incoming requests
    app.at("/").all(move |req: Request<()>| {
        let pool = pool.clone();
        async move {
            handle_request(req, pool).await
        }
    });

    println!("Listening on http://0.0.0.0:8081/");
    app.listen("0.0.0.0:8081").await.unwrap()
}

