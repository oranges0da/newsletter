use newsletter::config;
use sqlx::PgPool;

// make sure we can connect and query to postgres
#[tokio::test]
async fn query_postgres() {
    let config = config::get_config().expect("Failed to read config in conn_postgres");
    let conn_string = config.db_settings.get_connection_string();

    let conn_pool = PgPool::connect(&conn_string)
        .await
        .expect("Failed to connect to postgres.");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&conn_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "oranges0da@gmail.com");
    assert_eq!(saved.name, "oranges0da");
}
