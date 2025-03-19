// might need to make this public
pub(crate) const CREATE_USERS: &str = "CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP
)";

pub(crate) const CREATE_ROOMS: &str = "CREATE TABLE IF NOT EXISTS rooms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    online INTEGER NOT NULL DEFAULT 0,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    owner_id INTEGER,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
)";

pub(crate) const CREATE_ROOM_MESSAGES: &str = "CREATE TABLE IF NOT EXISTS room_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    user_id INTEGER,
    room_id INTEGER,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (room_id) REFERENCES rooms(id)
)";

pub(crate) const CREATE_DIRECT_MESSAGES: &str = "CREATE TABLE IF NOT EXISTS direct_messages (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    from_id INTEGER,
    to_id INTEGER,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (from_id) REFERENCES users(id),
    FOREIGN KEY (to_id) REFERENCES users(id)
)";


pub(crate) const GET_ROOMS: &str = "SELECT * FROM rooms ORDER BY online LIMIT 20";

pub(crate) const SEARCH_ROOMS: &str = "SELECT * FROM rooms where name LIKE ? LIMIT 20";

pub(crate) const SEARCH_USER: &str = "SELECT * FROM users WHERE username = ?";

pub(crate) const CREATE_NEW_ROOM: &str = "INSERT INTO rooms (name, owner_id) VALUES (?, ?)";

pub(crate) const CREATE_NEW_USER: &str = "INSERT INTO users (username, password_hash) VALUES (?, ?)";