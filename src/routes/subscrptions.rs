use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData{
    name : String,
    email : String
}

pub async fn subscribe(form_data: web::Form<FormData>, connection_pool : web::Data<PgPool>) -> HttpResponse{
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id,email,name,subscribed_at)
        values($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
        .execute(connection_pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e)=>{
            println!("Failed to execute query : {} ",e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}