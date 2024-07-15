use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn create_db_pool(url: &str) -> DatabasePool {
    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Pool creation failure");
    pool
}

pub fn run_migration(conn: &mut impl MigrationHarness<diesel::pg::Pg>) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
