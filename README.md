# Reading in text files and using a Postgres DB with Rust
Simple program that reads in a .txt file (provided in the src folder) then creates a Postgres database and places each line of text into the database.   
By default it prints out the contents of the DB, and upon completion drops the table.   
Ran on Ubuntu 20.04.2 LTS and OSX
## Running instructions:

Running `cargo run` will load the dependencies and run the program if Postgres is installed

### Install Postgres via terminal:
- `sudo apt update`
- `sudo apt install postgresql postgresql-contrib`

## Create Postgres user:
    Alternate way to run the program, should not be necessary
- `sudo -u postgres createuser --interactive`
- "Enter name of role to add: 
- `postgres`
Shall the new role be a superuser? (y/n)" 
- `y`
- `sudo adduser postgres`
- `sudo -i -u postgres`
- `psql`
- `ALTER USER postgres WITH PASSWORD 'myPassword';`