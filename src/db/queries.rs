// might need to make this public
pub(crate) const CREATE_USERS: &str = "CREATE TABLE IF NOT EXISTS users (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP
)";

pub(crate) const CREATE_ROOMS: &str = "CREATE TABLE IF NOT EXISTS rooms (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    online INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP
)";

pub(crate) const CREATE_ROOM_MESSAGES: &str = "CREATE TABLE IF NOT EXISTS room_messages (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (room_id) REFERENCES rooms(id),
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP
)";

pub(crate) const CREATE_DIRECT_MESSAGES: &str = "CREATE TABLE IF NOT EXISTS direct_messages (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    FOREIGN KEY (from_id) REFERENCES users(id),
    FOREIGN KEY (to_id) REFERENCES users(id),
    created_date DATETIME DEFAULT CURRENT_TIMESTAMP
)";


pub(crate) const GET_ROOMS: &str = "SELECT * FROM rooms ORDER BY online LIMIT 20";