use diesel::sqlite::Sqlite;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations =
    diesel_migrations::embed_migrations!("src/app/migrations");

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn migration_connection(database_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(database_url).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}
