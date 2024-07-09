use tide::{Request, Response, Result};
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct RequestData {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
    status: String,
    time_taken: String,
    received_data: Option<RequestData>,
}

#[derive(Clone)]
struct State {
    pool: Pool<SqliteConnectionManager>,
}

async fn handle_request(mut req: Request<State>) -> Result {
    let start = Instant::now();

    // Parse JSON from the request body
    let request_data: Result<RequestData> = req.body_json().await;

    let received_data = match request_data {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Failed to parse JSON request: {}", e);
            None
        }
    };

    // Get the connection pool from the state
    let pool = req.state().pool.clone();
    let sqlite_status = match pool.get() {
        Ok(conn) => {
            match conn.execute("SELECT 1", []) {
                Ok(_) => "Connection opened and query executed successfully".to_string(),
                Err(e) => {
                    eprintln!("Failed to execute SQLite query: {}", e);
                    "Failed to execute SQLite query".to_string()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get connection from pool: {}", e);
            "Failed to get connection from pool".to_string()
        }
    };

    let duration = start.elapsed();
    println!("Time taken to execute SQLite query: {:?}", duration);

    // Create a JSON response
    let response_data = ResponseData {
        message: "SQLite connection open/close measured".to_string(),
        status: sqlite_status,
        time_taken: format!("{:?}", duration),
        received_data,
    };

    let response_body = serde_json::to_string(&response_data)?;
    let mut response = Response::new(200);
    response.set_body(response_body);
    response.insert_header("Content-Type", "application/json");

    Ok(response)
}

pub async fn tide_pooled_db() {
    // Set up the SQLite connection manager and pool
    let manager = SqliteConnectionManager::file("my_database.db");
    let pool = Pool::new(manager).expect("Failed to create pool.");

    let mut app = tide::with_state(State { pool });

    // Define a route that handles all incoming requests
    app.at("/").all(|req: Request<State>| async move {
        handle_request(req).await
    });

    println!("Listening on http://0.0.0.0:8081/");
    app.listen("0.0.0.0:8081").await.unwrap();
}
