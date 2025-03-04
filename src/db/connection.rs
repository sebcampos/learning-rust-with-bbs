use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connect to SQLite (creates 'bbs.db' if not exists)
    let conn = Connection::open("bbs.db")?;

    // Create users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        );",
        [],
    )?;

    // Insert a test user
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params!["admin", "1234"],
    )?;

    println!("Database setup complete! ✅");
    Ok(())
}
