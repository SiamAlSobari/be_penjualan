use actix_web::{HttpResponse, Responder};

pub async fn register() ->impl Responder {
    HttpResponse::Ok().json("sa")
}

pub async fn login () -> impl Responder {
    HttpResponse::Ok().json("a")
}