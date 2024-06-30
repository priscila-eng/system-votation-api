use postgres::Error as PostgresError;
use postgres::{ Client, NoTls };
use crate::constants::constants::DB_URL;

//set_database function
pub fn set_database() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //Create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE,
            password VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL,
            token VARCHAR NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )"
    )?;

    Ok(())
}