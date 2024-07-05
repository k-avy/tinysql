use rusqlite::Connection;
use serde_json::json;
use std::time::Instant;
use tide::{Request, Response};


pub async fn tide_fn() {
    let mut app = tide::new();
    app.at("/").get(handle_request);

    println!("Listening on http://0.0.0.0:8000/");
    
    app.listen("0.0.0.0:8000").await.unwrap();
}

async fn handle_request(_: Request<()>) -> tide::Result {
    let start = Instant::now();

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_db.sqlite") {
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
    let response_data = json!({
        "message": "SQLite connection open/close measured",
        "status": sqlite_status,
        "time_taken": format!("{:?}", duration),
    });

    let response_body = response_data.to_string();
    let mut response = Response::new(200);
    response.set_body(response_body);
    response.set_content_type("application/json");

    Ok(response)
}
