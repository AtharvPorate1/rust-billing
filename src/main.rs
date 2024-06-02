use std::env::args;
use rusqlite::{params, Connection};

fn main() {

    let conn = Connection::open("file.db").expect("Connection failed");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS billing(
            id INTEGER PRIMARY KEY,
            data TEXT NOT NULL)"
        , ()).expect("Failed to create a database");


    
    let args: Vec<String> = args().collect();

    println!("{}", &args[1]);
}
