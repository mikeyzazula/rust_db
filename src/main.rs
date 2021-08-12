// Postgres crate to utilize basic db operations we'll need.
use postgres::{Client, Error, NoTls};

// fs is the module for basic filesystem manipulation, we just need File for opening.
use std::fs::File;

// From io we need BufRead/er for it's additional line reading capabilities and functionality.
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    // Create a database called postgres if it does not exit, and then connect to it.
    let mut client = Client::connect(
        "postgresql://postgres:myPassword@localhost:5432/postgres",
        NoTls,
    )?;

    // Creating table for the DB.
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS rangers (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL
        )
    ",
    )?;

    // Handling the file io and setting the reader.
    let file = File::open("src/text_file.txt").expect("File open error");
    let reader = BufReader::new(file);

    // BufRead lets us insert the text into the DB line by line.
    for line in reader.lines() {
        client.execute("INSERT INTO rangers (name) VALUES ($1)", &[&line.ok()])?;
    }

    // Querying the DB and printing each line to check contents.
    for row in client.query("SELECT name FROM rangers", &[])? {
        let name: &str = row.get(0);
        println!("Found: {}", name);
    }

    // Dropping the table after were done executing. Comment out if needed.
    client.batch_execute(
        "
    DROP TABLE rangers;
    ",
    )?;

    Ok(())
}
