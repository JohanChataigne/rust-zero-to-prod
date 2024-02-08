use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct SubscriptionInput {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<SubscriptionInput>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
