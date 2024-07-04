use rayon::ThreadPoolBuilder;

use rusqlite::Connection;
use tiny_http::{ Server,Response, Request, Header};
use std::time::Instant;
use serde_json::json;

// pub mod request_handler;

pub fn tiny_fn() {
    // Create an HTTP server that listens on port 8000
    let server = Server::http("0.0.0.0:8000").unwrap();

    // Create a thread pool with a custom number of threads
    let pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();

    println!("Listening on http://0.0.0.0:8000/");

    for request in server.incoming_requests() {
        // Use the thread pool to handle the request concurrently
        pool.spawn(move || {
           handle_request(request);
        });
    }
}

pub fn handle_request(request: Request){
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
    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

    if let Err(e) = request.respond(response) {
        eprintln!("Failed to respond to request: {}", e);
    }
}
