use rusqlite::{params, Connection, Result};
use tiny_http::{Server, Response, Request, Header, Method};
use std::io::Cursor;
use std::time::Instant;
use serde_json::{json, Value};
use serde::Deserialize;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Deserialize)]
struct PersonRequest {
    name: String,
    age: Option<i32>,
}

pub fn tinyhttp_crud() {
    // Create an HTTP server that listens on port 8000
    let server = Server::http("0.0.0.0:8000").unwrap();

    println!("Listening on http://0.0.0.0:8000/");

    // Open a connection to SQLite
    match Connection::open("db/my_database.db") {
        Ok(conn) => {
            // Create table if it doesn't exist
            if let Err(e) = create_table(&conn) {
                eprintln!("Failed to create table: {}", e);
                return;
            }

            // Handle incoming requests
            for request in server.incoming_requests() {
                handle_request(request, &conn);
            }
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
        }
    }
}

pub fn handle_request(mut request: Request, conn: &Connection) {
    let start = Instant::now();

    // Read the request body
    let mut body = String::new();
    if let Err(e) = request.as_reader().read_to_string(&mut body) {
        eprintln!("Failed to read request body: {}", e);
        respond_with_error(request, "Failed to read request body", 400);
        return;
    }

    // Parse the JSON body
    let json_data: Value = match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            respond_with_error(request, "Invalid JSON", 400);
            return;
        }
    };

    // Handle different HTTP methods
    let response = match request.method() {
        Method::Post => handle_post_request(conn, json_data),
        Method::Get => handle_get_request(conn, json_data),
        Method::Put => handle_put_request(conn, json_data),
        Method::Delete => handle_delete_request(conn, json_data),
        _ => respond_with_error_response("Unsupported HTTP method", 405),
    };

    if let Err(e) = request.respond(response) {
        eprintln!("Failed to respond to request: {}", e);
    }

    let duration = start.elapsed();
    println!("Time taken to handle request: {:?}", duration);
}

fn handle_post_request(conn: &Connection, json_data: Value) -> Response<Cursor<Vec<u8>>> {
    let person_request: PersonRequest = match serde_json::from_value(json_data) {
        Ok(data) => data,
        Err(_) => {
            return respond_with_error_response("Invalid JSON structure for POST", 400);
        }
    };

    match person_request.age {
        Some(age) => {
            if let Err(e) = insert_person(&conn, &person_request.name, age) {
                eprintln!("Failed to insert person: {}", e);
                return respond_with_error_response("Failed to insert person", 500);
            }
            respond_with_success_response("Person inserted successfully")
        }
        None => {
            respond_with_error_response("Age is required for inserting person", 400)
        }
    }
}

fn handle_get_request(conn: &Connection, json_data: Value) -> Response<Cursor<Vec<u8>>> {
    let person_request: PersonRequest = match serde_json::from_value(json_data) {
        Ok(data) => data,
        Err(_) => {
            return respond_with_error_response("Invalid JSON structure for GET", 400);
        }
    };

    if let Err(e) = select_person(&conn, &person_request.name) {
        eprintln!("Failed to select person: {}", e);
        respond_with_error_response("Failed to select person", 500)
    } else {
        respond_with_success_response("Person selected successfully")
    }
}

fn handle_put_request(conn: &Connection, json_data: Value) -> Response<Cursor<Vec<u8>>> {
    let person_request: PersonRequest = match serde_json::from_value(json_data) {
        Ok(data) => data,
        Err(_) => {
            return respond_with_error_response("Invalid JSON structure for PUT", 400);
        }
    };

    match person_request.age {
        Some(age) => {
            if let Err(e) = update_person_age(&conn, &person_request.name, age) {
                eprintln!("Failed to update person age: {}", e);
                return respond_with_error_response("Failed to update person age", 500);
            }
            respond_with_success_response("Person age updated successfully")
        }
        None => {
            respond_with_error_response("Age is required for updating person", 400)
        }
    }
}

fn handle_delete_request(conn: &Connection, json_data: Value) -> Response<Cursor<Vec<u8>>> {
    let person_request: PersonRequest = match serde_json::from_value(json_data) {
        Ok(data) => data,
        Err(_) => {
            return respond_with_error_response("Invalid JSON structure for DELETE", 400);
        }
    };

    if let Err(e) = delete_person(&conn, &person_request.name) {
        eprintln!("Failed to delete person: {}", e);
        respond_with_error_response("Failed to delete person", 500)
    } else {
        respond_with_success_response("Person deleted successfully")
    }
}

fn respond_with_error( request: Request, message: &str, status_code: u16) {
    let response = Response::from_string(message)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
        .with_status_code(status_code);
    if let Err(e) = request.respond(response) {
        eprintln!("Failed to respond to request: {}", e);
    }
}

fn respond_with_error_response(message: &str, status_code: u16) -> Response<Cursor<Vec<u8>>> {
    Response::from_string(message)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
        .with_status_code(status_code)
}

fn respond_with_success_response(message: &str) -> Response<Cursor<Vec<u8>>> {
    let response_data = json!({ "message": message });
    let response_body = response_data.to_string();
    Response::from_string(response_body)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
        .with_status_code(200)
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;
    Ok(())
}

fn select_person(conn: &Connection, name: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, age FROM person WHERE name = ?1")?;
    let person_iter = stmt.query_map(params![name], |row| {
        Ok(Person {
            name: row.get(0)?,
            age: row.get(1)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}

fn insert_person(conn: &Connection, name: &str, age: i32) -> Result<usize> {
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params![name, age],
    )
}

fn update_person_age(conn: &Connection, name: &str, new_age: i32) -> Result<usize> {
    conn.execute(
        "UPDATE person SET age = ?1 WHERE name = ?2",
        params![new_age, name],
    )
}

fn delete_person(conn: &Connection, name: &str) -> Result<usize> {
    conn.execute(
        "DELETE FROM person WHERE name = ?1",
        params![name],
    )
}
