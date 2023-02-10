use tokio_postgres::{Client, Error, NoTls};
use sqlx::{postgres::PgPoolOptions, Row};

pub async fn connect_to_db() -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("postgres://postgres:postgrespw@localhost:32768", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    // Create the table in the postgres Database.
    Ok(client)
}

pub async fn create_tables() -> Result<String, sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgrespw@localhost:32768")
        .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS users {
                id bigserial,
                name text
            }
        "#,
    ).execute(&pool).await?;

    // let row: (i64,) = sqlx::query_as("insert into users (name) values ($1) returning id")
    //     .bind("a new user")
    //     .fetch_one(&pool)
    //     .await?;

    let rows = sqlx::query("SELECT * FROM users").fetch_all(&pool).await?;
    let str_result: String = rows
        .iter()
        .map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
        .collect::<Vec<String>>().join(", ");
    Ok(str_result)
}
