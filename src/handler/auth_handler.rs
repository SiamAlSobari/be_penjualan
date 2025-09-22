use crate::{
    repository::{profile_repository::ProfileRepository, user_repository::UserRepository},
    service::auth_service::AuthService,
    validation::auth_validation::RegisterUser,
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use sqlx::MySqlPool;

pub async fn register(user: web::Json<RegisterUser>, pool: web::Data<MySqlPool>) -> impl Responder {
    let user_repo: UserRepository<'_> = UserRepository::new(pool.get_ref());
    let profile_repo: ProfileRepository<'_> = ProfileRepository::new(pool.get_ref());
    let service: AuthService<'_> = AuthService::new(user_repo, profile_repo);
    service.register_user(user.into_inner()).await
}

pub async fn login() -> impl Responder {
    //validate match
    //query db ke user apakah emailnya ada (usernya ada )
    //kalo ada verifikasi passwordnya
    //kalo bener masukan data user ke payload jwt lalu kirim ke cookie
    HttpResponse::Ok().json("a")
}
