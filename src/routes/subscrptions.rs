use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData{
    name : String,
    email : String
}

pub async fn subscribe(form_data: web::Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}