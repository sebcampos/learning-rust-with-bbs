// might need to make this public
const CREATE_IF_NOT_EXISTS: &str = "CREATE TABLE IF NOT EXISTS users (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP
)";

