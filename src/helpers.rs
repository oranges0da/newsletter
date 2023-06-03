use crate::{
    config::{self, DBSettings},
    server,
};
use sqlx::{query, Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// spawn instance of app for testing
pub async fn spawn_app() -> TestApp {
    // setup config and connect to postgres
    let mut config = config::get_config().expect("Failed to read config from yaml.");
    config.db_settings.database_name = Uuid::new_v4().to_string(); // set new test db name to random id

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind address to TcpListener.");

    // create new database for testing and connect to it
    let db_pool = spawn_db_pool(config.db_settings.clone()).await;

    let server = server::run(listener, db_pool.clone()).expect("Failed to bind address");

    // start the server as a background task using tokio::spawn
    // tokio::spawn will immediately return without waiting for task to complete
    let _ = tokio::spawn(server);

    // return instance of TestApp to be used by test cases
    TestApp { address, db_pool }
}

pub async fn spawn_db_pool(config: DBSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.get_database_url())
        .await
        .expect("Failed to connect to postgres");

    connection
        .execute(format!("CREATE DATABASE {}", config.database_name).as_str())
        .await
        .expect("Failed to create test database");

    // create new pool for connections and migrate database
    let db_pool = PgPool::connect(&config.get_database_url())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");

    db_pool
}
