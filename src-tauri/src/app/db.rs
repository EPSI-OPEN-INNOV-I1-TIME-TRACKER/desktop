use rusqlite::{Connection, OptionalExtension, params, Result as SqlResult};

pub fn connect_to_db() -> SqlResult<Connection> {
    let conn = Connection::open("window_tracker.db")?;
    // Initialize tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS windows (id INTEGER PRIMARY KEY AUTOINCREMENT, window_title TEXT NOT NULL, app_name TEXT NOT NULL)",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS time (window_id INTEGER NOT NULL, duration INTEGER NOT NULL, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (window_id) REFERENCES windows (id))",
        [],
    )?;
    Ok(conn)
}

pub fn update_or_insert_window(conn: &Connection, title: &str, app_name: &str) -> SqlResult<i64> {
    let mut stmt = conn.prepare("SELECT id FROM windows WHERE app_name = ?")?;
    if let Some(result) = stmt.query_row(params![app_name], |row| row.get(0)).optional()? {
        Ok(result)
    } else {
        conn.execute(
            "INSERT INTO windows (window_title, app_name) VALUES (?, ?)",
            params![title, app_name],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
