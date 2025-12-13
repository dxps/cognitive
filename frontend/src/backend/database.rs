/////////////////////
// the server side //
/////////////////////

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let db_file = "cognitive.db";
        // Open the database from the persisted file.
        let conn = rusqlite::Connection::open(db_file).expect(format!("Failed to open {} database", db_file).as_str());

        conn
    };
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
