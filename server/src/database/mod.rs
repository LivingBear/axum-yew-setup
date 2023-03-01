use crate::error::AuthError;
use super::Db;
use async_trait::async_trait;

pub mod mongo;
pub mod postgres;


/// A type that abstracts a database.
#[async_trait]
pub trait Db {
    /// A connection to the database.
    type Conn;

    /// Creates a new database pool connection.
    async fn conn(&self) -> Result<Self::Conn, AuthError>;
}