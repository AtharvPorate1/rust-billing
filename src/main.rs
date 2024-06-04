use std::env::args;
use rusqlite::{params, Connection};

fn main() {
    // Open a connection to the SQLite database
    let conn = Connection::open("file.db").expect("Connection failed");

    // Create the billing table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS billing (
            id INTEGER PRIMARY KEY,
            category TEXT NOT NULL,
            date TEXT NOT NULL,
            spent REAL NOT NULL,
            budget REAL NOT NULL
        )",
        (),  // This is the correct way to pass an empty parameter list
    ).expect("Failed to create a database");

    // Collect command-line arguments
    let args: Vec<String> = args().collect();

    // Check if an argument is provided to avoid out-of-bounds panic
    if args.len() > 1 {
        println!("{}", &args[1]);
        let command  = &args[1];

        match &command[..]{
            "show" => show_budget(),
            _ => println!("invalid command")

        }




    } else {
        eprintln!("No argument provided");
    }

    
}


fn show_budget(){
    println!("This will show you the budget");
}

