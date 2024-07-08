use async_std::task;
use tiny_http::{Server, Response, Header, Request};
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

async fn handle_request(request: Request, pool: SqlitePool) {
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
    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

    if let Err(e) = request.respond(response) {
        eprintln!("Failed to respond to request: {}", e);
    }
}

pub fn tiny_fn() {
    task::block_on(async {
        let pool = SqlitePool::connect("sqlite://my_database.db").await.unwrap();

        let server = Server::http("0.0.0.0:8081").unwrap();
        println!("Listening on http://0.0.0.0:8081/");

        for request in server.incoming_requests() {
            let pool = pool.clone();
            task::spawn(async move {
                handle_request(request, pool).await;
            });
        }
    });
}
