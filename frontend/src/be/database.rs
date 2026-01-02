use dioxus::prelude::*;

/////////////////////
// the server side //
/////////////////////

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let db_file = "cognitive.db";

        // Open the database from the persisted file.
        let conn = rusqlite::Connection::open(db_file).expect(format!("Failed to open '{}' database", db_file).as_str());

        // Migrate the database (apply changes).
        if let Err(e) = conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS general_store (
                name        TEXT     PRIMARY KEY,
                value_txt   TEXT     NOT NULL DEFAULT '',
                value_int   INTEGER  NOT NULL DEFAULT 0,
                value_fload REAL     NOT NULL DEFAULT 0.0,
                value_bool  BOOL     NOT NULL DEFAULT false
            );",
        ) {
            println!("Error: Failed to create user_accounts table: {}", e);
        }

        conn
    };
}

#[server(endpoint = "load_ui_state")]
#[get("/api/state")]
pub async fn load_ui_state() -> Result<crate::ui::UiState, ServerFnError> {
    let ui_state_str: String = DB.with(|f| {
        f.prepare("SELECT value_txt FROM general_store WHERE name = 'ui_state';")
            .unwrap()
            .query_row([], |r| r.get::<_, String>(0))
            .map_err(|e| {
                if e == rusqlite::Error::QueryReturnedNoRows {
                    debug!("There is no ui state in db.");
                    return Ok(String::new());
                }
                debug!("Failed to load ui state from db: message='{}' code={:?}", e, e.sqlite_error_code());
                Err(e)
            })
            .unwrap_or_default()
    });

    Ok(serde_json::from_str(&ui_state_str).unwrap_or_default())
}

#[server(endpoint = "save_ui_state")]
#[post("/api/state")]
pub async fn save_ui_state(state: crate::ui::UiState) -> Result<(), ServerFnError> {
    let ui_state_str = serde_json::to_string(&state).unwrap();
    DB.with(|f| {
        f.execute(
            "INSERT INTO general_store(name, value_txt) VALUES('ui_state', ?1)
             ON CONFLICT(name) DO UPDATE SET value_txt = ?1 WHERE name = 'ui_state';",
            &[&ui_state_str],
        )
        .map_err(|e| debug!("Failed to update ui state into db: {}", e))
        .unwrap();
    });
    debug!("Updated ui state in db as '{}'.", ui_state_str);
    Ok(())
}

// FYI: Just an example of using sqlite.
#[cfg(feature = "server")]
pub fn print_db() {
    DB.with(|f| {
        let dbs = f
            .query_row("PRAGMA database_list", [], |r| r.get::<_, String>(2))
            .map_err(|e| println!("Failed to get database list: {}", e));
        println!("Using sqlite file: {:?}.", dbs.unwrap_or_default());
    });
}
