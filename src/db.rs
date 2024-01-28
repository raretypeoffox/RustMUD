use rusqlite::{params, Connection, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

const DB_PATH: &str = "users.db";

pub struct UserDatabase {
    conn: Connection,
}

impl UserDatabase {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(DB_PATH)?;
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

    pub fn add_user(&self, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let username = username.to_lowercase();
        let hashed_password = hash(password, DEFAULT_COST).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
        self.conn.execute(
            "INSERT INTO users (username, password) VALUES (?1, ?2)",
            params![username, hashed_password],
        )?;
        println!("Added user {} to the database", username);
        Ok(())
    }
    
    pub fn check_user_exists(&self, username: &str) -> Result<bool> {
        let username = username.to_lowercase();
        let mut stmt = self.conn.prepare("SELECT username FROM users WHERE username = ?1")?;
        let user_exists = stmt.exists(params![username])?;
        Ok(user_exists)
    }

    pub fn check_password(&self, username: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let username = username.to_lowercase();
        let mut stmt = self.conn.prepare("SELECT password FROM users WHERE username = ?1")?;
        let mut rows = stmt.query(params![username])?;

        if let Some(row) = rows.next()? {
            let hashed_password: String = row.get(0)?;
            return Ok(verify(password, &hashed_password)?);
        }

        Ok(false)
    }
}