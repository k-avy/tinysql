use rusqlite::{Connection,params, Result};
use serde_json::json;
use std::time::Instant;
use tide::{Request, Response};

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

pub async fn tide_fn() {
    let mut app = tide::new();
    app.at("/").get(handle_request);

    println!("Listening on http://0.0.0.0:8000/");
    
    app.listen("0.0.0.0:8000").await.unwrap();
}

async fn handle_request(_: Request<()>) -> tide::Result {
    let start = Instant::now();

    // Open a connection to SQLite
    let sqlite_status = match Connection::open("my_database.db") {
        Ok(conn) => {
            match create_table(&conn) {
                Ok(_) => {
                    match select_person(&conn, "Alice") {
                        Ok(_) => "Person updated successfully",
                        Err(e) => {
                            eprintln!("Failed to insert person: {}", e);
                            "Failed to insert person"
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to create table: {}", e);
                    "Failed to create table"
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open SQLite connection: {}", e);
            "Failed to open SQLite connection"
        }
    };
    let _me = Person {
                name: "Kavya".to_string(),
                age: 22,
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

// fn insert_person(conn: &Connection, name: &str, age: i32) -> Result<usize> {
//     conn.execute(
//         "INSERT INTO person (name, age) VALUES (?1, ?2)",
//         params![name, age],
//     )
// }
// fn update_person_age(conn: &Connection, name: &str, new_age: i32) -> Result<usize> {
//         conn.execute(
//             "UPDATE person SET age = ?1 WHERE name = ?2",
//             params![new_age, name],
//         )
//     }
// fn delete_person(conn: &Connection, name: &str) -> Result<usize> {
//     conn.execute(
//         "DELETE FROM person WHERE name = ?1",
//         params![name],
//     )
// }
fn select_person(conn: &Connection, name: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, age FROM person WHERE name = ?1")?;
    let person_iter = stmt.query_map(params![name], |row| {
        Ok(Person {
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}