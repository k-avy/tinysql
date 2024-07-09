use async_std::task;
use tiny_http::{Server, Response, Header, Request};
use serde::{Serialize, Deserialize};
use std::time::Instant;
use serde_json::json;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params};

#[derive(Serialize)]
struct ResponseData {
    message: String,
    status: String,
    time_taken: String,
}

#[derive(Deserialize)]
struct RequestData {
    query: String,
}

async fn handle_request(mut request: Request, conn: Arc<Mutex<Connection>>) {
    let start = Instant::now();

    // Read and parse the JSON body
    let mut request_body = Vec::new();
    if let Err(e) = request.as_reader().read_to_end(&mut request_body) {
        eprintln!("Failed to read request body: {}", e);
        let response = Response::from_string("Failed to read request body")
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
        if let Err(e) = request.respond(response) {
            eprintln!("Failed to respond to request: {}", e);
        }
        return;
    }

    let request_data: RequestData = match serde_json::from_slice(&request_body) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse request body as JSON: {}", e);
            let response = Response::from_string("Failed to parse request body as JSON")
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap());
            if let Err(e) = request.respond(response) {
                eprintln!("Failed to respond to request: {}", e);
            }
            return;
        }
    };

    // Execute the query in SQLite
    let sqlite_status = match conn.lock().unwrap().execute(&request_data.query, params![]) {
        Ok(_) => "Query executed successfully".to_string(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            "Failed to execute query".to_string()
        }
    };

    let duration = start.elapsed();
    println!("Time taken to execute query: {:?}", duration);

    // Create a JSON response
    let response_data = ResponseData {
        message: "SQLite query execution measured".to_string(),
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

pub fn tiny_db_hosted() {
    task::block_on(async {
        let conn = Connection::open("my_database.db").unwrap();
        let conn = Arc::new(Mutex::new(conn));

        let server = Server::http("0.0.0.0:8081").unwrap();
        println!("Listening on http://0.0.0.0:8081/");

        for request in server.incoming_requests() {
            let conn = Arc::clone(&conn);
            task::spawn(async move {
                handle_request(request, conn).await;
            });
        }
    });
}