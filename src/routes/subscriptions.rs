use actix_web::HttpResponse;
use actix_web::web::Form;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
