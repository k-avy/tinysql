use rayon::ThreadPoolBuilder;
use rusqlite::Connection;
use tiny_http::{Server, Response, Request, Header};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize,Serialize)]
struct RequestData {
    // Add the fields expected in the JSON request
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
    status: String,
    time_taken: String,
    received_data: Option<RequestData>,
}

pub fn tiny_pooled() {
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

pub fn handle_request(mut request: Request) {
    let start = Instant::now();

    // Read the request body
    let mut body = String::new();
    if let Err(e) = request.as_reader().read_to_string(&mut body) {
        eprintln!("Failed to read request body: {}", e);
        let response = Response::from_string("Failed to read request body")
            .with_status_code(400);
        if let Err(e) = request.respond(response) {
            eprintln!("Failed to respond to request: {}", e);
        }
        return;
    }

    // Parse JSON from the request body
    let request_data: Result<RequestData, serde_json::Error> = serde_json::from_str(&body);
    let received_data = match request_data {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Failed to parse JSON request: {}", e);
            let response = Response::from_string("Failed to parse JSON request")
                .with_status_code(400);
            if let Err(e) = request.respond(response) {
                eprintln!("Failed to respond to request: {}", e);
            }
            return;
        }
    };

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_db.sqlite") {
        Ok(conn) => {
            // Close the connection by letting it go out of scope
            drop(conn);
            "Connection opened and closed successfully".to_string()
        }
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
        received_data,
    };

    let response_body = serde_json::to_string(&response_data).unwrap();
    let response = Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

    if let Err(e) = request.respond(response) {
        eprintln!("Failed to respond to request: {}", e);
    }
}
