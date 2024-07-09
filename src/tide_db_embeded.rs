use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tide::{Request, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize here
struct RequestData {
    key: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct ResponseData {
    message: String,
    status: String,
    time_taken: String,
    received_data: Option<RequestData>,
}

pub async fn tide_embedded() {
    let mut app = tide::new();
    app.at("/").post(handle_request);

    println!("Listening on http://0.0.0.0:8000/");

    app.listen("0.0.0.0:8000").await.unwrap();
}

async fn handle_request(mut req: Request<()>) -> tide::Result {
    let start = Instant::now();

    // Parse JSON from request body
    let request_data: Result<RequestData, tide::Error> = req.body_json().await;

    let received_data = match request_data {
        Ok(data) => Some(data),
        Err(_) => None,
    };

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("db/my_database.sqlite") {
        Ok(conn) => {
            // Close the connection by letting it go out of scope
            drop(conn);
            "Connection opened and closed successfully"
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            "Failed to open SQLite connection"
        }
    };

    let duration = start.elapsed();
    println!("Time taken to open and close SQLite connection: {:?}", duration);

    // Create a JSON response
    let response_data = ResponseData {
        message: String::from("SQLite connection open/close measured"),
        status: sqlite_status.to_string(),
        time_taken: format!("{:?}", duration),
        received_data,
    };

    let response_body = serde_json::to_string(&response_data)?;
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(response_body);
    response.set_content_type("application/json");

    Ok(response)
}
