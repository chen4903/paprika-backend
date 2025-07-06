use dotenv::dotenv;
use eyre::Result;
use rusqlite::{params, Connection};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::{DEFAULT_DATABASE_PATH, EXPIRED_TIME};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(is_in_memory: bool) -> Result<Self> {
        dotenv().ok();
        let db_path = env::var("DB_PATH").unwrap_or_else(|_| DEFAULT_DATABASE_PATH.to_string());

        let conn = if is_in_memory {
            Connection::open_in_memory()?
        } else {
            Connection::open(db_path)?
        };

        conn.execute(
            "CREATE TABLE IF NOT EXISTS runtime_code (
                chain_id INTEGER NOT NULL,
                address TEXT NOT NULL,
                runtime_code TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                PRIMARY KEY (chain_id, address)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS selector (
                selector TEXT PRIMARY KEY NOT NULL,
                signature TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn get_current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    /// This is designed to prevent the database from becoming too large, making the program small and exquisite
    pub fn cleanup_expired_records(&self) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let expired_time = Self::get_current_timestamp() - EXPIRED_TIME;

        let tx = conn.transaction()?;

        tx.execute(
            "DELETE FROM runtime_code WHERE created_at < ?",
            params![expired_time],
        )?;

        tx.execute(
            "DELETE FROM selector WHERE created_at < ?",
            params![expired_time],
        )?;

        tx.commit()?;

        Ok(())
    }

    pub fn get_runtime_code(&self, chain_id: u64, address: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT runtime_code, created_at FROM runtime_code WHERE chain_id = ? AND address = ?",
        )?;

        let result = stmt.query_row(params![chain_id as i64, address], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        });

        match result {
            Ok((runtime_code, _)) => Ok(Some(runtime_code)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save_runtime_code(
        &self,
        chain_id: u64,
        address: &str,
        runtime_code: &str,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO runtime_code (chain_id, address, runtime_code, created_at) 
             VALUES (?, ?, ?, ?)",
            params![
                chain_id as i64,
                address,
                runtime_code,
                Self::get_current_timestamp()
            ],
        )?;
        Ok(())
    }

    pub fn get_signature(&self, selector: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT signature, created_at FROM selector WHERE selector = ?")?;

        let result = stmt.query_row(params![selector], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        });

        match result {
            Ok((signature, _)) => Ok(Some(signature)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save_signature(&self, selector: &str, signature: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO selector (selector, signature, created_at) 
             VALUES (?, ?, ?)",
            params![selector, signature, Self::get_current_timestamp()],
        )?;
        Ok(())
    }
}
