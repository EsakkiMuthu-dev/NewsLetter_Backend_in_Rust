use actix_web::{HttpResponse, web};
use chrono::Utc;
use log::log;
use sqlx::PgPool;
use tracing::{info_span, Instrument};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData{
    name : String,
    email : String
}

pub async fn subscribe(form_data: web::Form<FormData>, connection_pool : web::Data<PgPool>) -> HttpResponse{
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber",
        %request_id,
        subscriber_email = %form_data.email,
        subscriber_name = %form_data.name
    );
    let _request_span_guard = request_span.enter();
    
    tracing::info!("request_id -{} Adding '{}' '{}' as a new subscriber. " , request_id,form_data.email,form_data.name);
    tracing::info!("request_id - {} Saving new Subscriber details in database ",request_id);
    
    let query_span = tracing::info_span!("Saving new Subscriber details into database");
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
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            tracing::info!("request_id : {} New subscriber details saved!!",request_id);
            HttpResponse::Ok().finish()
        },
        Err(e)=>{
            tracing::error!("request_id {} - Failed to execute query : {:?} ",request_id,e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}