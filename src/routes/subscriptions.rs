use {
    crate::*,
    uuid::Uuid,
    chrono::Utc,
    sqlx::types::Uuid as Sqlx_Uuid,
};

#[derive(serde::Deserialize)]
pub struct FormData{
    pub name: String,
    pub email: String,
}

pub async fn subscribe(form: web::Form<FormData>, 
db_connection: web::Data<PgPool>) -> HttpResponse{
    let request_id = Uuid::new_v4();
    log::info!("request_id {} - Adding '{}' '{}' as a new subscriber", 
               request_id, form.name, form.email);
    log::info!("request_id {} - Saving new subscriber details in the database",
               request_id);

    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Sqlx_Uuid::from_bytes(*Uuid::new_v4().as_bytes()), // fix for this?
        form.email,
        form.name,
        Utc::now(),
        )
        .execute(db_connection.get_ref())
        .await{
            Ok(_) => {
                log::info!("request_id {} - New subscriber details have been saved",
                           request_id);
                HttpResponse::Ok().finish()
            },
            Err(e) => {
                log::error!("request_id {} - Failed to execute query: {:?}", 
                            request_id, e);
                HttpResponse::InternalServerError().finish()
            },
        }
}
