// use sqlx::sqlite::SqlitePool;
// use tiny_http::{Server, Response, Header};
// use std::time::Instant;
// use serde_json::json;
// use async_std::task;


// pub async fn tinysqlserver() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a connection pool to SQLite
//     let pool = SqlitePool::connect("sqlite:my_db.sqlite").await?;

//     // Create an HTTP server that listens on port 8000
//     let server = Server::http("0.0.0.0:8000").unwrap();
//     println!("Listening on http://0.0.0.0:8000/");

//     for request in server.incoming_requests() {
//         let pool = pool.clone();  // Clone the pool to use it in async block

//         task::spawn(async move {
//             let start = Instant::now();

//             // Open and close a connection from the pool
//             match pool.acquire().await {
//                 Ok(_conn) => {
//                     // The connection will be automatically returned to the pool when it goes out of scope
//                 }
//                 Err(e) => {
//                     eprintln!("Failed to acquire SQLite connection: {}", e);
//                 }
//             }

//             let duration = start.elapsed();
//             println!("Time taken to acquire and release SQLite connection: {:?}", duration);

//             // Prepare the JSON response
//             let response_body = json!({
//                 "message": "SQLite connection open/close measured",
//                 "duration_ms": duration.as_millis(),
//             });
//             let response_string = response_body.to_string();
//             let response = Response::from_string(response_string)
//                 .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

//             // Respond to the request
//             if let Err(e) = request.respond(response) {
//                 eprintln!("Failed to send response: {}", e);
//             }
//         });
//     }

//     Ok(())
// }
