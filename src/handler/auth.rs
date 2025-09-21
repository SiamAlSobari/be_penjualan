use crate::validation::auth::RegisterUser;
use actix_web::{HttpResponse, Responder, web};
use backend::{Response, ValidateErrItem, map_validation};
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

pub async fn register(user: web::Json<RegisterUser>, pool: web::Data<MySqlPool>) -> impl Responder {
    //match validate
    match user.validate() {
        Ok(_) => {
            let existing = sqlx::query!("SELECT id FROM users WHERE email = ? LIMIT 1", user.email)
                .fetch_optional(pool.get_ref())
                .await;
            match existing {
                Ok(Some(_)) => {
                    let res = Response::<()> {
                        status: "Failed".to_string(),
                        message: "Email sudah digunakan".to_string(),
                        data: None,
                    };
                    HttpResponse::Conflict().json(res)
                }
                Ok(None) => {
                    // gen user_id
                    let user_id = Uuid::new_v4().to_string();
                    //gen password hash
                    let hashed_password = bcrypt::hash(&user.password, 10).expect("hash err");
                    //insert user
                    let _ = sqlx::query!(
                        "INSERT INTO users ( id,email, hash_password) VALUES (?, ?, ?)",
                        user_id,
                        user.email,
                        hashed_password
                    )
                    .execute(pool.get_ref())
                    .await
                    .expect("Failed insert user");

                    // generate profile_id
                    let profile_id = Uuid::new_v4().to_string();

                    // insert profile
                    let _ = sqlx::query!(
                        "INSERT INTO profiles (id, user_name, user_id) VALUES (?, ?, ?)",
                        profile_id,
                        user.user_name,
                        user_id
                    )
                    .execute(pool.get_ref())
                    .await
                    .expect("Failed insert profile");
                    // response sukses
                    let res = Response::<()> {
                        status: "Success".to_string(),
                        message: "User dan profile berhasil dibuat ✅".to_string(),
                        data: None,
                    };

                    HttpResponse::Ok().json(res)
                }
                Err(err) => {
                    let res = Response::<String> {
                        status: "Failed".to_string(),
                        message: "Data base error".to_string(),
                        data: Some(format!("{}", err)),
                    };
                    HttpResponse::InternalServerError().json(res)
                }
            }
        }
        Err(err) => {
            let errs = map_validation(err);
            let res = Response::<Vec<ValidateErrItem>> {
                status: "Failed".to_string(),
                message: "Validation Error".to_string(),
                data: Some(errs),
            };
            HttpResponse::BadRequest().json(res)
        }
    }
}

pub async fn login() -> impl Responder {
    HttpResponse::Ok().json("a")
}
