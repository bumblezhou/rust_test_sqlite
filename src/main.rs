use rusqlite::{params, Connection, Result};
use std::fs;

// Define a struct representing your data
#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    let db_file_path = "person.db";

    // Open a connection to the SQLite database
    let conn = Connection::open(&db_file_path)?;

    // Create a table to store Person data if it doesn't exist
    conn.execute("CREATE TABLE IF NOT EXISTS person (
        id              INTEGER PRIMARY KEY,
        name            TEXT NOT NULL,
        age             INTEGER NOT NULL)", [])?;
    
    // Insert some data into the table
    conn.execute("INSERT INTO person (name, age) VALUES (?1, ?2)", params!["Alice", 30])?;
    conn.execute("INSERT INTO person (name, age) VALUES (?1, ?2)", params!["Bob", 25])?;

    // Retrieve data from the table
    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    // Iterate over the retrieved data and print it
    for person in person_iter {
        println!("{:?}", person.unwrap());
    }

    // Check if the file exists
    if fs::metadata(&db_file_path).is_ok() {
        if let Err(err) = fs::remove_file(&db_file_path) {
            eprintln!("Error removing file: {}", err);
        } else {
            println!("File '{}' removed successfully.", &db_file_path);
        }
    } else {
        println!("File '{}' does not exist.", db_file_path);
    }

    Ok(())
    
}
