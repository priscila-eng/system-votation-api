use tokio_postgres::{Client, NoTls, Error as PostgresError};
use crate::constants::constants::DB_URL;

pub async fn set_database() -> Result<(), PostgresError> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(DB_URL, NoTls).await?;

    // Spawn a new task to run the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {:?}", e);
        }
    });

    // Create tables
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE,
            password VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).await?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL,
            token VARCHAR NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )"
    ).await?;

    Ok(())
}
