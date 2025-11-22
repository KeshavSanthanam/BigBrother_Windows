use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    // Users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            claude_api_key TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Tasks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL DEFAULT 1,
            title TEXT NOT NULL,
            description TEXT,
            due_date TEXT NOT NULL,
            min_duration INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            video_path TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        [],
    )?;

    // Recordings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recordings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            duration INTEGER NOT NULL DEFAULT 0,
            start_time TEXT NOT NULL,
            end_time TEXT,
            file_path TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'recording',
            FOREIGN KEY (task_id) REFERENCES tasks(id)
        )",
        [],
    )?;

    // Verifications table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_verifications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            verified BOOLEAN NOT NULL,
            ai_verification TEXT,
            ai_confidence INTEGER,
            time_on_task INTEGER,
            explanation TEXT,
            verified_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (task_id) REFERENCES tasks(id)
        )",
        [],
    )?;

    // Verifiers table (for future use)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS verifiers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            verifier_user_id INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (verifier_user_id) REFERENCES users(id)
        )",
        [],
    )?;

    // Create default user if not exists
    conn.execute(
        "INSERT OR IGNORE INTO users (id, email, username, password_hash)
         VALUES (1, 'default@bigbrother.app', 'default_user', 'temp')",
        [],
    )?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)",
        [],
    )?;

    Ok(())
}
