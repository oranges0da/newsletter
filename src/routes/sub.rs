use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// if form data valid return 200 OK
pub async fn subscribe(_form: web::Form<FormData>, _db_pool: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
