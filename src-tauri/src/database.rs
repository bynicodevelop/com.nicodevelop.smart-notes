use rusqlite::{Connection, Result};
use sqlite_vec::sqlite3_vec_init;
use std::fs;
use std::path::PathBuf;

fn get_database_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let app_dir = home_dir.join(".smart-notes");

    fs::create_dir_all(&app_dir).expect("Could not create app directory");

    app_dir.join("notes.db")
}

pub fn init_database() -> Result<Connection> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path)?;

    unsafe {
        conn.load_extension_enable()?;
        conn.load_extension(sqlite3_vec_init as *const (), None)?;
        conn.load_extension_disable()?;
    }

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            file_path TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS note_embeddings USING vec0 (
            note_id INTEGER PRIMARY KEY,
            embedding FLOAT[384]
        );

        CREATE TRIGGER IF NOT EXISTS notes_updated_at
        AFTER UPDATE ON notes
        FOR EACH ROW
        BEGIN
            UPDATE notes SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
        END;
        "
    )?;

    log::info!("Database initialized at {:?}", db_path);

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_database() {
        let conn = init_database();
        assert!(conn.is_ok());
    }
}
