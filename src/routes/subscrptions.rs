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

#[tracing::instrument(
name="Adding a new Subscriber",
skip(form_data,connection_pool),
fields(
    request_id = %Uuid::new_v4(),
    subscriber_email=%form_data.email,
    subscriber_name=%form_data.name
)
)]
pub async fn subscribe(form_data: web::Form<FormData>, connection_pool : web::Data<PgPool>) -> HttpResponse{
    
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

#[tracing::instrument(
"Saving a new Subscriber details to database",
skip(form_data,connection_pool)
)]

pub async fn insert_subscriber(
    connection_pool : &PgPool,
    form_data: &FormData
) -> Result<(),sqlx::Error>{
    sqlx::query!(
        r#"
            INSERT Into subscriptions(id,email,name,subscribed_at)
            VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
        .execute(connection_pool)
        .await
        .map_err(|e|{
            tracing::error!(" Failed to excecute query: {:?}",e);
            e
        })?;
    Ok(())
}