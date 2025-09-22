use actix_web::HttpResponse;
use backend::{Response, ValidateErrItem, map_validation};
use uuid::Uuid;
use validator::Validate;

use crate::{repository::{profile_repository, user_repository}, validation::auth_validation};

pub struct AuthService<'a> {
    user_repo: user_repository::UserRepository<'a>,
    profile_repo: profile_repository::ProfileRepository<'a>,
}

impl<'a> AuthService<'a> {
    pub fn new(
        user_repo: user_repository::UserRepository<'a>,
        profile_repo: profile_repository::ProfileRepository<'a>,
    ) -> Self {
        Self {
            user_repo,
            profile_repo,
        }
    }

    pub async fn register_user(&self, user: auth_validation::RegisterUser) -> HttpResponse {
        if let Err(err) = user.validate() {
            let errs = map_validation(err);
            return HttpResponse::BadRequest().json(Response::<Vec<ValidateErrItem>> {
                status: "Failed".to_string(),
                message: "Validation Error".to_string(),
                data: Some(errs),
            });
        }
        if let Ok(Some(_user)) = self.user_repo.find_by_email(&user.email).await {
            return HttpResponse::Conflict().json(Response::<()> {
                status: "Failed".to_string(),
                message: "Email sudah digunakan".to_string(),
                data: None,
            });
        }

        if let Ok(Some(_profile)) = self.profile_repo.find_by_user_name(&user.user_name).await {
            return HttpResponse::Conflict().json(Response::<()> {
                status: "Failed".to_string(),
                message: "Username sudah digunakan".to_string(),
                data: None,
            });
        }
        let user_id = Uuid::new_v4().to_string();
        let profile_id = Uuid::new_v4().to_string();

        let hashed_password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();
        if let Err(err) = self
            .user_repo
            .insert_user(&user.email, &hashed_password, &user_id)
            .await
        {
            return HttpResponse::InternalServerError().json(Response::<String> {
                status: "Failed".to_string(),
                message: "Gagal insert user".to_string(),
                data: Some(err.to_string()),
            });
        }
        if let Err(err) = self.profile_repo.insert_profile(&profile_id,&user_id).await {
            return HttpResponse::InternalServerError().json(Response::<String> {
                status: "Failed".to_string(),
                message: "Gagal insert profile".to_string(),
                data: Some(err.to_string()),
            });
        }
        HttpResponse::Ok().json(Response::<()> {
            status: "Success".to_string(),
            message: "User dan profile berhasil dibuat ✅".to_string(),
            data: None,
        })
    }
}
