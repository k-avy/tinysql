use rayon::ThreadPoolBuilder;
use std::sync::Arc;
use tiny_http::{Server,Request, Response, Header};
use std::time::Instant;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;


pub fn start_server() {
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

pub fn handle_request(request: Request, pool_sqlite: Pool<SqliteConnectionManager>) {
    let start = Instant::now();

    // Get a connection from the pool
    let sqlite_status = match pool_sqlite.get() {
        Ok(conn) => {
            // Close the connection by letting it go out of scope
            drop(conn);
            "Connection opened and closed successfully"
        }
        Err(e) => {
            eprintln!("Failed to get SQLite connection: {}", e);
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
