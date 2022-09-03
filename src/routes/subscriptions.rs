use actix_web::{Responder, HttpResponse, web};



#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}


pub async fn subscribe(
    _form: web::Form<FormData>
) -> impl Responder {
    HttpResponse::Ok().finish()
}