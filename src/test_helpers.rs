use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sql_query;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


pub fn connect_db(test_db: &str) -> Pool<ConnectionManager<PgConnection>> {
    let pool = crate::meal_plan::db::establish_connection_pool(
        "postgres://postgres:postgres@db".to_string(),
        1,
    );
    let conn = &mut pool.get().expect("Failed to get DB connection from pool");
    sql_query(format!("DROP DATABASE IF EXISTS {}", test_db).as_str())
        .execute(conn)
        .expect("Error deleting test database");

    sql_query(format!("CREATE DATABASE {}", test_db).as_str())
        .execute(conn)
        .expect("Error creating test database");

    let pool = crate::meal_plan::db::establish_connection_pool(
        format!("postgres://postgres:postgres@db/{}", test_db),
        1,
    );

    let conn = &mut pool.get().expect("Failed to get DB connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    return pool;
}
