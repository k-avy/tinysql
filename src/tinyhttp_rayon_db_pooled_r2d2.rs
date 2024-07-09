use rayon::ThreadPoolBuilder;
use std::sync::Arc;
use tiny_http::{Server, Request as TinyRequest, Response as TinyResponse, Header, Method};
use std::time::Instant;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize)]
struct MyRequest {
    // Define your request fields here
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct MyResponse {
    message: String,
    status: String,
    time_taken: String,
}

pub fn server_db_pooled() {
    // Create an HTTP server that listens on port 8000
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("Listening on http://0.0.0.0:8000/");

    // Create a connection manager and pool for SQLite
    let manager = SqliteConnectionManager::file("my_db.sqlite");
    let pooldb = Pool::new(manager).unwrap();

    // Use a rayon thread pool for handling requests
    let thread_pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();
    let thread_pool = Arc::new(thread_pool);

    for request in server.incoming_requests() {
        let pool_clone = Arc::clone(&thread_pool);
        let pool_sqlite = pooldb.clone();
        pool_clone.spawn(move || {
            handle_request(request, pool_sqlite);
        });
    }
}

pub fn handle_request(mut request: TinyRequest, pool_sqlite: Pool<SqliteConnectionManager>) {
    let start = Instant::now();

    // Only handle POST requests
    if request.method() == &Method::Post {
        let mut content = String::new();
        if let Err(e) = request.as_reader().read_to_string(&mut content) {
            eprintln!("Failed to read request body: {}", e);
            let response = TinyResponse::from_string("Failed to read request body")
                .with_status_code(400);
            if let Err(e) = request.respond(response) {
                eprintln!("Failed to respond to request: {}", e);
            }
            return;
        }

        // Parse the request body as JSON into MyRequest struct
        let json_data: MyRequest = match serde_json::from_str(&content) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                let response = TinyResponse::from_string("Invalid JSON")
                    .with_status_code(400);
                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
                return;
            }
        };

        // Example of how you might use the parsed JSON data
        println!("Received JSON data: {:?}", json_data);

        // Get a connection from the pool
        let sqlite_status = match pool_sqlite.get() {
            Ok(conn) => {
                // Close the connection by letting it go out of scope
                drop(conn);
                "Connection opened and closed successfully".to_string()
            }
            Err(e) => {
                eprintln!("Failed to get SQLite connection: {}", e);
                "Failed to open SQLite connection".to_string()
            }
        };

        let duration = start.elapsed();
        println!("Time taken to open and close SQLite connection: {:?}", duration);

        // Create a JSON response using MyResponse struct
        let response_data = MyResponse {
            message: "SQLite connection open/close measured".to_string(),
            status: sqlite_status,
            time_taken: format!("{:?}", duration),
        };

        let response_body = serde_json::to_string(&response_data).unwrap();
        let response = TinyResponse::from_string(response_body)
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

        if let Err(e) = request.respond(response) {
            eprintln!("Failed to respond to request: {}", e);
        }
    } else {
        // Respond with 405 Method Not Allowed for non-POST requests
        let response = TinyResponse::from_string("Method Not Allowed")
            .with_status_code(405);
        if let Err(e) = request.respond(response) {
            eprintln!("Failed to respond to request: {}", e);
        }
    }
}

fn main() {
    server_db_pooled();
}
