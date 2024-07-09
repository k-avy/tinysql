use rusqlite::{Connection, params, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;
use tide::{Request, Response, StatusCode};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

pub async fn tide_crud() {
    // Open a connection to SQLite and create the table once
    let conn = Connection::open("my_database.db").expect("Failed to open SQLite connection");
    create_table(&conn).expect("Failed to create table");

    let mut app = tide::new();
    app.at("/").post(handle_request);

    println!("Listening on http://0.0.0.0:8000/");
    
    app.listen("0.0.0.0:8000").await.unwrap();
}

async fn handle_request(mut req: Request<()>) -> tide::Result {
    let start = Instant::now();

    // Parse the JSON body of the request
    let person: Person = match req.body_json().await {
        Ok(person) => person,
        Err(_) => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body("Invalid JSON data");
            return Ok(response);
        }
    };

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_database.db") {
        Ok(conn) => {
            match select_person(&conn, &person.name) {
                Ok(_) => "Person retrieved successfully",
                Err(e) => {
                    eprintln!("Failed to retrieve person: {}", e);
                    "Failed to retrieve person"
                }
            }
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
        "received_data": person,
        "sqlite_status": sqlite_status,
        "time_taken": format!("{:?}", duration),
    });
    
    let response_body = response_data.to_string();
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(response_body);
    response.set_content_type("application/json");
    
    Ok(response)
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
