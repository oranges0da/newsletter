use actix_web::{http::StatusCode, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let res = sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(db_pool.get_ref())
    .await;

    match res {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(_) => {
            return HttpResponse::build(StatusCode::BAD_REQUEST).body("Unable to insert into db.")
        }
    }
}
