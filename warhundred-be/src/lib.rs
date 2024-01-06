pub mod schema;
pub mod domain;

pub mod backend {
    use diesel::{Connection, PgConnection};
    use dotenvy::dotenv;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}