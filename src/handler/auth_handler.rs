use crate::{
    repository::{profile_repository::ProfileRepository, user_repository::UserRepository},
    service::auth_service::AuthService,
    validation::auth_validation::{LoginUser, RegisterUser},
};
use actix_web::{
    Responder,
    web::{self},
};
use sqlx::MySqlPool;

pub async fn register(user: web::Json<RegisterUser>, pool: web::Data<MySqlPool>) -> impl Responder {
    let user_repo: UserRepository<'_> = UserRepository::new(pool.get_ref());
    let profile_repo: ProfileRepository<'_> = ProfileRepository::new(pool.get_ref());
    let service: AuthService<'_> = AuthService::new(user_repo, profile_repo);
    service.register_user(user.into_inner()).await
}

pub async fn login(user: web::Json<LoginUser>, pool: web::Data<MySqlPool>) -> impl Responder {
    let user_repo: UserRepository<'_> = UserRepository::new(pool.get_ref());
    let profile_repo: ProfileRepository<'_> = ProfileRepository::new(pool.get_ref());
    let service: AuthService<'_> = AuthService::new(user_repo, profile_repo);
    service.login_user(user.into_inner()).await
}
