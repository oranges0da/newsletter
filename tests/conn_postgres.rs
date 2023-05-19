use newsletter::config;
use sqlx::{Connection, PgConnection};

// test to make sure we can connect to postgres
#[tokio::test]
async fn connect_to_postgres() {
    let config = config::get_config().expect("Failed to read config in conn_postgres");
    let conn_string = config.db.get_connection_string();

    PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to postgres.");
}
