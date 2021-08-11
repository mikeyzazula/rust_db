//postgres crate to utilize basic db operations we'll need
use postgres::{Client, Error, NoTls};

//fs is the module for basic filesystem manipulation, we just need File for opening
use std::fs::File;

//From io we need BufReader for it's additional line reading capabilities
//TODO what the hell is prelude
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Error> {
    //Connecting to database
    let mut client = Client::connect(
        "postgresql://postgres:myPassword@localhost:5432/postgres",
        NoTls,
    )?;

    //creating table for the DB
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS rangers (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL
        )
    ",
    )?;

    //handling the io
    let file = File::open("src/text_file.txt").expect("File open error");
    let reader = BufReader::new(file);

    //inserting the text into the DB
    for line in reader.lines() {
        client.execute("INSERT INTO rangers (name) VALUES ($1)", &[&line.ok()])?;
    }

    //Querying the DB and printing to check
    for row in client.query("SELECT name FROM rangers", &[])? {
        let name: &str = row.get(0);
        println!("Found: {}", name);
    }
    //Clearing the DB
    client.batch_execute(
        "
    DROP TABLE rangers;
    ",
    )?;
    Ok(())
}
