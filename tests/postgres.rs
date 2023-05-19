use newsletter::config;
use sqlx::{Connection, PgConnection};

// test to make sure we can connect to postgres
#[tokio::test]
async fn connect_to_postgres() {
    let config = config::get_config().expect("Failed to read config in conn_postgres");
    let conn_string = config.db_settings.get_connection_string();

    PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to postgres.");
}

#[tokio::test]
async fn query_postgres() {
    let config = config::get_config().expect("Failed to read config in conn_postgres");
    let conn_string = config.db_settings.get_connection_string();

    let mut conn = PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to postgres.");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut conn)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "oranges0da@gmail.com");
    assert_eq!(saved.name, "oranges0da");
}