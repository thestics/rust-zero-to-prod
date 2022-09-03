use actix_web::{Responder, HttpResponse, web};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}


pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>
) -> impl Responder {
    let q_result = sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now()
        )
        .execute(connection.get_ref())
        .await;
    
    match q_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}