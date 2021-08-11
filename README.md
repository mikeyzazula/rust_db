# rust_db
Running instructions:

Install Postgres via terminal:
$sudo apt update
$sudo apt install postgresql postgresql-contrib
Create postgres user:
$sudo -u postgres createuser --interactive
"Enter name of role to add: postgres
Shall the new role be a superuser? (y/n)" 
$y
$sudo adduser postgres
$sudo -i -u postgres
$psql
ALTER USER postgres WITH PASSWORD 'myPassword';
In a separate terminal run :
$cargo build
