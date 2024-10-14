use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod models;
mod schema;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type Timestamp = chrono::NaiveDateTime;
pub type Timestamptz = chrono::DateTime<chrono::Utc>;

pub type PGConn = PgConnection;

pub mod wrapper {
    pub use diesel::result::*;
}

pub struct PGClient(pub DBPool);

impl PGClient {
    // pub fn new(database_url: String) -> Self {
    pub fn new() -> Self {
        let database_url = "postgres://postgres:6e2115148f4ba7e80ca0ce786d17c64f@localhost:5432/helio".to_string();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool.");

        PGClient(pool)
    }
}
