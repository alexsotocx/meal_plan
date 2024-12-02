use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(database_url: String, max_size: u32) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(max_size)
        .build(manager)
        .expect("Failed to create pool.")
}
