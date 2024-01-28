use rusqlite::{params, Connection, Result};
use bcrypt::{hash, DEFAULT_COST};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn create_users_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                username TEXT PRIMARY KEY,
                password TEXT NOT NULL
            )",
            params![],
        )?;
        Ok(())
    }

    pub fn add_user(&self, username: &str, password: &str) -> Result<()> {
        let hashed_password = hash(password, DEFAULT_COST)?;
        self.conn.execute(
            "INSERT INTO users (username, password) VALUES (?1, ?2)",
            params![username, hashed_password],
        )?;
        Ok(())
    }

    pub fn check_user_exists(&self, username: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare("SELECT username FROM users WHERE username = ?1")?;
        let user_exists = stmt.exists(params![username])?;
        Ok(user_exists)
    }
}