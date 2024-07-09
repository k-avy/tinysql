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

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: Option<T>,
    error: Option<String>,
    time_taken: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePersonRequest {
    age: i32,
}


pub async fn tide_crud() -> tide::Result<()> {
    // Open a connection to SQLite and create the table once
    let conn = Connection::open("my_database.db").expect("Failed to open SQLite connection");
    create_table(&conn).expect("Failed to create table");

    let mut app = tide::new();
    app.at("/").post(handle_post_request);
    app.at("/:name").get(handle_get_request);
    app.at("/:name").put(handle_put_request);
    app.at("/:name").delete(handle_delete_request);

    println!("Listening on http://0.0.0.0:8000/");
    
    app.listen("0.0.0.0:8000").await?;
    Ok(())
}

async fn handle_post_request(mut req: Request<()>) -> tide::Result {
    let start = Instant::now();

    // Parse the JSON body of the request
    let person: Person = match req.body_json().await {
        Ok(person) => person,
        Err(_) => {
            let response = ApiResponse::<()>::error("Invalid JSON data", start.elapsed());
            return Ok(response.into());
        }
    };

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_database.db") {
        Ok(conn) => {
            match insert_person(&conn, &person.name, person.age) {
                Ok(_) => "Person inserted successfully".to_string(),
                Err(e) => {
                    eprintln!("Failed to insert person: {}", e);
                    "Failed to insert person".to_string()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            "Failed to open SQLite connection".to_string()
        }
    };

    let duration = start.elapsed();
    println!("Time taken to handle request: {:?}", duration);

    // Create a JSON response
    let response = ApiResponse {
        status: sqlite_status,
        data: Some(person),
        error: None,
        time_taken: format!("{:?}", duration),
    };
    
    Ok(response.into())
}

async fn handle_get_request(req: Request<()>) -> tide::Result {
    let start = Instant::now();
    let name = req.param("name")?;

    // Open a connection to SQLite
    let (sqlite_status, person) = match Connection::open("my_database.db") {
        Ok(conn) => {
            match select_person(&conn, name) {
                Ok(person) => ("Person retrieved successfully".to_string(), Some(person)),
                Err(e) => {
                    eprintln!("Failed to retrieve person: {}", e);
                    ("Failed to retrieve person".to_string(), None)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            ("Failed to open SQLite connection".to_string(), None)
        }
    };

    let duration = start.elapsed();
    println!("Time taken to handle request: {:?}", duration);
    
    // Create a JSON response
    let response = ApiResponse {
        status: sqlite_status,
        data: person,
        error: None,
        time_taken: format!("{:?}", duration),
    };
    
    Ok(response.into())
}

async fn handle_put_request(mut req: Request<()>) -> tide::Result {
    let start = Instant::now();
    let name = req.param("name")?.to_string(); // Convert to String to own the data

    let update_request: UpdatePersonRequest = match req.body_json().await {
        Ok(data) => data,
        Err(_) => {
            let response = ApiResponse::<()>::error("Invalid JSON data", start.elapsed());
            return Ok(response.into());
        }
    };

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_database.db") {
        Ok(conn) => {
            match update_person_age(&conn, &name, update_request.age) {
                Ok(_) => "Person updated successfully".to_string(),
                Err(e) => {
                    eprintln!("Failed to update person: {}", e);
                    "Failed to update person".to_string()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            "Failed to open SQLite connection".to_string()
        }
    };

    let duration = start.elapsed();
    println!("Time taken to handle request: {:?}", duration);

    // Create a JSON response
    let response = ApiResponse {
        status: sqlite_status,
        data: Some(update_request),
        error: None,
        time_taken: format!("{:?}", duration),
    };

    Ok(response.into())
}

async fn handle_delete_request(req: Request<()>) -> tide::Result {
    let start = Instant::now();
    let name = req.param("name")?;

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_database.db") {
        Ok(conn) => {
            match delete_person(&conn, name) {
                Ok(_) => "Person deleted successfully".to_string(),
                Err(e) => {
                    eprintln!("Failed to delete person: {}", e);
                    "Failed to delete person".to_string()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            "Failed to open SQLite connection".to_string()
        }
    };

    let duration = start.elapsed();
    println!("Time taken to handle request: {:?}", duration);
    
    // Create a JSON response
    let response = ApiResponse::<()> {
        status: sqlite_status,
        data: None,
        error: None,
        time_taken: format!("{:?}", duration),
    };

    Ok(response.into())
}

impl<T> ApiResponse<T> {
    fn error(message: &str, duration: std::time::Duration) -> Self {
        Self {
            status: "Error".to_string(),
            data: None,
            error: Some(message.to_string()),
            time_taken: format!("{:?}", duration),
        }
    }
}

impl<T: Serialize> Into<Response> for ApiResponse<T> {
    fn into(self) -> Response {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(serde_json::to_string(&self).unwrap());
        response.set_content_type("application/json");
        response
    }
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

fn select_person(conn: &Connection, name: &str) -> Result<Person> {
    let mut stmt = conn.prepare("SELECT name, age FROM person WHERE name = ?1")?;
    let person_iter = stmt.query_map(params![name], |row| {
        Ok(Person {
            name: row.get(0)?,
            age: row.get(1)?,
        })
    })?;

    for person in person_iter {
        return Ok(person?);
    }
    Err(rusqlite::Error::QueryReturnedNoRows)
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
